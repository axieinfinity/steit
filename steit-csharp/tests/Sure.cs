using System;

using Steit.Builtins;
using Steit.Codec;
using Steit.Collections;
using Steit.State;
using Steit.State.Event;

namespace Just.To.Test {
    public sealed partial class Sure<T> : IState {
        public Path Path { get; }
        public T F0 { get; private set; }

        public Sure(Path path = null) {
            StateFactory.ValidateType(typeof(T));
            this.Path = path ?? Path.Root;
            this.F0 = StateFactory.Construct<T>(this.Path.GetNested(0));
        }

        public static event EventHandler<FieldUpdateEventArgs<T, Sure<T>>> OnF0Update;

        public static void ClearF0UpdateHandlers() {
            OnF0Update = null;
        }

        public static void ClearUpdateHandlers() {
            OnF0Update = null;
        }

        public static Sure<T> Deserialize(IReader reader, Path path = null) {
            var sure = new Sure<T>(path);
            sure.Replace(reader, shouldNotify: false);
            return sure;
        }

        public WireType? GetWireType(UInt32 tag) {
            switch (tag) {
                case 0: return StateFactory.IsStateType(typeof(T)) ? WireType.Sized : WireType.Varint;
                default: return null;
            }
        }

        public IState GetNested(UInt32 tag) {
            switch (tag) {
                case 0: return this.F0 as IState;
                default: return null;
            }
        }

        public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
            switch (tag) {
                case 0: this.F0 = this.MaybeNotify(0, StateFactory.Deserialize<T>(reader, this.Path, 0), this.F0, OnF0Update, shouldNotify); break;
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
            EventHandler<FieldUpdateEventArgs<TValue, Sure<T>>> handler,
            bool shouldNotify
        ) {
            if (shouldNotify) {
                var args = new FieldUpdateEventArgs<TValue, Sure<T>>(tag, newValue, oldValue, this);
                handler?.Invoke(this, args);
            }

            return newValue;
        }
    }
}
