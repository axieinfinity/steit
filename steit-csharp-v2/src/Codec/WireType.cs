using System;

namespace Steit.Codec {
    public enum WireType : Byte {
        Varint = 0,
        Sized = 2,
    }

    public static class WireTypeHelper {
        public static WireType New(UInt32 value) {
            switch (value) {
                case (byte) WireType.Varint:
                case (byte) WireType.Sized:
                    return (WireType) value;

                default:
                    throw new ArgumentException(String.Format("Invalid wire type value: {0}", value));
            }
        }
    }
}
