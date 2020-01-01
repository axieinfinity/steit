using System;
using System.Collections.Generic;

using Steit.Encoding;

namespace Steit.State {
    public static class Replayer {
        public static void Replay<T>(ref T root, Reader reader) where T : IState {
            while (!reader.Eof()) {
                ReplayOnce(ref root, reader.Nested((int) reader.ReadUInt32()));
            }
        }

        public static void ReplayOnce<T>(ref T root, Reader reader) where T : IState {
            var logType = (LogType) reader.ReadUInt16();

            if (logType != LogType.Update && logType != LogType.Add && logType != LogType.Remove) {
                Console.Error.WriteLine("Invalid log variant tag {0}", logType);
                reader.Exhaust();
                return;
            }

            var path = new List<UInt16>();

            if (!reader.Eof() && reader.ReadKey().Tag == 0) {
                var pathLength = (int) reader.ReadUInt32();

                while (pathLength-- > 0) {
                    path.Add(reader.ReadUInt16());
                }

                if (!reader.Eof()) {
                    reader.ReadKey();
                }
            }

            if (path.Count <= 0 && logType == LogType.Update) { // Update the root state
                var type = typeof(T);

                if (!reader.Eof()) {
                    var method = type.GetMethod("Deserialize");
                    var arguments = new object[] { reader, /* path: */ null, /* shouldNotify: */ true };
                    root = (T) method.Invoke(null, arguments);
                } else {
                    var arguments = new object[] { /* path: */ null };
                    root = (T) Activator.CreateInstance(type, arguments);
                }

                return;
            }

            UInt16 tag = 0;

            if (logType == LogType.Update || logType == LogType.Remove) {
                tag = path[path.Count - 1];
                path.RemoveAt(path.Count - 1);
            }

            IState state = root.Nested(path);

            if (state == null) {
                reader.Exhaust();
                return;
            }

            switch (logType) {
                case LogType.Update: state.ReplayUpdate(tag, reader); break;
                case LogType.Add: if (state.IsAddSupported()) state.ReplayAdd(reader); break;
                case LogType.Remove: if (state.IsRemoveSupported()) state.ReplayRemove(tag); break;
            }

            reader.Exhaust();
        }

        public static void ReplayUpdate(this IState state, UInt16 tag, Reader reader) {
            var wireType = state.WireType(tag);

            if (wireType >= 0) {
                if ((WireType) wireType != WireType.Sized) reader.ReadUInt32();
                state.ReplaceAt(tag, (WireType) wireType, reader, shouldNotify: true);
            } else {
                reader.Exhaust();
            }
        }
    }
}
