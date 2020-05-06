using System;

namespace Steit.State.Event {
    public sealed class ItemPopEventArgs<TItem, TList> : EventArgs where TList : IState {
        public UInt32 OldTag { get; }
        public TItem OldItem { get; }
        public TList List { get; }

        public ItemPopEventArgs(UInt32 oldTag, TItem oldItem, TList list) {
            this.OldTag = oldTag;
            this.OldItem = oldItem;
            this.List = list;
        }
    }
}
