using System;
using System.Collections.Generic;

namespace Steit.State.Event {
    public sealed class MapInsertEventArgs<TValue, TMap> : EventArgs where TMap : IDictionary<UInt32, TValue>, IState {
        public UInt32 Tag { get; }
        public TValue Value { get; }
        public TMap Map { get; }

        public MapInsertEventArgs(UInt32 tag, TValue value, TMap map) {
            this.Tag = tag;
            this.Value = value;
            this.Map = map;
        }
    }
}
