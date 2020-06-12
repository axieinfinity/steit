using System;

using Steit.Builtins;
using Steit.Codec;
using Steit.Collections;
using Steit.State;
using Steit.State.Event;

namespace Steit.Builtins {
    public sealed partial class Maybe<T> : IEnumState {
        public const UInt32 NoneTag = 0;
        public const UInt32 SomeTag = 1;

        public Path Path { get; }

        public UInt32 Tag { get; private set; }
        public IState Variant { get; private set; }

        public None NoneVariant { get { return this.Variant as None; } }
        public Some SomeVariant { get { return this.Variant as Some; } }

        public Maybe(Path path = null) {
            StateFactory.ValidateType(typeof(T));
            this.Path = path ?? Path.Root;
            this.Tag = 0;
            this.Variant = new None(this.Path.GetNested(0));
        }

        public static event EventHandler<VariantUpdateEventArgs<Maybe<T>>> OnUpdate;

        public static void ClearUpdateHandlers() {
            OnUpdate = null;
        }

        public static Maybe<T> Deserialize(IReader reader, Path path = null) {
            var maybe = new Maybe<T>(path);
            maybe.Replace(reader, shouldNotify: false);
            return maybe;
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
                case 0: this.UpdateAndNotify(0, None.Deserialize(reader, this.Path.GetNested(0)), shouldNotify); break;
                case 1: this.UpdateAndNotify(1, Some.Deserialize(reader, this.Path.GetNested(1)), shouldNotify); break;
                default: reader.SkipToEnd(); break;
            }
        }

        public void ReplayListPush(IReader reader) { throw new NotSupportedException(); }
        public void ReplayListPop() { throw new NotSupportedException(); }
        public void ReplayMapRemove(UInt32 key) { throw new NotSupportedException(); }

        private void UpdateAndNotify(UInt32 newTag, IState newVariant, bool shouldNotify) {
            if (shouldNotify) {
                var args = new VariantUpdateEventArgs<Maybe<T>>(newTag, newVariant, this.Tag, this.Variant, this);
                Maybe<T>.OnUpdate?.Invoke(this, args);
            }

            this.Tag = newTag;
            this.Variant = newVariant;
        }

        // Variant (0): None

        public sealed partial class None : IState {
            public Path Path { get; }

            internal None(Path path = null) {
                this.Path = path ?? Path.Root;
            }

            public static void ClearUpdateHandlers() { }

            internal static None Deserialize(IReader reader, Path path = null) {
                var none = new None(path);
                none.Replace(reader, shouldNotify: false);
                return none;
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
                EventHandler<FieldUpdateEventArgs<TValue, None>> handler,
                bool shouldNotify
            ) {
                if (shouldNotify) {
                    var args = new FieldUpdateEventArgs<TValue, None>(tag, newValue, oldValue, this);
                    handler?.Invoke(this, args);
                }

                return newValue;
            }
        }

        // Variant (1): Some

        public sealed partial class Some : IState {
            public Path Path { get; }
            public T F0 { get; private set; }

            internal Some(Path path = null) {
                this.Path = path ?? Path.Root;
                this.F0 = StateFactory.Construct<T>(this.Path.GetNested(0));
            }

            public static event EventHandler<FieldUpdateEventArgs<T, Some>> OnF0Update;

            public static void ClearF0UpdateHandlers() {
                OnF0Update = null;
            }

            public static void ClearUpdateHandlers() {
                OnF0Update = null;
            }

            internal static Some Deserialize(IReader reader, Path path = null) {
                var some = new Some(path);
                some.Replace(reader, shouldNotify: false);
                return some;
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
                EventHandler<FieldUpdateEventArgs<TValue, Some>> handler,
                bool shouldNotify
            ) {
                if (shouldNotify) {
                    var args = new FieldUpdateEventArgs<TValue, Some>(tag, newValue, oldValue, this);
                    handler?.Invoke(this, args);
                }

                return newValue;
            }
        }
    }
}
