using System;

using Steit.Builtins;
using Steit.Codec;
using Steit.Collections;
using Steit.State;
using Steit.State.Event;

namespace Just.To.Test {
    public sealed partial class ActionsOr<T> : IEnumState {
        public const UInt32 ActionsTag = 0;
        public const UInt32 ValueTag = 1;

        public Path Path { get; }

        public UInt32 Tag { get; private set; }
        public IState Variant { get; private set; }

        public Actions ActionsVariant { get { return this.Variant as Actions; } }
        public Value ValueVariant { get { return this.Variant as Value; } }

        public ActionsOr(Path path = null) : this(path, 0) { }

        public ActionsOr(Path path, UInt32 tag) {
            StateFactory.ValidateType(typeof(T));
            this.Path = path ?? Path.Root;
            this.Tag = tag;

            switch (tag) {
                case 0: this.Variant = new Actions(this.Path.GetNested(0)); break;
                case 1: this.Variant = new Value(this.Path.GetNested(1)); break;
                default: this.Variant = new Actions(this.Path.GetNested(0)); break;
            }
        }

        public static ActionsOr NewActions(Path path = null) { return new ActionsOr(path, 0); }
        public static ActionsOr NewValue(Path path = null) { return new ActionsOr(path, 1); }

        public static event EventHandler<VariantUpdateEventArgs<ActionsOr<T>>> OnUpdate;

        public static void ClearUpdateHandlers() {
            OnUpdate = null;
        }

        public static ActionsOr<T> Deserialize(IReader reader, Path path = null) {
            var actionsOr = new ActionsOr<T>(path);
            actionsOr.Replace(reader, shouldNotify: false);
            return actionsOr;
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
                case 0: this.UpdateAndNotify(0, Actions.Deserialize(reader, this.Path.GetNested(0)), shouldNotify); break;
                case 1: this.UpdateAndNotify(1, Value.Deserialize(reader, this.Path.GetNested(1)), shouldNotify); break;
                default: reader.SkipToEnd(); break;
            }
        }

        public void ReplayListPush(IReader reader) { throw new NotSupportedException(); }
        public void ReplayListPop() { throw new NotSupportedException(); }
        public void ReplayMapRemove(UInt32 key) { throw new NotSupportedException(); }

        private void UpdateAndNotify(UInt32 newTag, IState newVariant, bool shouldNotify) {
            if (shouldNotify) {
                var args = new VariantUpdateEventArgs<ActionsOr<T>>(newTag, newVariant, this.Tag, this.Variant, this);
                ActionsOr<T>.OnUpdate?.Invoke(this, args);
            }

            this.Tag = newTag;
            this.Variant = newVariant;
        }

        // Variant (0): Actions

        public sealed partial class Actions : IState {
            public Path Path { get; }
            public Vector<Action> F0 { get; private set; }

            internal Actions(Path path = null) {
                this.Path = path ?? Path.Root;
                this.F0 = new Vector<Action>(this.Path.GetNested(0));
            }

            public static event EventHandler<FieldUpdateEventArgs<Vector<Action>, Actions>> OnF0Update;

            public static void ClearF0UpdateHandlers() {
                OnF0Update = null;
            }

            public static void ClearUpdateHandlers() {
                OnF0Update = null;
            }

            internal static Actions Deserialize(IReader reader, Path path = null) {
                var actions = new Actions(path);
                actions.Replace(reader, shouldNotify: false);
                return actions;
            }

            public WireType? GetWireType(UInt32 tag) {
                switch (tag) {
                    case 0: return WireType.Sized;
                    default: return null;
                }
            }

            public IState GetNested(UInt32 tag) {
                switch (tag) {
                    case 0: return this.F0;
                    default: return null;
                }
            }

            public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
                switch (tag) {
                    case 0: this.F0 = this.MaybeNotify(0, Vector<Action>.Deserialize(reader, this.Path.GetNested(0)), this.F0, OnF0Update, shouldNotify); break;
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
                EventHandler<FieldUpdateEventArgs<TValue, Actions>> handler,
                bool shouldNotify
            ) {
                if (shouldNotify) {
                    var args = new FieldUpdateEventArgs<TValue, Actions>(tag, newValue, oldValue, this);
                    handler?.Invoke(this, args);
                }

                return newValue;
            }
        }

        // Variant (1): Value

        public sealed partial class Value : IState {
            public Path Path { get; }
            public T F0 { get; private set; }

            internal Value(Path path = null) {
                this.Path = path ?? Path.Root;
                this.F0 = StateFactory.Construct<T>(this.Path.GetNested(0));
            }

            public static event EventHandler<FieldUpdateEventArgs<T, Value>> OnF0Update;

            public static void ClearF0UpdateHandlers() {
                OnF0Update = null;
            }

            public static void ClearUpdateHandlers() {
                OnF0Update = null;
            }

            internal static Value Deserialize(IReader reader, Path path = null) {
                var value = new Value(path);
                value.Replace(reader, shouldNotify: false);
                return value;
            }

            public WireType? GetWireType(UInt32 tag) {
                switch (tag) {
                    case 0: return StateFactory.IsStateType(typeof(T)) ? WireType.Sized : WireType.Varint;
                    default: return null;
                }
            }

            public IState GetNested(UInt32 tag) {
                switch (tag) {
                    case 0: return this.F0 as IState;
                    default: return null;
                }
            }

            public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
                switch (tag) {
                    case 0: this.F0 = this.MaybeNotify(0, StateFactory.Deserialize<T>(reader, this.Path, 0), this.F0, OnF0Update, shouldNotify); break;
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
                EventHandler<FieldUpdateEventArgs<TValue, Value>> handler,
                bool shouldNotify
            ) {
                if (shouldNotify) {
                    var args = new FieldUpdateEventArgs<TValue, Value>(tag, newValue, oldValue, this);
                    handler?.Invoke(this, args);
                }

                return newValue;
            }
        }
    }
}
