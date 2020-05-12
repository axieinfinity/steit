using System;

namespace Steit.State.Event {
    public sealed class FieldUpdateEventArgs<TValue, TContainer> : EventArgs where TContainer : IState {
        public UInt32 Tag { get; }
        public TValue NewValue { get; }
        public TValue OldValue { get; }
        public TContainer Container { get; }

        public FieldUpdateEventArgs(UInt32 tag, TValue newValue, TValue oldValue, TContainer container) {
            this.Tag = tag;
            this.NewValue = newValue;
            this.OldValue = oldValue;
            this.Container = container;
        }
    }
}
