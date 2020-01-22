using System;
using System.Collections.Generic;

using Steit;
using Steit.Collections;
using Steit.Encoding;
using Steit.State;

namespace Steit.Test1 {
    public sealed class Inner : IState {
        private static IList<Listener<Int32>> fooListeners = new List<Listener<Int32>>();
        private static IList<Listener<Boolean>> barListeners = new List<Listener<Boolean>>();

        public Path Path { get; private set; }

        public Int32 Foo { get; private set; }
        public Boolean Bar { get; private set; }

        public Inner(Path path = null) {
            this.Path = path != null ? path : Path.Root;
        }

        public delegate void Listener<T>(T newValue, T oldValue, Inner container);

        public static int OnUpdateFoo(Listener<Int32> listener) { return Utilities.Add(fooListeners, listener); }
        public static int OnUpdateBar(Listener<Boolean> listener) { return Utilities.Add(barListeners, listener); }

        public static void RemoveFooListener(Listener<Int32> listener) { fooListeners.Remove(listener); }
        public static void RemoveBarListener(Listener<Boolean> listener) { barListeners.Remove(listener); }

        public static void RemoveFooListenerAt(int index) { fooListeners.RemoveAt(index); }
        public static void RemoveBarListenerAt(int index) { barListeners.RemoveAt(index); }

        public static void ClearFooListeners() { fooListeners.Clear(); }
        public static void ClearBarListeners() { barListeners.Clear(); }

        public static void ClearAllListeners() {
            fooListeners.Clear();
            barListeners.Clear();
        }

        public static Inner Deserialize(Reader reader, Path path = null, bool shouldNotify = false) {
            var inner = new Inner(path);

            if (!reader.Eof()) {
                inner.ReplaceAll(reader.Nested((int) reader.ReadUInt32()), shouldNotify: false);
            }

            return inner;
        }

        public Int16 WireType(UInt16 tag) {
            switch (tag) {
                case 0: return (Int16) Steit.Encoding.WireType.Varint;
                case 1: return (Int16) Steit.Encoding.WireType.Varint;
                default: return -1;
            }
        }

        public IState Nested(UInt16 tag) {
            switch (tag) {
                default: return null;
            }
        }

        public bool IsAddSupported() { return false; }
        public bool IsRemoveSupported() { return false; }

        public void ReplayAdd(Reader reader) { throw new Exception("Not supported"); }
        public void ReplayRemove(UInt16 tag) { throw new Exception("Not supported"); }

        public void ReplaceAt(UInt16 tag, WireType wireType, Reader reader, bool shouldNotify) {
            switch (tag) {
                case 0: this.Foo = this.Notify(reader.ReadInt32(), this.Foo, shouldNotify, fooListeners); break;
                case 1: this.Bar = this.Notify(reader.ReadBoolean(), this.Bar, shouldNotify, barListeners); break;
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
