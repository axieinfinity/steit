using System;

using Steit.Builtins;
using Steit.Codec;
using Steit.Collections;
using Steit.State;
using Steit.State.Event;

namespace Just.To.Test {
    public sealed partial class OldAction : IEnumState {
        public const UInt32 RawTag = 0;
        public const UInt32 AttackTag = 1;

        public Path Path { get; }

        public UInt32 Tag { get; private set; }
        public IState Variant { get; private set; }

        public Raw RawVariant { get { return this.Variant as Raw; } }
        public Attack AttackVariant { get { return this.Variant as Attack; } }

        public OldAction(Path path = null) : this(path, 0) { }

        public OldAction(Path path, UInt32 tag) {
            this.Path = path ?? Path.Root;
            this.Tag = tag;

            switch (tag) {
                case 0: this.Variant = new Raw(this.Path.GetNested(0)); break;
                case 1: this.Variant = new Attack(this.Path.GetNested(1)); break;
                default: this.Variant = new Raw(this.Path.GetNested(0)); break;
            }
        }

        public static OldAction NewRaw(Path path = null) { return new OldAction(path, 0); }
        public static OldAction NewAttack(Path path = null) { return new OldAction(path, 1); }

        public static event EventHandler<VariantUpdateEventArgs<OldAction>> OnUpdate;

        public static void ClearUpdateHandlers() {
            OnUpdate = null;
        }

        public static OldAction Deserialize(IReader reader, Path path = null) {
            var oldAction = new OldAction(path);
            oldAction.Replace(reader, shouldNotify: false);
            return oldAction;
        }

        public WireType? GetWireType(UInt32 tag) {
            switch (tag) {
                case 0: return WireType.Sized;
                case 1: return WireType.Sized;
                default: return null;
            }
        }

        public IState GetNested(UInt32 tag) {
            return tag == this.Tag ? this.Variant : null;
        }

        public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
            switch (tag) {
                case 0: this.UpdateAndNotify(0, Raw.Deserialize(reader, this.Path.GetNested(0)), shouldNotify); break;
                case 1: this.UpdateAndNotify(1, Attack.Deserialize(reader, this.Path.GetNested(1)), shouldNotify); break;
                default: reader.SkipToEnd(); break;
            }
        }

        public void ReplayListPush(IReader reader) { throw new NotSupportedException(); }
        public void ReplayListPop() { throw new NotSupportedException(); }
        public void ReplayMapRemove(UInt32 key) { throw new NotSupportedException(); }

        private void UpdateAndNotify(UInt32 newTag, IState newVariant, bool shouldNotify) {
            if (shouldNotify) {
                var args = new VariantUpdateEventArgs<OldAction>(newTag, newVariant, this.Tag, this.Variant, this);
                OldAction.OnUpdate?.Invoke(this, args);
            }

            this.Tag = newTag;
            this.Variant = newVariant;
        }

        // Variant (0): Raw

        public sealed partial class Raw : IState {
            public Path Path { get; }
            public StateList<Byte> LogEntries { get; private set; }

            internal Raw(Path path = null) {
                this.Path = path ?? Path.Root;
                this.LogEntries = new StateList<Byte>(this.Path.GetNested(0));
            }

            public static event EventHandler<FieldUpdateEventArgs<StateList<Byte>, Raw>> OnLogEntriesUpdate;

            public static void ClearLogEntriesUpdateHandlers() {
                OnLogEntriesUpdate = null;
            }

            public static void ClearUpdateHandlers() {
                OnLogEntriesUpdate = null;
            }

            internal static Raw Deserialize(IReader reader, Path path = null) {
                var raw = new Raw(path);
                raw.Replace(reader, shouldNotify: false);
                return raw;
            }

            public WireType? GetWireType(UInt32 tag) {
                switch (tag) {
                    case 0: return WireType.Sized;
                    default: return null;
                }
            }

            public IState GetNested(UInt32 tag) {
                switch (tag) {
                    case 0: return this.LogEntries;
                    default: return null;
                }
            }

            public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
                switch (tag) {
                    case 0: this.LogEntries = this.MaybeNotify(0, StateList<Byte>.Deserialize(reader, this.Path.GetNested(0)), this.LogEntries, OnLogEntriesUpdate, shouldNotify); break;
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
                EventHandler<FieldUpdateEventArgs<TValue, Raw>> handler,
                bool shouldNotify
            ) {
                if (shouldNotify) {
                    var args = new FieldUpdateEventArgs<TValue, Raw>(tag, newValue, oldValue, this);
                    handler?.Invoke(this, args);
                }

                return newValue;
            }
        }

        // Variant (1): Attack

        public sealed partial class Attack : IState {
            public Path Path { get; }

            public Byte Attacker { get; private set; }
            public Byte Defender { get; private set; }
            public StateList<OldHit> Hits { get; private set; }

            internal Attack(Path path = null) {
                this.Path = path ?? Path.Root;
                this.Hits = new StateList<OldHit>(this.Path.GetNested(2));
            }

            public static event EventHandler<FieldUpdateEventArgs<Byte, Attack>> OnAttackerUpdate;
            public static event EventHandler<FieldUpdateEventArgs<Byte, Attack>> OnDefenderUpdate;
            public static event EventHandler<FieldUpdateEventArgs<StateList<OldHit>, Attack>> OnHitsUpdate;

            public static void ClearAttackerUpdateHandlers() { OnAttackerUpdate = null; }
            public static void ClearDefenderUpdateHandlers() { OnDefenderUpdate = null; }
            public static void ClearHitsUpdateHandlers() { OnHitsUpdate = null; }

            public static void ClearUpdateHandlers() {
                OnAttackerUpdate = null;
                OnDefenderUpdate = null;
                OnHitsUpdate = null;
            }

            internal static Attack Deserialize(IReader reader, Path path = null) {
                var attack = new Attack(path);
                attack.Replace(reader, shouldNotify: false);
                return attack;
            }

            public WireType? GetWireType(UInt32 tag) {
                switch (tag) {
                    case 0: return WireType.Varint;
                    case 1: return WireType.Varint;
                    case 2: return WireType.Sized;
                    default: return null;
                }
            }

            public IState GetNested(UInt32 tag) {
                switch (tag) {
                    case 2: return this.Hits;
                    default: return null;
                }
            }

            public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
                switch (tag) {
                    case 0: this.Attacker = this.MaybeNotify(0, reader.ReadByte(), this.Attacker, OnAttackerUpdate, shouldNotify); break;
                    case 1: this.Defender = this.MaybeNotify(1, reader.ReadByte(), this.Defender, OnDefenderUpdate, shouldNotify); break;
                    case 2: this.Hits = this.MaybeNotify(2, StateList<OldHit>.Deserialize(reader, this.Path.GetNested(2)), this.Hits, OnHitsUpdate, shouldNotify); break;
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
                EventHandler<FieldUpdateEventArgs<TValue, Attack>> handler,
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
}
