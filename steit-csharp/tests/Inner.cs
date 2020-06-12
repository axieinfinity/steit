using System;

using Steit.Builtins;
using Steit.Codec;
using Steit.Collections;
using Steit.State;
using Steit.State.Event;

namespace Just.To.Test {
    public sealed partial class Inner : IState {
        public Path Path { get; }

        public Int32 Foo { get; private set; }
        public Boolean Bar { get; private set; }

        public Inner(Path path = null) {
            this.Path = path ?? Path.Root;
        }

        public static event EventHandler<FieldUpdateEventArgs<Int32, Inner>> OnFooUpdate;
        public static event EventHandler<FieldUpdateEventArgs<Boolean, Inner>> OnBarUpdate;

        public static void ClearFooUpdateHandlers() { OnFooUpdate = null; }
        public static void ClearBarUpdateHandlers() { OnBarUpdate = null; }

        public static void ClearUpdateHandlers() {
            OnFooUpdate = null;
            OnBarUpdate = null;
        }

        public static Inner Deserialize(IReader reader, Path path = null) {
            var inner = new Inner(path);
            inner.Replace(reader, shouldNotify: false);
            return inner;
        }

        public WireType? GetWireType(UInt32 tag) {
            switch (tag) {
                case 0: return WireType.Varint;
                case 1: return WireType.Varint;
                default: return null;
            }
        }

        public IState GetNested(UInt32 tag) {
            switch (tag) {
                default: return null;
            }
        }

        public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
            switch (tag) {
                case 0: this.Foo = this.MaybeNotify(0, reader.ReadInt32(), this.Foo, OnFooUpdate, shouldNotify); break;
                case 1: this.Bar = this.MaybeNotify(1, reader.ReadBoolean(), this.Bar, OnBarUpdate, shouldNotify); break;
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
            EventHandler<FieldUpdateEventArgs<TValue, Inner>> handler,
            bool shouldNotify
        ) {
            if (shouldNotify) {
                var args = new FieldUpdateEventArgs<TValue, Inner>(tag, newValue, oldValue, this);
                handler?.Invoke(this, args);
            }

            return newValue;
        }
    }
}
