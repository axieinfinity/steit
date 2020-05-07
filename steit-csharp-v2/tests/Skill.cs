using System;

using Steit.Builtins;
using Steit.Codec;
using Steit.Collections;
using Steit.State;
using Steit.State.Event;

namespace Just.To.Test {
    public sealed class Skill : IState {
        public Path Path { get; }

        public UInt16 TargetIndex { get; private set; }
        public Vector<Action> PreCast { get; private set; }
        public Vector<Action> Cast { get; private set; }
        public Vector<Sure<Action>> PostCast { get; private set; }

        public Skill(Path? path = null) {
            this.Path = path ?? Path.Root;
            this.PreCast = new Vector<Action>(this.Path.GetNested(1));
            this.Cast = new Vector<Action>(this.Path.GetNested(2));
            this.PostCast = new Vector<Sure<Action>>(this.Path.GetNested(3));
        }

        public static event EventHandler<FieldUpdateEventArgs<UInt16, Skill>>? OnTargetIndexUpdate;
        public static event EventHandler<FieldUpdateEventArgs<Vector<Action>, Skill>>? OnPreCastUpdate;
        public static event EventHandler<FieldUpdateEventArgs<Vector<Action>, Skill>>? OnCastUpdate;
        public static event EventHandler<FieldUpdateEventArgs<Vector<Sure<Action>>, Skill>>? OnPostCastUpdate;

        public static void ClearTargetIndexUpdateHandlers() { OnTargetIndexUpdate = null; }
        public static void ClearPreCastUpdateHandlers() { OnPreCastUpdate = null; }
        public static void ClearCastUpdateHandlers() { OnCastUpdate = null; }
        public static void ClearPostCastUpdateHandlers() { OnPostCastUpdate = null; }

        public static void ClearUpdateHandlers() {
            OnTargetIndexUpdate = null;
            OnPreCastUpdate = null;
            OnCastUpdate = null;
            OnPostCastUpdate = null;
        }

        public static Skill Deserialize(IReader reader, Path? path = null) {
            var skill = new Skill(path);
            skill.Replace(reader);
            return skill;
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
                case 1: return this.PreCast;
                case 2: return this.Cast;
                case 3: return this.PostCast;
                default: return null;
            }
        }

        public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
            switch (tag) {
                case 0: this.TargetIndex = this.MaybeNotify(0, reader.ReadUInt16(), this.TargetIndex, OnTargetIndexUpdate, shouldNotify); break;
                case 1: this.PreCast = this.MaybeNotify(1, Vector<Action>.Deserialize(reader.GetNested(), this.Path.GetNested(1)), this.PreCast, OnPreCastUpdate, shouldNotify); break;
                case 2: this.Cast = this.MaybeNotify(2, Vector<Action>.Deserialize(reader.GetNested(), this.Path.GetNested(2)), this.Cast, OnCastUpdate, shouldNotify); break;
                case 3: this.PostCast = this.MaybeNotify(3, Vector<Sure<Action>>.Deserialize(reader.GetNested(), this.Path.GetNested(3)), this.PostCast, OnPostCastUpdate, shouldNotify); break;
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
            EventHandler<FieldUpdateEventArgs<TValue, Skill>>? handler,
            bool shouldNotify
        ) {
            if (shouldNotify) {
                var args = new FieldUpdateEventArgs<TValue, Skill>(tag, newValue, oldValue, this);
                handler?.Invoke(this, args);
            }

            return newValue;
        }
    }
}
