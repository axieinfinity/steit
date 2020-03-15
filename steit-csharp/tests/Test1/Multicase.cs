using System;
using System.Collections.Generic;

using Steit;
using Steit.Builtins;
using Steit.Collections;
using Steit.Encoding;
using Steit.State;

namespace Test1 {
    public sealed class Multicase : IEnumState {
        public static UInt16 FIRST_CASE_VARIANT = 0;
        public static UInt16 SECOND_CASE_VARIANT = 1;

        private static IList<Listener> listeners = new List<Listener>();

        public Path Path { get; private set; }

        public UInt16 Variant { get; private set; }
        public IState Value { get; private set; }

        public FirstCase FirstCaseValue { get { return this.Variant == 0 ? (FirstCase) this.Value : null; } }
        public SecondCase SecondCaseValue { get { return this.Variant == 1 ? (SecondCase) this.Value : null; } }

        public Multicase(Path path = null) {
            this.Path = path != null ? path : Path.Root;
            this.Value = new FirstCase(this.Path.Nested(0));
        }

        public delegate void Listener(IState newValue, UInt16 newVariant, IState oldValue, UInt16 oldVariant, Multicase container);

        public static int OnUpdate(Listener listener) { return Utilities.Add(listeners, listener); }
        public static void RemoveListener(Listener listener) { listeners.Remove(listener); }
        public static void RemoveListenerAt(int index) { listeners.RemoveAt(index); }
        public static void ClearListeners() { listeners.Clear(); }

        public static Multicase Deserialize(Reader reader, Path path = null, bool shouldNotify = false) {
            var multicase = new Multicase(path);
            multicase.ReplaceAll(reader, shouldNotify);
            return multicase;
        }

        public Int16 WireType(UInt16 tag) {
            switch (tag) {
                case 0: return (Int16) Steit.Encoding.WireType.Sized;
                case 1: return (Int16) Steit.Encoding.WireType.Sized;
                default: return -1;
            }
        }

        public IState Nested(UInt16 tag) {
            return tag == this.Variant ? this.Value : null;
        }

        public bool IsAddSupported() { return false; }
        public bool IsRemoveSupported() { return false; }

        public void ReplayAdd(Reader reader) { throw new Exception("Not supported"); }
        public void ReplayRemove(UInt16 tag) { throw new Exception("Not supported"); }

        public void ReplaceAt(UInt16 tag, WireType wireType, Reader reader, bool shouldNotify) {
            reader = !reader.Eof() ? reader : new Reader(new byte[0]);

            switch (tag) {
                case 0: this.NotifyAndUpdate(0, FirstCase.Deserialize(reader, this.Path.Nested(0)), shouldNotify); break;
                case 1: this.NotifyAndUpdate(1, SecondCase.Deserialize(reader, this.Path.Nested(1)), shouldNotify); break;
                default: reader.Exhaust(); break;
            }
        }

        private void NotifyAndUpdate(UInt16 newVariant, IState newValue, bool shouldNotify) {
            if (shouldNotify) {
                foreach (var listener in listeners) {
                    listener(newValue, newVariant, this.Value, this.Variant, this);
                }
            }

            this.Variant = newVariant;
            this.Value = newValue;
        }

        public sealed class FirstCase : IState {
            private static IList<Listener<Int32>> fooListeners = new List<Listener<Int32>>();
            private static IList<Listener<Boolean>> barListeners = new List<Listener<Boolean>>();

            public Path Path { get; private set; }

            public Int32 Foo { get; private set; }
            public Boolean Bar { get; private set; }

            // This is not meant to be used directly.
            public FirstCase(Path path = null) {
                this.Path = path != null ? path : Path.Root;
            }

            public delegate void Listener<T>(T newValue, T oldValue, FirstCase container);

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

            // This is not meant to be used directly.
            public static FirstCase Deserialize(Reader reader, Path path = null, bool shouldNotify = false) {
                var firstCase = new FirstCase(path);
                firstCase.ReplaceAll(reader, shouldNotify);
                return firstCase;
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

        public sealed class SecondCase : IState {
            private static IList<Listener<Int32>> fooListeners = new List<Listener<Int32>>();
            private static IList<Listener<Boolean>> barListeners = new List<Listener<Boolean>>();

            public Path Path { get; private set; }

            public Int32 Foo { get; private set; }
            public Boolean Bar { get; private set; }

            // This is not meant to be used directly.
            public SecondCase(Path path = null) {
                this.Path = path != null ? path : Path.Root;
            }

            public delegate void Listener<T>(T newValue, T oldValue, SecondCase container);

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

            // This is not meant to be used directly.
            public static SecondCase Deserialize(Reader reader, Path path = null, bool shouldNotify = false) {
                var secondCase = new SecondCase(path);
                secondCase.ReplaceAll(reader, shouldNotify);
                return secondCase;
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
}
