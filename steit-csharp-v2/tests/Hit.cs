using System;

using Steit.Builtins;
using Steit.Codec;
using Steit.Collections;
using Steit.State;
using Steit.State.Event;

namespace Just.To.Test {
    public sealed class Hit : IState {
        public Path Path { get; }

        public Boolean IsMiss { get; private set; }
        public Vector<Action> PreDamage { get; private set; }
        public Vector<Action> Damage { get; private set; }
        public Vector<Action> PostDamage { get; private set; }

        public Hit(Path? path = null) {
            this.Path = path ?? Path.Root;
            this.PreDamage = new Vector<Action>(this.Path.GetNested(1));
            this.Damage = new Vector<Action>(this.Path.GetNested(2));
            this.PostDamage = new Vector<Action>(this.Path.GetNested(3));
        }

        public static event EventHandler<FieldUpdateEventArgs<Boolean, Hit>>? OnIsMissUpdate;
        public static event EventHandler<FieldUpdateEventArgs<Vector<Action>, Hit>>? OnPreDamageUpdate;
        public static event EventHandler<FieldUpdateEventArgs<Vector<Action>, Hit>>? OnDamageUpdate;
        public static event EventHandler<FieldUpdateEventArgs<Vector<Action>, Hit>>? OnPostDamageUpdate;

        public static void ClearIsMissUpdateHandlers() { OnIsMissUpdate = null; }
        public static void ClearPreDamageUpdateHandlers() { OnPreDamageUpdate = null; }
        public static void ClearDamageUpdateHandlers() { OnDamageUpdate = null; }
        public static void ClearPostDamageUpdateHandlers() { OnPostDamageUpdate = null; }

        public static void ClearUpdateHandlers() {
            OnIsMissUpdate = null;
            OnPreDamageUpdate = null;
            OnDamageUpdate = null;
            OnPostDamageUpdate = null;
        }

        public static Hit Deserialize(IReader reader, Path? path = null) {
            var hit = new Hit(path);
            hit.Replace(reader);
            return hit;
        }

        public WireType? GetWireType(UInt32 tag) {
            switch (tag) {
                case 0: return WireType.Varint;
                case 1: return WireType.Sized;
                case 2: return WireType.Sized;
                case 3: return WireType.Sized;
                default: return null;
            }
        }

        public IState? GetNested(UInt32 tag) {
            switch (tag) {
                case 1: return this.PreDamage;
                case 2: return this.Damage;
                case 3: return this.PostDamage;
                default: return null;
            }
        }

        public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
            switch (tag) {
                case 0: this.IsMiss = this.MaybeNotify(0, reader.ReadBoolean(), this.IsMiss, OnIsMissUpdate, shouldNotify); break;
                case 1: this.PreDamage = this.MaybeNotify(1, Vector<Action>.Deserialize(reader, this.Path.GetNested(1)), this.PreDamage, OnPreDamageUpdate, shouldNotify); break;
                case 2: this.Damage = this.MaybeNotify(2, Vector<Action>.Deserialize(reader, this.Path.GetNested(2)), this.Damage, OnDamageUpdate, shouldNotify); break;
                case 3: this.PostDamage = this.MaybeNotify(3, Vector<Action>.Deserialize(reader, this.Path.GetNested(3)), this.PostDamage, OnPostDamageUpdate, shouldNotify); break;
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
            EventHandler<FieldUpdateEventArgs<TValue, Hit>>? handler,
            bool shouldNotify
        ) {
            if (shouldNotify) {
                var args = new FieldUpdateEventArgs<TValue, Hit>(tag, newValue, oldValue, this);
                handler?.Invoke(this, args);
            }

            return newValue;
        }
    }
}
