using System;

using Steit.Builtins;
using Steit.Codec;
using Steit.Collections;
using Steit.State;
using Steit.State.Event;

namespace Just.To.Test {
    public sealed class Outer : IState {
        public Path Path { get; }

        public Int32 Foo { get; private set; }
        public Boolean Bar { get; private set; }
        public Inner Inner { get; private set; }

        public Outer(Path? path = null) {
            this.Path = path ?? Path.Root;
            this.Inner = new Inner(this.Path.GetNested(2));
        }

        public static event EventHandler<FieldUpdateEventArgs<Int32, Outer>>? OnFooUpdate;
        public static event EventHandler<FieldUpdateEventArgs<Boolean, Outer>>? OnBarUpdate;
        public static event EventHandler<FieldUpdateEventArgs<Inner, Outer>>? OnInnerUpdate;

        public static void ClearFooUpdateHandlers() { OnFooUpdate = null; }
        public static void ClearBarUpdateHandlers() { OnBarUpdate = null; }
        public static void ClearInnerUpdateHandlers() { OnInnerUpdate = null; }

        public static void ClearUpdateHandlers() {
            OnFooUpdate = null;
            OnBarUpdate = null;
            OnInnerUpdate = null;
        }

        public static Outer Deserialize(IReader reader, Path? path = null) {
            var outer = new Outer(path);
            outer.Replace(reader);
            return outer;
        }

        public WireType? GetWireType(UInt32 tag) {
            switch (tag) {
                case 0: return WireType.Varint;
                case 1: return WireType.Varint;
                case 2: return WireType.Sized;
                default: return null;
            }
        }

        public IState? GetNested(UInt32 tag) {
            switch (tag) {
                case 2: return this.Inner;
                default: return null;
            }
        }

        public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
            switch (tag) {
                case 0: this.Foo = this.MaybeNotify(0, reader.ReadInt32(), this.Foo, OnFooUpdate, shouldNotify); break;
                case 1: this.Bar = this.MaybeNotify(1, reader.ReadBoolean(), this.Bar, OnBarUpdate, shouldNotify); break;
                case 2: this.Inner = this.MaybeNotify(2, Inner.Deserialize(reader, this.Path.GetNested(2)), this.Inner, OnInnerUpdate, shouldNotify); break;
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
            EventHandler<FieldUpdateEventArgs<TValue, Outer>>? handler,
            bool shouldNotify
        ) {
            if (shouldNotify) {
                var args = new FieldUpdateEventArgs<TValue, Outer>(tag, newValue, oldValue, this);
                handler?.Invoke(this, args);
            }

            return newValue;
        }
    }
}
