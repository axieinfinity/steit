using System;

namespace Steit.State {
    public interface IEnumState : IState {
        UInt32 Tag { get; }
        IState Variant { get; }
    }
}
