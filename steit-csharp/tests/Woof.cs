using System;

using Steit.Builtins;
using Steit.Codec;
using Steit.Collections;
using Steit.State;
using Steit.State.Event;

namespace Just.To.Test {
    public sealed class Woof : IState {
        public Path Path { get; }
        public StateMap<Int32> Map { get; private set; }

        public Woof(Path path = null) {
            this.Path = path ?? Path.Root;
            this.Map = new StateMap<Int32>(this.Path.GetNested(0));
        }

        public static event EventHandler<FieldUpdateEventArgs<StateMap<Int32>, Woof>> OnMapUpdate;

        public static void ClearMapUpdateHandlers() {
            OnMapUpdate = null;
        }

        public static void ClearUpdateHandlers() {
            OnMapUpdate = null;
        }

        public static Woof Deserialize(IReader reader, Path path = null) {
            var woof = new Woof(path);
            woof.Replace(reader);
            return woof;
        }

        public WireType? GetWireType(UInt32 tag) {
            switch (tag) {
                case 0: return WireType.Sized;
                default: return null;
            }
        }

        public IState GetNested(UInt32 tag) {
            switch (tag) {
                case 0: return this.Map;
                default: return null;
            }
        }

        public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
            switch (tag) {
                case 0: this.Map = this.MaybeNotify(0, StateMap<Int32>.Deserialize(reader, this.Path.GetNested(0)), this.Map, OnMapUpdate, shouldNotify); break;
                default: reader.SkipField(wireType); break;
            }
        }

        public void ReplayListPush(IReader reader) { throw new NotSupportedException(); }
        public void ReplayListPop() { throw new NotSupportedException(); }
        public void ReplayMapRemove(UInt32 key) { throw new NotSupportedException(); }

        private TValue MaybeNotify<TValue>(
            UInt32 tag,
            TValue newValue,
            TValue oldValue,
            EventHandler<FieldUpdateEventArgs<TValue, Woof>> handler,
            bool shouldNotify
        ) {
            if (shouldNotify) {
                var args = new FieldUpdateEventArgs<TValue, Woof>(tag, newValue, oldValue, this);
                handler?.Invoke(this, args);
            }

            return newValue;
        }
    }
}
