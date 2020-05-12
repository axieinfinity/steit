using System;
using System.Collections.Generic;

using Steit.Codec;
using Steit.Collections;

namespace Steit.State {
    public static class StateReplayer {
        public static void Replay<T>(ref T root, IReader reader) where T : IState {
            while (!reader.EndOfStream()) {
                var entry = LogEntryV2.Deserialize(reader.GetNested());
                Replay(ref root, entry);
            }
        }

        public static void Replay<T>(ref T root, LogEntryV2 entry) where T : IState {
            var path = new List<UInt32>(GetPath(entry));
            var tag = 0U;

            if (entry.Tag == LogEntryV2.UpdateTag) {
                if (path.Count > 0) {
                    tag = path[path.Count - 1];
                    path.RemoveAt(path.Count - 1);
                } else {
                    var reader = new ByteReader(entry.UpdateVariant!.Value);
                    root = StateFactory.Deserialize<T>(reader, root.Path);
                    return;
                }
            }

            var container = root.GetNested(path);

            if (container == null) {
                return;
            }

            switch (entry.Tag) {
                case LogEntryV2.UpdateTag: {
                        var wireType = container.GetWireType(tag);
                        if (wireType == null) { return; }
                        var reader = new ByteReader(entry.UpdateVariant!.Value);
                        container.ReplaceAt(tag, wireType.Value, reader, shouldNotify: true);
                        break;
                    }

                case LogEntryV2.ListPushTag: {
                        var reader = new ByteReader(entry.ListPushVariant!.Item);
                        container.ReplayListPush(reader);
                        break;
                    }

                case LogEntryV2.ListPopTag: {
                        container.ReplayListPop();
                        break;
                    }

                case LogEntryV2.MapRemoveTag: {
                        var key = entry.MapRemoveVariant!.Key;
                        container.ReplayMapRemove(key);
                        break;
                    }

                default: break;
            }
        }

        private static Vector<UInt32> GetPath(LogEntryV2 entry) {
            switch (entry.Tag) {
                case LogEntryV2.UpdateTag: return entry.UpdateVariant!.FlattenPath;
                case LogEntryV2.ListPushTag: return entry.ListPushVariant!.FlattenPath;
                case LogEntryV2.ListPopTag: return entry.ListPopVariant!.FlattenPath;
                case LogEntryV2.MapRemoveTag: return entry.MapRemoveVariant!.FlattenPath;
                default: throw new InvalidOperationException(String.Format("Unknown log entry tag {0}", entry.Tag));
            }
        }
    }
}
