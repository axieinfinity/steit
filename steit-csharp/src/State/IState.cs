using System;

using Steit.Encoding;

namespace Steit.State {
    public interface IState {
        Int16 WireType(UInt16 tag);
        IState Nested(UInt16 tag);

        bool IsAddSupported();
        bool IsRemoveSupported();

        void ReplayAdd(Reader reader);
        void ReplayRemove(UInt16 tag);

        void ReplaceAt(UInt16 tag, WireType wireType, Reader reader, bool shouldNotify = true);
    }
}
