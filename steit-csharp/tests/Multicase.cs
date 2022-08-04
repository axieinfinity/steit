using System;

using Steit.Builtins;
using Steit.Codec;
using Steit.Collections;
using Steit.State;
using Steit.State.Event;

namespace Just.To.Test {
    public sealed partial class Multicase : IEnumState {
        public const UInt32 FirstCaseTag = 0;
        public const UInt32 SecondCaseTag = 1;

        public Path Path { get; }

        public UInt32 Tag { get; private set; }
        public IState Variant { get; private set; }

        public FirstCase FirstCaseVariant { get { return this.Variant as FirstCase; } }
        public SecondCase SecondCaseVariant { get { return this.Variant as SecondCase; } }

        public Multicase(Path path = null) : this(path, 0) { }

        public Multicase(Path path, UInt32 tag) {
            this.Path = path ?? Path.Root;
            this.Tag = tag;

            switch (tag) {
                case 0: this.Variant = new FirstCase(this.Path.GetNested(0)); break;
                case 1: this.Variant = new SecondCase(this.Path.GetNested(1)); break;
                default: this.Variant = new FirstCase(this.Path.GetNested(0)); break;
            }
        }

        public static Multicase NewFirstCase(Path path = null) { return new Multicase(path, 0); }
        public static Multicase NewSecondCase(Path path = null) { return new Multicase(path, 1); }

        public static event EventHandler<VariantUpdateEventArgs<Multicase>> OnUpdate;

        public static void ClearUpdateHandlers() {
            OnUpdate = null;
        }

        public static Multicase Deserialize(IReader reader, Path path = null) {
            var multicase = new Multicase(path);
            multicase.Replace(reader, shouldNotify: false);
            return multicase;
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
                case 0: this.UpdateAndNotify(0, FirstCase.Deserialize(reader, this.Path.GetNested(0)), shouldNotify); break;
                case 1: this.UpdateAndNotify(1, SecondCase.Deserialize(reader, this.Path.GetNested(1)), shouldNotify); break;
                default: reader.SkipToEnd(); break;
            }
        }

        public void ReplayListPush(IReader reader) { throw new NotSupportedException(); }
        public void ReplayListPop() { throw new NotSupportedException(); }
        public void ReplayMapRemove(UInt32 key) { throw new NotSupportedException(); }

        private void UpdateAndNotify(UInt32 newTag, IState newVariant, bool shouldNotify) {
            if (shouldNotify) {
                var args = new VariantUpdateEventArgs<Multicase>(newTag, newVariant, this.Tag, this.Variant, this);
                Multicase.OnUpdate?.Invoke(this, args);
            }

            this.Tag = newTag;
            this.Variant = newVariant;
        }

        // Variant (0): FirstCase

        public sealed partial class FirstCase : IState {
            public Path Path { get; }

            public Int32 Counter { get; private set; }
            public Boolean Enabled { get; private set; }

            internal FirstCase(Path path = null) {
                this.Path = path ?? Path.Root;
            }

            public static event EventHandler<FieldUpdateEventArgs<Int32, FirstCase>> OnCounterUpdate;
            public static event EventHandler<FieldUpdateEventArgs<Boolean, FirstCase>> OnEnabledUpdate;

            public static void ClearCounterUpdateHandlers() { OnCounterUpdate = null; }
            public static void ClearEnabledUpdateHandlers() { OnEnabledUpdate = null; }

            public static void ClearUpdateHandlers() {
                OnCounterUpdate = null;
                OnEnabledUpdate = null;
            }

            internal static FirstCase Deserialize(IReader reader, Path path = null) {
                var firstCase = new FirstCase(path);
                firstCase.Replace(reader, shouldNotify: false);
                return firstCase;
            }

            public WireType? GetWireType(UInt32 tag) {
                switch (tag) {
                    case 0: return WireType.Varint;
                    case 1: return WireType.Varint;
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
                    case 0: this.Counter = this.MaybeNotify(0, reader.ReadInt32(), this.Counter, OnCounterUpdate, shouldNotify); break;
                    case 1: this.Enabled = this.MaybeNotify(1, reader.ReadBoolean(), this.Enabled, OnEnabledUpdate, shouldNotify); break;
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
                EventHandler<FieldUpdateEventArgs<TValue, FirstCase>> handler,
                bool shouldNotify
            ) {
                if (shouldNotify) {
                    var args = new FieldUpdateEventArgs<TValue, FirstCase>(tag, newValue, oldValue, this);
                    handler?.Invoke(this, args);
                }

                return newValue;
            }
        }

        // Variant (1): SecondCase

        public sealed partial class SecondCase : IState {
            public Path Path { get; }

            public Int32 Counter { get; private set; }
            public Boolean Enabled { get; private set; }

            internal SecondCase(Path path = null) {
                this.Path = path ?? Path.Root;
            }

            public static event EventHandler<FieldUpdateEventArgs<Int32, SecondCase>> OnCounterUpdate;
            public static event EventHandler<FieldUpdateEventArgs<Boolean, SecondCase>> OnEnabledUpdate;

            public static void ClearCounterUpdateHandlers() { OnCounterUpdate = null; }
            public static void ClearEnabledUpdateHandlers() { OnEnabledUpdate = null; }

            public static void ClearUpdateHandlers() {
                OnCounterUpdate = null;
                OnEnabledUpdate = null;
            }

            internal static SecondCase Deserialize(IReader reader, Path path = null) {
                var secondCase = new SecondCase(path);
                secondCase.Replace(reader, shouldNotify: false);
                return secondCase;
            }

            public WireType? GetWireType(UInt32 tag) {
                switch (tag) {
                    case 0: return WireType.Varint;
                    case 1: return WireType.Varint;
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
                    case 0: this.Counter = this.MaybeNotify(0, reader.ReadInt32(), this.Counter, OnCounterUpdate, shouldNotify); break;
                    case 1: this.Enabled = this.MaybeNotify(1, reader.ReadBoolean(), this.Enabled, OnEnabledUpdate, shouldNotify); break;
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
                EventHandler<FieldUpdateEventArgs<TValue, SecondCase>> handler,
                bool shouldNotify
            ) {
                if (shouldNotify) {
                    var args = new FieldUpdateEventArgs<TValue, SecondCase>(tag, newValue, oldValue, this);
                    handler?.Invoke(this, args);
                }

                return newValue;
            }
        }
    }
}
