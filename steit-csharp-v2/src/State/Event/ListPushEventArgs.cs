using System;
using System.Collections.Generic;

namespace Steit.State.Event {
    public sealed class ListPushEventArgs<TItem, TList> : EventArgs where TList : IList<TItem>, IState {
        public UInt32 Tag { get; }
        public TItem Item { get; }
        public TList List { get; }

        public ListPushEventArgs(UInt32 newTag, TItem newItem, TList list) {
            this.Tag = newTag;
            this.Item = newItem;
            this.List = list;
        }
    }
}
