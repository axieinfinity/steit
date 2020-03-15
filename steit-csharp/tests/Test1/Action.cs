using System;
using System.Collections.Generic;

using Steit;
using Steit.Builtins;
using Steit.Collections;
using Steit.Encoding;
using Steit.State;

namespace Test1 {
    public sealed class Action : IEnumState {
        public static UInt16 RAW_VARIANT = 0;
        public static UInt16 ATTACK_VARIANT = 1;

        private static IList<Listener> listeners = new List<Listener>();

        public Path Path { get; private set; }

        public UInt16 Variant { get; private set; }
        public IState Value { get; private set; }

        public Raw RawValue { get { return this.Variant == 0 ? (Raw) this.Value : null; } }
        public Attack AttackValue { get { return this.Variant == 1 ? (Attack) this.Value : null; } }

        public Action(Path path = null) {
            this.Path = path != null ? path : Path.Root;
            this.Value = new Raw(this.Path.Nested(0));
        }

        public delegate void Listener(IState newValue, UInt16 newVariant, IState oldValue, UInt16 oldVariant, Action container);

        public static int OnUpdate(Listener listener) { return Utilities.Add(listeners, listener); }
        public static void RemoveListener(Listener listener) { listeners.Remove(listener); }
        public static void RemoveListenerAt(int index) { listeners.RemoveAt(index); }
        public static void ClearListeners() { listeners.Clear(); }

        public static Action Deserialize(Reader reader, Path path = null, bool shouldNotify = false) {
            var action = new Action(path);
            action.ReplaceAll(reader, shouldNotify);
            return action;
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
                case 0: this.NotifyAndUpdate(0, Raw.Deserialize(reader, this.Path.Nested(0)), shouldNotify); break;
                case 1: this.NotifyAndUpdate(1, Attack.Deserialize(reader, this.Path.Nested(1)), shouldNotify); break;
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

        public sealed class Raw : IState {
            private static IList<Listener<StateList<Byte>>> logEntriesListeners = new List<Listener<StateList<Byte>>>();

            public Path Path { get; private set; }

            public StateList<Byte> LogEntries { get; private set; }

            // This is not meant to be used directly.
            public Raw(Path path = null) {
                this.Path = path != null ? path : Path.Root;
                this.LogEntries = new StateList<Byte>(this.Path.Nested(0));
            }

            public delegate void Listener<T>(T newValue, T oldValue, Raw container);

            public static int OnUpdateLogEntries(Listener<StateList<Byte>> listener) { return Utilities.Add(logEntriesListeners, listener); }

            public static void RemoveLogEntriesListener(Listener<StateList<Byte>> listener) { logEntriesListeners.Remove(listener); }

            public static void RemoveLogEntriesListenerAt(int index) { logEntriesListeners.RemoveAt(index); }

            public static void ClearLogEntriesListeners() { logEntriesListeners.Clear(); }

            public static void ClearAllListeners() {
                logEntriesListeners.Clear();
            }

            // This is not meant to be used directly.
            public static Raw Deserialize(Reader reader, Path path = null, bool shouldNotify = false) {
                var raw = new Raw(path);
                raw.ReplaceAll(reader, shouldNotify);
                return raw;
            }

            public Int16 WireType(UInt16 tag) {
                switch (tag) {
                    case 0: return (Int16) Steit.Encoding.WireType.Sized;
                    default: return -1;
                }
            }

            public IState Nested(UInt16 tag) {
                switch (tag) {
                    case 0: return this.LogEntries;
                    default: return null;
                }
            }

            public bool IsAddSupported() { return false; }
            public bool IsRemoveSupported() { return false; }

            public void ReplayAdd(Reader reader) { throw new Exception("Not supported"); }
            public void ReplayRemove(UInt16 tag) { throw new Exception("Not supported"); }

            public void ReplaceAt(UInt16 tag, WireType wireType, Reader reader, bool shouldNotify) {
                switch (tag) {
                    case 0: this.LogEntries = this.Notify(StateList<Byte>.Deserialize(reader.Nested(), this.Path.Nested(0)), this.LogEntries, shouldNotify, logEntriesListeners); break;
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

        public sealed class Attack : IState {
            private static IList<Listener<Byte>> attackerListeners = new List<Listener<Byte>>();
            private static IList<Listener<Byte>> defenderListeners = new List<Listener<Byte>>();
            private static IList<Listener<StateList<Hit>>> hitsListeners = new List<Listener<StateList<Hit>>>();

            public Path Path { get; private set; }

            public Byte Attacker { get; private set; }
            public Byte Defender { get; private set; }
            public StateList<Hit> Hits { get; private set; }

            // This is not meant to be used directly.
            public Attack(Path path = null) {
                this.Path = path != null ? path : Path.Root;
                this.Hits = new StateList<Hit>(this.Path.Nested(2));
            }

            public delegate void Listener<T>(T newValue, T oldValue, Attack container);

            public static int OnUpdateAttacker(Listener<Byte> listener) { return Utilities.Add(attackerListeners, listener); }
            public static int OnUpdateDefender(Listener<Byte> listener) { return Utilities.Add(defenderListeners, listener); }
            public static int OnUpdateHits(Listener<StateList<Hit>> listener) { return Utilities.Add(hitsListeners, listener); }

            public static void RemoveAttackerListener(Listener<Byte> listener) { attackerListeners.Remove(listener); }
            public static void RemoveDefenderListener(Listener<Byte> listener) { defenderListeners.Remove(listener); }
            public static void RemoveHitsListener(Listener<StateList<Hit>> listener) { hitsListeners.Remove(listener); }

            public static void RemoveAttackerListenerAt(int index) { attackerListeners.RemoveAt(index); }
            public static void RemoveDefenderListenerAt(int index) { defenderListeners.RemoveAt(index); }
            public static void RemoveHitsListenerAt(int index) { hitsListeners.RemoveAt(index); }

            public static void ClearAttackerListeners() { attackerListeners.Clear(); }
            public static void ClearDefenderListeners() { defenderListeners.Clear(); }
            public static void ClearHitsListeners() { hitsListeners.Clear(); }

            public static void ClearAllListeners() {
                attackerListeners.Clear();
                defenderListeners.Clear();
                hitsListeners.Clear();
            }

            // This is not meant to be used directly.
            public static Attack Deserialize(Reader reader, Path path = null, bool shouldNotify = false) {
                var attack = new Attack(path);
                attack.ReplaceAll(reader, shouldNotify);
                return attack;
            }

            public Int16 WireType(UInt16 tag) {
                switch (tag) {
                    case 0: return (Int16) Steit.Encoding.WireType.Varint;
                    case 1: return (Int16) Steit.Encoding.WireType.Varint;
                    case 2: return (Int16) Steit.Encoding.WireType.Sized;
                    default: return -1;
                }
            }

            public IState Nested(UInt16 tag) {
                switch (tag) {
                    case 2: return this.Hits;
                    default: return null;
                }
            }

            public bool IsAddSupported() { return false; }
            public bool IsRemoveSupported() { return false; }

            public void ReplayAdd(Reader reader) { throw new Exception("Not supported"); }
            public void ReplayRemove(UInt16 tag) { throw new Exception("Not supported"); }

            public void ReplaceAt(UInt16 tag, WireType wireType, Reader reader, bool shouldNotify) {
                switch (tag) {
                    case 0: this.Attacker = this.Notify(reader.ReadByte(), this.Attacker, shouldNotify, attackerListeners); break;
                    case 1: this.Defender = this.Notify(reader.ReadByte(), this.Defender, shouldNotify, defenderListeners); break;
                    case 2: this.Hits = this.Notify(StateList<Hit>.Deserialize(reader.Nested(), this.Path.Nested(2)), this.Hits, shouldNotify, hitsListeners); break;
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
