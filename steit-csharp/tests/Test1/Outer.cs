using System;
using System.Collections.Generic;

using Steit;
using Steit.Reader;

namespace Steit.Test1 {
    public sealed class Outer : State {
        private static IList<Listener<Int32>> fooListeners = new List<Listener<Int32>>();
        private static IList<Listener<Boolean>> barListeners = new List<Listener<Boolean>>();
        private static IList<Listener<Inner>> innerListeners = new List<Listener<Inner>>();

        public Path Path { get; private set; }

        public Int32 Foo { get; private set; }
        public Boolean Bar { get; private set; }
        public Inner Inner { get; private set; }

        public Outer(Path path = null) {
            this.Path = path != null ? path : Path.Root;
            this.Inner = new Inner(this.Path.Nested(2));
        }

        public delegate void Listener<T>(T newValue, T oldValue, Outer container);

        public static int OnUpdateFoo(Listener<Int32> listener) { return Utils.Add(fooListeners, listener); }
        public static int OnUpdateBar(Listener<Boolean> listener) { return Utils.Add(barListeners, listener); }
        public static int OnUpdateInner(Listener<Inner> listener) { return Utils.Add(innerListeners, listener); }

        public static void RemoveFooListener(Listener<Int32> listener) { fooListeners.Remove(listener); }
        public static void RemoveBarListener(Listener<Boolean> listener) { barListeners.Remove(listener); }
        public static void RemoveInnerListener(Listener<Inner> listener) { innerListeners.Remove(listener); }

        public static void RemoveFooListenerAt(int index) { fooListeners.RemoveAt(index); }
        public static void RemoveBarListenerAt(int index) { barListeners.RemoveAt(index); }
        public static void RemoveInnerListenerAt(int index) { innerListeners.RemoveAt(index); }

        public static void ClearFooListeners() { fooListeners.Clear(); }
        public static void ClearBarListeners() { barListeners.Clear(); }
        public static void ClearInnerListeners() { innerListeners.Clear(); }

        public static void ClearAllListeners() {
            fooListeners.Clear();
            barListeners.Clear();
            innerListeners.Clear();
        }

        public static Outer Deserialize(StateReader reader, Path path = null) {
            var outer = new Outer(path);
            outer.ReplaceAll(reader.Nested((int) reader.ReadUInt32()), shouldNotify: false);
            return outer;
        }

        public override State Nested(UInt16 tag) {
            switch (tag) {
                case 2: return this.Inner;
                default: return null;
            }
        }

        protected override Int16 WireType(UInt16 tag) {
            switch (tag) {
                case 0: return 0;
                case 1: return 0;
                case 2: return 2;
                default: return -1;
            }
        }

        protected override void ReplaceAt(UInt16 tag, Byte wireType, StateReader reader, bool shouldNotify) {
            switch (tag) {
                case 0: this.Foo = this.Notify(reader.ReadInt32(), this.Foo, shouldNotify, fooListeners); break;
                case 1: this.Bar = this.Notify(reader.ReadBoolean(), this.Bar, shouldNotify, barListeners); break;
                case 2: this.Inner = this.Notify(Inner.Deserialize(reader, this.Path.Nested(2)), this.Inner, shouldNotify, innerListeners); break;
                default: reader.SkipWireTyped(wireType); break;
            }
        }

        private T Notify<T>(T newValue, T oldValue, bool shouldNotify, IList<Listener<T>> listeners) {
            if (shouldNotify) {
                foreach (var listener in listeners) {
                    listener(newValue, oldValue, this);
                }
            }

            return newValue;
        }
    }
}
