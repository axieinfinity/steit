using System;
using System.Collections.Generic;

using Steit.Codec;
using Steit.Collections;

namespace Steit.State {
    public static class StateReplayer {
        public static void Replay<T>(ref T root, IReader reader) where T : IState {
            while (!reader.EndOfStream()) {
                var entry = LogEntry.Deserialize(reader.GetNested());
                Replay(ref root, entry);
            }
        }

        public static void Replay<T>(ref T root, LogEntry entry) where T : IState {
            var path = new List<UInt32>(GetPath(entry));
            var tag = 0U;

            if (entry.Tag == LogEntry.UpdateTag) {
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
                case LogEntry.UpdateTag: {
                        var wireType = container.GetWireType(tag);
                        if (wireType == null) { return; }
                        var reader = new ByteReader(entry.UpdateVariant!.Value);
                        container.ReplaceAt(tag, wireType.Value, reader, shouldNotify: true);
                        break;
                    }

                case LogEntry.ListPushTag: {
                        var itemReader = new ByteReader(entry.ListPushVariant!.Item);
                        container.ReplayListPush(itemReader);
                        break;
                    }

                case LogEntry.ListPopTag: {
                        container.ReplayListPop();
                        break;
                    }

                case LogEntry.MapInsertTag: {
                        var mapInsertEntry = entry.MapInsertVariant;
                        var keyReader = new ByteReader(mapInsertEntry!.Key);
                        var valueReader = new ByteReader(mapInsertEntry!.Value);
                        container.ReplayMapInsert(keyReader, valueReader);
                        break;
                    }

                case LogEntry.MapRemoveTag: {
                        var keyReader = new ByteReader(entry.MapInsertVariant!.Key);
                        container.ReplayMapRemove(keyReader);
                        break;
                    }

                default: break;
            }
        }

        private static Vector<UInt32> GetPath(LogEntry entry) {
            switch (entry.Tag) {
                case LogEntry.UpdateTag: return entry.UpdateVariant!.FlattenPath;
                case LogEntry.ListPushTag: return entry.ListPushVariant!.FlattenPath;
                case LogEntry.ListPopTag: return entry.ListPopVariant!.FlattenPath;
                case LogEntry.MapInsertTag: return entry.MapInsertVariant!.FlattenPath;
                case LogEntry.MapRemoveTag: return entry.MapRemoveVariant!.FlattenPath;
                default: throw new InvalidOperationException(String.Format("Unknown log entry tag {0}", entry.Tag));
            }
        }
    }
}
