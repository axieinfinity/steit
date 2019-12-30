using System;
using System.Collections.Generic;

using Steit.Reader;

namespace Steit {
    public abstract class State {
        public const UInt16 LOG_UPDATE = 0;
        public const UInt16 LOG_ADD = 1;
        public const UInt16 LOG_REMOVE = 2;

        public static void Replay<T>(ref T root, StateReader reader) where T : State {
            var variant = reader.ReadUInt16();

            if (variant != LOG_UPDATE && variant != LOG_ADD && variant != LOG_REMOVE) {
                Console.Error.WriteLine("Invalid log variant tag {0}", variant);
                reader.Exhaust();
                return;
            }

            reader.ReadKey();

            var path = new List<UInt16>();
            var pathLength = (int) reader.ReadUInt32();

            if (pathLength <= 0) { // Target is the root state
                throw new Exception("Not implemented yet");
            }

            while (pathLength-- > 0) {
                path.Add(reader.ReadUInt16());
            }

            UInt16 tag = 0;

            if (variant == LOG_UPDATE || variant == LOG_REMOVE) {
                tag = path[path.Count - 1];
                path.RemoveAt(path.Count - 1);
            }

            State state = root.NestedDeep(path);

            if (state == null) {
                reader.Exhaust();
                return;
            }

            reader.ReadKey();

            switch (variant) {
                case LOG_UPDATE: if (state.IsUpdateSupported()) state.ReplayUpdate(tag, reader); break;
                case LOG_ADD: if (state.IsAddSupported()) state.ReplayAdd(reader); break;
                case LOG_REMOVE: if (state.IsRemoveSupported()) state.ReplayRemove(tag); break;
            }

            reader.Exhaust();
        }

        public static void ReplayAll<T>(ref T root, StateReader reader) where T : State {
            while (!reader.Eof()) {
                Replay(ref root, reader.Nested((int) reader.ReadUInt32()));
            }
        }

        public abstract State Nested(UInt16 tag);

        public State NestedDeep(IList<UInt16> path) {
            State state = this;

            foreach (var tag in path) {
                if (state != null) {
                    state = state.Nested(tag);
                }
            }

            return state;
        }

        public bool IsUpdateSupported() { return true; }
        public bool IsAddSupported() { return false; }
        public bool IsRemoveSupported() { return false; }

        public void ReplayUpdate(UInt16 tag, StateReader reader) {
            var wireType = this.WireType(tag);

            if (wireType >= 0) {
                if (wireType != StateReader.WIRE_TYPE_SIZED) reader.ReadUInt32();
                this.ReplaceAt(tag, (Byte) wireType, reader, shouldNotify: true);
            } else {
                reader.Exhaust();
            }
        }

        public void ReplayAdd(StateReader reader) {
            throw new Exception("Unsupported");
        }

        public void ReplayRemove(UInt16 tag) {
            throw new Exception("Unsupported");
        }

        protected abstract Int16 WireType(UInt16 tag);
        protected abstract void ReplaceAt(UInt16 tag, Byte wireType, StateReader reader, bool shouldNotify = true);

        protected void Replace(StateReader reader, bool shouldNotify) {
            var (tag, wireType) = reader.ReadKey();
            var expectedWireType = this.WireType(tag);

            if (expectedWireType >= 0 && wireType != expectedWireType) {
                Console.Error.WriteLine("Unexpected tag {0} or wire type {1}", tag, wireType);
                reader.SkipWireTyped(wireType);
                return;
            }

            this.ReplaceAt(tag, wireType, reader, shouldNotify);
        }

        protected void ReplaceAll(StateReader reader, bool shouldNotify) {
            while (!reader.Eof()) {
                this.Replace(reader, shouldNotify);
            }
        }
    }
}
