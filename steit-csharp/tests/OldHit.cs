using System;

using Steit.Builtins;
using Steit.Codec;
using Steit.Collections;
using Steit.State;
using Steit.State.Event;

namespace Just.To.Test {
    public sealed partial class OldHit : IState {
        public Path Path { get; }

        public OldAction BeforeAttacking { get; private set; }
        public OldAction BeforeDamaging { get; private set; }
        public OldAction Damaging { get; private set; }
        public OldAction AfterDamaging { get; private set; }
        public OldAction AfterAttacking { get; private set; }
        public Int32 Dummy { get; private set; }

        public OldHit(Path path = null) {
            this.Path = path ?? Path.Root;
            this.BeforeAttacking = new OldAction(this.Path.GetNested(0));
            this.BeforeDamaging = new OldAction(this.Path.GetNested(1));
            this.Damaging = new OldAction(this.Path.GetNested(2));
            this.AfterDamaging = new OldAction(this.Path.GetNested(3));
            this.AfterAttacking = new OldAction(this.Path.GetNested(4));
        }

        public static event EventHandler<FieldUpdateEventArgs<OldAction, OldHit>> OnBeforeAttackingUpdate;
        public static event EventHandler<FieldUpdateEventArgs<OldAction, OldHit>> OnBeforeDamagingUpdate;
        public static event EventHandler<FieldUpdateEventArgs<OldAction, OldHit>> OnDamagingUpdate;
        public static event EventHandler<FieldUpdateEventArgs<OldAction, OldHit>> OnAfterDamagingUpdate;
        public static event EventHandler<FieldUpdateEventArgs<OldAction, OldHit>> OnAfterAttackingUpdate;
        public static event EventHandler<FieldUpdateEventArgs<Int32, OldHit>> OnDummyUpdate;

        public static void ClearBeforeAttackingUpdateHandlers() { OnBeforeAttackingUpdate = null; }
        public static void ClearBeforeDamagingUpdateHandlers() { OnBeforeDamagingUpdate = null; }
        public static void ClearDamagingUpdateHandlers() { OnDamagingUpdate = null; }
        public static void ClearAfterDamagingUpdateHandlers() { OnAfterDamagingUpdate = null; }
        public static void ClearAfterAttackingUpdateHandlers() { OnAfterAttackingUpdate = null; }
        public static void ClearDummyUpdateHandlers() { OnDummyUpdate = null; }

        public static void ClearUpdateHandlers() {
            OnBeforeAttackingUpdate = null;
            OnBeforeDamagingUpdate = null;
            OnDamagingUpdate = null;
            OnAfterDamagingUpdate = null;
            OnAfterAttackingUpdate = null;
            OnDummyUpdate = null;
        }

        public static OldHit Deserialize(IReader reader, Path path = null) {
            var oldHit = new OldHit(path);
            oldHit.Replace(reader, shouldNotify: false);
            return oldHit;
        }

        public WireType? GetWireType(UInt32 tag) {
            switch (tag) {
                case 0: return WireType.Sized;
                case 1: return WireType.Sized;
                case 2: return WireType.Sized;
                case 3: return WireType.Sized;
                case 4: return WireType.Sized;
                case 5: return WireType.Varint;
                default: return null;
            }
        }

        public IState GetNested(UInt32 tag) {
            switch (tag) {
                case 0: return this.BeforeAttacking;
                case 1: return this.BeforeDamaging;
                case 2: return this.Damaging;
                case 3: return this.AfterDamaging;
                case 4: return this.AfterAttacking;
                default: return null;
            }
        }

        public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
            switch (tag) {
                case 0: this.BeforeAttacking = this.MaybeNotify(0, OldAction.Deserialize(reader, this.Path.GetNested(0)), this.BeforeAttacking, OnBeforeAttackingUpdate, shouldNotify); break;
                case 1: this.BeforeDamaging = this.MaybeNotify(1, OldAction.Deserialize(reader, this.Path.GetNested(1)), this.BeforeDamaging, OnBeforeDamagingUpdate, shouldNotify); break;
                case 2: this.Damaging = this.MaybeNotify(2, OldAction.Deserialize(reader, this.Path.GetNested(2)), this.Damaging, OnDamagingUpdate, shouldNotify); break;
                case 3: this.AfterDamaging = this.MaybeNotify(3, OldAction.Deserialize(reader, this.Path.GetNested(3)), this.AfterDamaging, OnAfterDamagingUpdate, shouldNotify); break;
                case 4: this.AfterAttacking = this.MaybeNotify(4, OldAction.Deserialize(reader, this.Path.GetNested(4)), this.AfterAttacking, OnAfterAttackingUpdate, shouldNotify); break;
                case 5: this.Dummy = this.MaybeNotify(5, reader.ReadInt32(), this.Dummy, OnDummyUpdate, shouldNotify); break;
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
            EventHandler<FieldUpdateEventArgs<TValue, OldHit>> handler,
            bool shouldNotify
        ) {
            if (shouldNotify) {
                var args = new FieldUpdateEventArgs<TValue, OldHit>(tag, newValue, oldValue, this);
                handler?.Invoke(this, args);
            }

            return newValue;
        }
    }
}
