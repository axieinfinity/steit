using System;

using Steit.Builtins;
using Steit.Codec;
using Steit.Collections;
using Steit.State;
using Steit.State.Event;

namespace Just.To.Test {
    public sealed partial class Hello : IState {
        public Path Path { get; }

        public StateList<Int32> Numbers { get; private set; }
        public Vector<Int32> Others { get; private set; }

        public Hello(Path path = null) {
            this.Path = path ?? Path.Root;
            this.Numbers = new StateList<Int32>(this.Path.GetNested(0));
            this.Others = new Vector<Int32>(this.Path.GetNested(1));
        }

        public static event EventHandler<FieldUpdateEventArgs<StateList<Int32>, Hello>> OnNumbersUpdate;
        public static event EventHandler<FieldUpdateEventArgs<Vector<Int32>, Hello>> OnOthersUpdate;

        public static void ClearNumbersUpdateHandlers() { OnNumbersUpdate = null; }
        public static void ClearOthersUpdateHandlers() { OnOthersUpdate = null; }

        public static void ClearUpdateHandlers() {
            OnNumbersUpdate = null;
            OnOthersUpdate = null;
        }

        public static Hello Deserialize(IReader reader, Path path = null) {
            var hello = new Hello(path);
            hello.Replace(reader, shouldNotify: false);
            return hello;
        }

        public WireType? GetWireType(UInt32 tag) {
            switch (tag) {
                case 0: return WireType.Sized;
                case 1: return WireType.Sized;
                default: return null;
            }
        }

        public IState GetNested(UInt32 tag) {
            switch (tag) {
                case 0: return this.Numbers;
                case 1: return this.Others;
                default: return null;
            }
        }

        public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
            switch (tag) {
                case 0: this.Numbers = this.MaybeNotify(0, StateList<Int32>.Deserialize(reader, this.Path.GetNested(0)), this.Numbers, OnNumbersUpdate, shouldNotify); break;
                case 1: this.Others = this.MaybeNotify(1, Vector<Int32>.Deserialize(reader, this.Path.GetNested(1)), this.Others, OnOthersUpdate, shouldNotify); break;
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
            EventHandler<FieldUpdateEventArgs<TValue, Hello>> handler,
            bool shouldNotify
        ) {
            if (shouldNotify) {
                var args = new FieldUpdateEventArgs<TValue, Hello>(tag, newValue, oldValue, this);
                handler?.Invoke(this, args);
            }

            return newValue;
        }
    }
}
