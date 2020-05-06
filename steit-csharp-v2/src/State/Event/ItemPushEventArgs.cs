using System;

namespace Steit.State.Event {
    public sealed class ItemPushEventArgs<TItem, TList> : EventArgs where TList : IState {
        public UInt32 NewTag { get; }
        public TItem NewItem { get; }
        public TList List { get; }

        public ItemPushEventArgs(UInt32 newTag, TItem newItem, TList list) {
            this.NewTag = newTag;
            this.NewItem = newItem;
            this.List = list;
        }
    }
}
