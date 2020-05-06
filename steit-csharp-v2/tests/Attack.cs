using System;

using Steit.Builtins;
using Steit.Codec;
using Steit.Collections;
using Steit.State;

namespace Just.To.Test {
    public sealed class Attack : IState {
        public Path Path { get; }

        public UInt16 TargetIndex { get; private set; }
        public Vector<Action> BeforeHits { get; private set; }
        public ActionsOr<Vector<ActionsOr<Hit>>> Hits { get; private set; }
        public Vector<Action> AfterHits { get; private set; }

        public Attack(Path? path = null) {
            this.Path = path ?? Path.Root;
            this.BeforeHits = new Vector<Action>(this.Path.GetNested(1));
            this.Hits = new ActionsOr<Vector<ActionsOr<Hit>>>(this.Path.GetNested(2));
            this.AfterHits = new Vector<Action>(this.Path.GetNested(3));
        }

        public static event EventHandler<FieldUpdateEventArgs<UInt16, Attack>>? OnTargetIndexUpdate;
        public static event EventHandler<FieldUpdateEventArgs<Vector<Action>, Attack>>? OnBeforeHitsUpdate;
        public static event EventHandler<FieldUpdateEventArgs<ActionsOr<Vector<ActionsOr<Hit>>>, Attack>>? OnHitsUpdate;
        public static event EventHandler<FieldUpdateEventArgs<Vector<Action>, Attack>>? OnAfterHitsUpdate;

        public static void ClearTargetIndexUpdateHandlers() { OnTargetIndexUpdate = null; }
        public static void ClearBeforeHitsUpdateHandlers() { OnBeforeHitsUpdate = null; }
        public static void ClearHitsUpdateHandlers() { OnHitsUpdate = null; }
        public static void ClearAfterHitsUpdateHandlers() { OnAfterHitsUpdate = null; }

        public static void ClearUpdateHandlers() {
            OnTargetIndexUpdate = null;
            OnBeforeHitsUpdate = null;
            OnHitsUpdate = null;
            OnAfterHitsUpdate = null;
        }

        public static Attack Deserialize(IReader reader, Path? path = null) {
            var attack = new Attack(path);
            attack.Replace(reader);
            return attack;
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
                case 1: return this.BeforeHits;
                case 2: return this.Hits;
                case 3: return this.AfterHits;
                default: return null;
            }
        }

        public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
            switch (tag) {
                case 0: this.TargetIndex = this.MaybeNotify(0, reader.ReadUInt16(), this.TargetIndex, OnTargetIndexUpdate, shouldNotify); break;
                case 1: this.BeforeHits = this.MaybeNotify(1, Vector<Action>.Deserialize(reader.GetNested(), this.Path.GetNested(1)), this.BeforeHits, OnBeforeHitsUpdate, shouldNotify); break;
                case 2: this.Hits = this.MaybeNotify(2, ActionsOr<Vector<ActionsOr<Hit>>>.Deserialize(reader.GetNested(), this.Path.GetNested(2)), this.Hits, OnHitsUpdate, shouldNotify); break;
                case 3: this.AfterHits = this.MaybeNotify(3, Vector<Action>.Deserialize(reader.GetNested(), this.Path.GetNested(3)), this.AfterHits, OnAfterHitsUpdate, shouldNotify); break;
                default: reader.SkipField(wireType); break;
            }
        }

        public bool IsList() { return false; }
        public void ReplayListPush(IReader reader) { throw new NotSupportedException(); }
        public void ReplayListPop() { throw new NotSupportedException(); }

        public bool IsMap() { return false; }
        public void ReplayMapInsert(IReader reader) { throw new NotSupportedException(); }
        public void ReplayMapRemove(IReader reader) { throw new NotSupportedException(); }

        private TValue MaybeNotify<TValue>(
            UInt32 tag,
            TValue newValue,
            TValue oldValue,
            EventHandler<FieldUpdateEventArgs<TValue, Attack>>? handler,
            bool shouldNotify
        ) {
            if (shouldNotify) {
                var args = new FieldUpdateEventArgs<TValue, Attack>(tag, newValue, oldValue, this);
                handler?.Invoke(this, args);
            }

            return newValue;
        }
    }
}
