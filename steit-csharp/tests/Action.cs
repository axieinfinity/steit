using System;

using Steit.Builtins;
using Steit.Codec;
using Steit.Collections;
using Steit.State;
using Steit.State.Event;

namespace Just.To.Test {
    public sealed partial class Action : IEnumState {
        public const UInt32 RawTag = 0;
        public const UInt32 CardDrawTag = 1;
        public const UInt32 CardDiscardTag = 2;
        public const UInt32 AttackTag = 3;
        public const UInt32 SkillTag = 4;

        public Path Path { get; }

        public UInt32 Tag { get; private set; }
        public IState Variant { get; private set; }

        public Raw RawVariant { get { return this.Variant as Raw; } }
        public CardDraw CardDrawVariant { get { return this.Variant as CardDraw; } }
        public CardDiscard CardDiscardVariant { get { return this.Variant as CardDiscard; } }
        public Attack AttackVariant { get { return this.Variant as Attack; } }
        public Skill SkillVariant { get { return this.Variant as Skill; } }

        public Action(Path path = null) : this(path, 0) { }

        public Action(Path path, UInt32 tag) {
            this.Path = path ?? Path.Root;
            this.Tag = tag;

            switch (tag) {
                case 0: this.Variant = new Raw(this.Path.GetNested(0)); break;
                case 1: this.Variant = new CardDraw(this.Path.GetNested(1)); break;
                case 2: this.Variant = new CardDiscard(this.Path.GetNested(2)); break;
                case 3: this.Variant = new Attack(this.Path.GetNested(3)); break;
                case 4: this.Variant = new Skill(this.Path.GetNested(4)); break;
                default: this.Variant = new Raw(this.Path.GetNested(0)); break;
            }
        }

        public static Action NewRaw(Path path = null) { return new Action(path, 0); }
        public static Action NewCardDraw(Path path = null) { return new Action(path, 1); }
        public static Action NewCardDiscard(Path path = null) { return new Action(path, 2); }
        public static Action NewAttack(Path path = null) { return new Action(path, 3); }
        public static Action NewSkill(Path path = null) { return new Action(path, 4); }

        public static event EventHandler<VariantUpdateEventArgs<Action>> OnUpdate;

        public static void ClearUpdateHandlers() {
            OnUpdate = null;
        }

        public static Action Deserialize(IReader reader, Path path = null) {
            var action = new Action(path);
            action.Replace(reader, shouldNotify: false);
            return action;
        }

        public WireType? GetWireType(UInt32 tag) {
            switch (tag) {
                case 0: return WireType.Sized;
                case 1: return WireType.Sized;
                case 2: return WireType.Sized;
                case 3: return WireType.Sized;
                case 4: return WireType.Sized;
                default: return null;
            }
        }

        public IState GetNested(UInt32 tag) {
            return tag == this.Tag ? this.Variant : null;
        }

        public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
            switch (tag) {
                case 0: this.UpdateAndNotify(0, Raw.Deserialize(reader, this.Path.GetNested(0)), shouldNotify); break;
                case 1: this.UpdateAndNotify(1, CardDraw.Deserialize(reader, this.Path.GetNested(1)), shouldNotify); break;
                case 2: this.UpdateAndNotify(2, CardDiscard.Deserialize(reader, this.Path.GetNested(2)), shouldNotify); break;
                case 3: this.UpdateAndNotify(3, Attack.Deserialize(reader, this.Path.GetNested(3)), shouldNotify); break;
                case 4: this.UpdateAndNotify(4, Skill.Deserialize(reader, this.Path.GetNested(4)), shouldNotify); break;
                default: reader.SkipToEnd(); break;
            }
        }

        public void ReplayListPush(IReader reader) { throw new NotSupportedException(); }
        public void ReplayListPop() { throw new NotSupportedException(); }
        public void ReplayMapRemove(UInt32 key) { throw new NotSupportedException(); }

        private void UpdateAndNotify(UInt32 newTag, IState newVariant, bool shouldNotify) {
            if (shouldNotify) {
                var args = new VariantUpdateEventArgs<Action>(newTag, newVariant, this.Tag, this.Variant, this);
                Action.OnUpdate?.Invoke(this, args);
            }

            this.Tag = newTag;
            this.Variant = newVariant;
        }

        // Variant (0): Raw

        public sealed partial class Raw : IState {
            public Path Path { get; }

            internal Raw(Path path = null) {
                this.Path = path ?? Path.Root;
            }

            public static void ClearUpdateHandlers() { }

            internal static Raw Deserialize(IReader reader, Path path = null) {
                var raw = new Raw(path);
                raw.Replace(reader, shouldNotify: false);
                return raw;
            }

            public WireType? GetWireType(UInt32 tag) {
                switch (tag) {
                    default: return null;
                }
            }

            public IState GetNested(UInt32 tag) {
                switch (tag) {
                    default: return null;
                }
            }

            public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
                switch (tag) {
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

        // Variant (1): CardDraw

        public sealed partial class CardDraw : IState {
            public Path Path { get; }

            public UInt16 PlayerIndex { get; private set; }
            public Vector<Action> Draw { get; private set; }
            public Vector<Action> PostDraw { get; private set; }

            internal CardDraw(Path path = null) {
                this.Path = path ?? Path.Root;
                this.Draw = new Vector<Action>(this.Path.GetNested(1));
                this.PostDraw = new Vector<Action>(this.Path.GetNested(2));
            }

            public static event EventHandler<FieldUpdateEventArgs<UInt16, CardDraw>> OnPlayerIndexUpdate;
            public static event EventHandler<FieldUpdateEventArgs<Vector<Action>, CardDraw>> OnDrawUpdate;
            public static event EventHandler<FieldUpdateEventArgs<Vector<Action>, CardDraw>> OnPostDrawUpdate;

            public static void ClearPlayerIndexUpdateHandlers() { OnPlayerIndexUpdate = null; }
            public static void ClearDrawUpdateHandlers() { OnDrawUpdate = null; }
            public static void ClearPostDrawUpdateHandlers() { OnPostDrawUpdate = null; }

            public static void ClearUpdateHandlers() {
                OnPlayerIndexUpdate = null;
                OnDrawUpdate = null;
                OnPostDrawUpdate = null;
            }

            internal static CardDraw Deserialize(IReader reader, Path path = null) {
                var cardDraw = new CardDraw(path);
                cardDraw.Replace(reader, shouldNotify: false);
                return cardDraw;
            }

            public WireType? GetWireType(UInt32 tag) {
                switch (tag) {
                    case 0: return WireType.Varint;
                    case 1: return WireType.Sized;
                    case 2: return WireType.Sized;
                    default: return null;
                }
            }

            public IState GetNested(UInt32 tag) {
                switch (tag) {
                    case 1: return this.Draw;
                    case 2: return this.PostDraw;
                    default: return null;
                }
            }

            public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
                switch (tag) {
                    case 0: this.PlayerIndex = this.MaybeNotify(0, reader.ReadUInt16(), this.PlayerIndex, OnPlayerIndexUpdate, shouldNotify); break;
                    case 1: this.Draw = this.MaybeNotify(1, Vector<Action>.Deserialize(reader, this.Path.GetNested(1)), this.Draw, OnDrawUpdate, shouldNotify); break;
                    case 2: this.PostDraw = this.MaybeNotify(2, Vector<Action>.Deserialize(reader, this.Path.GetNested(2)), this.PostDraw, OnPostDrawUpdate, shouldNotify); break;
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
                EventHandler<FieldUpdateEventArgs<TValue, CardDraw>> handler,
                bool shouldNotify
            ) {
                if (shouldNotify) {
                    var args = new FieldUpdateEventArgs<TValue, CardDraw>(tag, newValue, oldValue, this);
                    handler?.Invoke(this, args);
                }

                return newValue;
            }
        }

        // Variant (2): CardDiscard

        public sealed partial class CardDiscard : IState {
            public Path Path { get; }

            internal CardDiscard(Path path = null) {
                this.Path = path ?? Path.Root;
            }

            public static void ClearUpdateHandlers() { }

            internal static CardDiscard Deserialize(IReader reader, Path path = null) {
                var cardDiscard = new CardDiscard(path);
                cardDiscard.Replace(reader, shouldNotify: false);
                return cardDiscard;
            }

            public WireType? GetWireType(UInt32 tag) {
                switch (tag) {
                    default: return null;
                }
            }

            public IState GetNested(UInt32 tag) {
                switch (tag) {
                    default: return null;
                }
            }

            public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
                switch (tag) {
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
                EventHandler<FieldUpdateEventArgs<TValue, CardDiscard>> handler,
                bool shouldNotify
            ) {
                if (shouldNotify) {
                    var args = new FieldUpdateEventArgs<TValue, CardDiscard>(tag, newValue, oldValue, this);
                    handler?.Invoke(this, args);
                }

                return newValue;
            }
        }

        // Variant (3): Attack

        public sealed partial class Attack : IState {
            public Path Path { get; }

            public UInt16 AttackerIndex { get; private set; }
            public UInt32 CardId { get; private set; }
            public Vector<Action> BeforeAttacks { get; private set; }
            public ActionsOr<Vector<ActionsOr<Attack>>> Attacks { get; private set; }
            public Vector<Action> AfterAttacks { get; private set; }

            internal Attack(Path path = null) {
                this.Path = path ?? Path.Root;
                this.BeforeAttacks = new Vector<Action>(this.Path.GetNested(2));
                this.Attacks = new ActionsOr<Vector<ActionsOr<Attack>>>(this.Path.GetNested(3));
                this.AfterAttacks = new Vector<Action>(this.Path.GetNested(4));
            }

            public static event EventHandler<FieldUpdateEventArgs<UInt16, Attack>> OnAttackerIndexUpdate;
            public static event EventHandler<FieldUpdateEventArgs<UInt32, Attack>> OnCardIdUpdate;
            public static event EventHandler<FieldUpdateEventArgs<Vector<Action>, Attack>> OnBeforeAttacksUpdate;
            public static event EventHandler<FieldUpdateEventArgs<ActionsOr<Vector<ActionsOr<Attack>>>, Attack>> OnAttacksUpdate;
            public static event EventHandler<FieldUpdateEventArgs<Vector<Action>, Attack>> OnAfterAttacksUpdate;

            public static void ClearAttackerIndexUpdateHandlers() { OnAttackerIndexUpdate = null; }
            public static void ClearCardIdUpdateHandlers() { OnCardIdUpdate = null; }
            public static void ClearBeforeAttacksUpdateHandlers() { OnBeforeAttacksUpdate = null; }
            public static void ClearAttacksUpdateHandlers() { OnAttacksUpdate = null; }
            public static void ClearAfterAttacksUpdateHandlers() { OnAfterAttacksUpdate = null; }

            public static void ClearUpdateHandlers() {
                OnAttackerIndexUpdate = null;
                OnCardIdUpdate = null;
                OnBeforeAttacksUpdate = null;
                OnAttacksUpdate = null;
                OnAfterAttacksUpdate = null;
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
                    case 3: return WireType.Sized;
                    case 4: return WireType.Sized;
                    default: return null;
                }
            }

            public IState GetNested(UInt32 tag) {
                switch (tag) {
                    case 2: return this.BeforeAttacks;
                    case 3: return this.Attacks;
                    case 4: return this.AfterAttacks;
                    default: return null;
                }
            }

            public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
                switch (tag) {
                    case 0: this.AttackerIndex = this.MaybeNotify(0, reader.ReadUInt16(), this.AttackerIndex, OnAttackerIndexUpdate, shouldNotify); break;
                    case 1: this.CardId = this.MaybeNotify(1, reader.ReadUInt32(), this.CardId, OnCardIdUpdate, shouldNotify); break;
                    case 2: this.BeforeAttacks = this.MaybeNotify(2, Vector<Action>.Deserialize(reader, this.Path.GetNested(2)), this.BeforeAttacks, OnBeforeAttacksUpdate, shouldNotify); break;
                    case 3: this.Attacks = this.MaybeNotify(3, ActionsOr<Vector<ActionsOr<Attack>>>.Deserialize(reader, this.Path.GetNested(3)), this.Attacks, OnAttacksUpdate, shouldNotify); break;
                    case 4: this.AfterAttacks = this.MaybeNotify(4, Vector<Action>.Deserialize(reader, this.Path.GetNested(4)), this.AfterAttacks, OnAfterAttacksUpdate, shouldNotify); break;
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

        // Variant (4): Skill

        public sealed partial class Skill : IState {
            public Path Path { get; }

            public UInt16 CasterIndex { get; private set; }
            public UInt32 CardId { get; private set; }
            public Vector<Action> BeforeSkills { get; private set; }
            public ActionsOr<Vector<ActionsOr<Skill>>> Skills { get; private set; }
            public Vector<Action> AfterSkills { get; private set; }

            internal Skill(Path path = null) {
                this.Path = path ?? Path.Root;
                this.BeforeSkills = new Vector<Action>(this.Path.GetNested(2));
                this.Skills = new ActionsOr<Vector<ActionsOr<Skill>>>(this.Path.GetNested(3));
                this.AfterSkills = new Vector<Action>(this.Path.GetNested(4));
            }

            public static event EventHandler<FieldUpdateEventArgs<UInt16, Skill>> OnCasterIndexUpdate;
            public static event EventHandler<FieldUpdateEventArgs<UInt32, Skill>> OnCardIdUpdate;
            public static event EventHandler<FieldUpdateEventArgs<Vector<Action>, Skill>> OnBeforeSkillsUpdate;
            public static event EventHandler<FieldUpdateEventArgs<ActionsOr<Vector<ActionsOr<Skill>>>, Skill>> OnSkillsUpdate;
            public static event EventHandler<FieldUpdateEventArgs<Vector<Action>, Skill>> OnAfterSkillsUpdate;

            public static void ClearCasterIndexUpdateHandlers() { OnCasterIndexUpdate = null; }
            public static void ClearCardIdUpdateHandlers() { OnCardIdUpdate = null; }
            public static void ClearBeforeSkillsUpdateHandlers() { OnBeforeSkillsUpdate = null; }
            public static void ClearSkillsUpdateHandlers() { OnSkillsUpdate = null; }
            public static void ClearAfterSkillsUpdateHandlers() { OnAfterSkillsUpdate = null; }

            public static void ClearUpdateHandlers() {
                OnCasterIndexUpdate = null;
                OnCardIdUpdate = null;
                OnBeforeSkillsUpdate = null;
                OnSkillsUpdate = null;
                OnAfterSkillsUpdate = null;
            }

            internal static Skill Deserialize(IReader reader, Path path = null) {
                var skill = new Skill(path);
                skill.Replace(reader, shouldNotify: false);
                return skill;
            }

            public WireType? GetWireType(UInt32 tag) {
                switch (tag) {
                    case 0: return WireType.Varint;
                    case 1: return WireType.Varint;
                    case 2: return WireType.Sized;
                    case 3: return WireType.Sized;
                    case 4: return WireType.Sized;
                    default: return null;
                }
            }

            public IState GetNested(UInt32 tag) {
                switch (tag) {
                    case 2: return this.BeforeSkills;
                    case 3: return this.Skills;
                    case 4: return this.AfterSkills;
                    default: return null;
                }
            }

            public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
                switch (tag) {
                    case 0: this.CasterIndex = this.MaybeNotify(0, reader.ReadUInt16(), this.CasterIndex, OnCasterIndexUpdate, shouldNotify); break;
                    case 1: this.CardId = this.MaybeNotify(1, reader.ReadUInt32(), this.CardId, OnCardIdUpdate, shouldNotify); break;
                    case 2: this.BeforeSkills = this.MaybeNotify(2, Vector<Action>.Deserialize(reader, this.Path.GetNested(2)), this.BeforeSkills, OnBeforeSkillsUpdate, shouldNotify); break;
                    case 3: this.Skills = this.MaybeNotify(3, ActionsOr<Vector<ActionsOr<Skill>>>.Deserialize(reader, this.Path.GetNested(3)), this.Skills, OnSkillsUpdate, shouldNotify); break;
                    case 4: this.AfterSkills = this.MaybeNotify(4, Vector<Action>.Deserialize(reader, this.Path.GetNested(4)), this.AfterSkills, OnAfterSkillsUpdate, shouldNotify); break;
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
                EventHandler<FieldUpdateEventArgs<TValue, Skill>> handler,
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
}
