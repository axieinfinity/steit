using System;
using System.Collections.Generic;

using Steit;
using Steit.Collections;
using Steit.Encoding;
using Steit.State;

namespace Steit.Test1 {
    public sealed class Hit : IState {
        private static IList<Listener<Action>> beforeAttackingListeners = new List<Listener<Action>>();
        private static IList<Listener<Action>> beforeDamagingListeners = new List<Listener<Action>>();
        private static IList<Listener<Action>> damagingListeners = new List<Listener<Action>>();
        private static IList<Listener<Action>> afterDamagingListeners = new List<Listener<Action>>();
        private static IList<Listener<Action>> afterAttackingListeners = new List<Listener<Action>>();
        private static IList<Listener<Int32>> dummyListeners = new List<Listener<Int32>>();

        public Path Path { get; private set; }

        public Action BeforeAttacking { get; private set; }
        public Action BeforeDamaging { get; private set; }
        public Action Damaging { get; private set; }
        public Action AfterDamaging { get; private set; }
        public Action AfterAttacking { get; private set; }
        public Int32 Dummy { get; private set; }

        public Hit(Path path = null) {
            this.Path = path != null ? path : Path.Root;
            this.BeforeAttacking = new Action(this.Path.Nested(0));
            this.BeforeDamaging = new Action(this.Path.Nested(1));
            this.Damaging = new Action(this.Path.Nested(2));
            this.AfterDamaging = new Action(this.Path.Nested(3));
            this.AfterAttacking = new Action(this.Path.Nested(4));
        }

        public delegate void Listener<T>(T newValue, T oldValue, Hit container);

        public static int OnUpdateBeforeAttacking(Listener<Action> listener) { return Utilities.Add(beforeAttackingListeners, listener); }
        public static int OnUpdateBeforeDamaging(Listener<Action> listener) { return Utilities.Add(beforeDamagingListeners, listener); }
        public static int OnUpdateDamaging(Listener<Action> listener) { return Utilities.Add(damagingListeners, listener); }
        public static int OnUpdateAfterDamaging(Listener<Action> listener) { return Utilities.Add(afterDamagingListeners, listener); }
        public static int OnUpdateAfterAttacking(Listener<Action> listener) { return Utilities.Add(afterAttackingListeners, listener); }
        public static int OnUpdateDummy(Listener<Int32> listener) { return Utilities.Add(dummyListeners, listener); }

        public static void RemoveBeforeAttackingListener(Listener<Action> listener) { beforeAttackingListeners.Remove(listener); }
        public static void RemoveBeforeDamagingListener(Listener<Action> listener) { beforeDamagingListeners.Remove(listener); }
        public static void RemoveDamagingListener(Listener<Action> listener) { damagingListeners.Remove(listener); }
        public static void RemoveAfterDamagingListener(Listener<Action> listener) { afterDamagingListeners.Remove(listener); }
        public static void RemoveAfterAttackingListener(Listener<Action> listener) { afterAttackingListeners.Remove(listener); }
        public static void RemoveDummyListener(Listener<Int32> listener) { dummyListeners.Remove(listener); }

        public static void RemoveBeforeAttackingListenerAt(int index) { beforeAttackingListeners.RemoveAt(index); }
        public static void RemoveBeforeDamagingListenerAt(int index) { beforeDamagingListeners.RemoveAt(index); }
        public static void RemoveDamagingListenerAt(int index) { damagingListeners.RemoveAt(index); }
        public static void RemoveAfterDamagingListenerAt(int index) { afterDamagingListeners.RemoveAt(index); }
        public static void RemoveAfterAttackingListenerAt(int index) { afterAttackingListeners.RemoveAt(index); }
        public static void RemoveDummyListenerAt(int index) { dummyListeners.RemoveAt(index); }

        public static void ClearBeforeAttackingListeners() { beforeAttackingListeners.Clear(); }
        public static void ClearBeforeDamagingListeners() { beforeDamagingListeners.Clear(); }
        public static void ClearDamagingListeners() { damagingListeners.Clear(); }
        public static void ClearAfterDamagingListeners() { afterDamagingListeners.Clear(); }
        public static void ClearAfterAttackingListeners() { afterAttackingListeners.Clear(); }
        public static void ClearDummyListeners() { dummyListeners.Clear(); }

        public static void ClearAllListeners() {
            beforeAttackingListeners.Clear();
            beforeDamagingListeners.Clear();
            damagingListeners.Clear();
            afterDamagingListeners.Clear();
            afterAttackingListeners.Clear();
            dummyListeners.Clear();
        }

        public static Hit Deserialize(Reader reader, Path path = null, bool shouldNotify = false) {
            var hit = new Hit(path);
            hit.ReplaceAll(reader, shouldNotify);
            return hit;
        }

        public Int16 WireType(UInt16 tag) {
            switch (tag) {
                case 0: return (Int16) Steit.Encoding.WireType.Sized;
                case 1: return (Int16) Steit.Encoding.WireType.Sized;
                case 2: return (Int16) Steit.Encoding.WireType.Sized;
                case 3: return (Int16) Steit.Encoding.WireType.Sized;
                case 4: return (Int16) Steit.Encoding.WireType.Sized;
                case 5: return (Int16) Steit.Encoding.WireType.Varint;
                default: return -1;
            }
        }

        public IState Nested(UInt16 tag) {
            switch (tag) {
                case 0: return this.BeforeAttacking;
                case 1: return this.BeforeDamaging;
                case 2: return this.Damaging;
                case 3: return this.AfterDamaging;
                case 4: return this.AfterAttacking;
                default: return null;
            }
        }

        public bool IsAddSupported() { return false; }
        public bool IsRemoveSupported() { return false; }

        public void ReplayAdd(Reader reader) { throw new Exception("Not supported"); }
        public void ReplayRemove(UInt16 tag) { throw new Exception("Not supported"); }

        public void ReplaceAt(UInt16 tag, WireType wireType, Reader reader, bool shouldNotify) {
            switch (tag) {
                case 0: this.BeforeAttacking = this.Notify(Action.Deserialize(reader.Nested((int) reader.ReadUInt32()), this.Path.Nested(0)), this.BeforeAttacking, shouldNotify, beforeAttackingListeners); break;
                case 1: this.BeforeDamaging = this.Notify(Action.Deserialize(reader.Nested((int) reader.ReadUInt32()), this.Path.Nested(1)), this.BeforeDamaging, shouldNotify, beforeDamagingListeners); break;
                case 2: this.Damaging = this.Notify(Action.Deserialize(reader.Nested((int) reader.ReadUInt32()), this.Path.Nested(2)), this.Damaging, shouldNotify, damagingListeners); break;
                case 3: this.AfterDamaging = this.Notify(Action.Deserialize(reader.Nested((int) reader.ReadUInt32()), this.Path.Nested(3)), this.AfterDamaging, shouldNotify, afterDamagingListeners); break;
                case 4: this.AfterAttacking = this.Notify(Action.Deserialize(reader.Nested((int) reader.ReadUInt32()), this.Path.Nested(4)), this.AfterAttacking, shouldNotify, afterAttackingListeners); break;
                case 5: this.Dummy = this.Notify(reader.ReadInt32(), this.Dummy, shouldNotify, dummyListeners); break;
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
