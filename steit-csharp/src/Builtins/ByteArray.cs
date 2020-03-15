using System;

using Steit.Encoding;
using Steit.State;

namespace Steit.Builtins {
    public sealed class ByteArray : IState {
        public Path Path { get; private set; }
        public byte[] Bytes { get; private set; }

        public ByteArray(Path path = null) {
            this.Path = path != null ? path : Path.Root;
            this.Bytes = new byte[0];
        }

        public static ByteArray Deserialize(Reader reader, Path path = null, bool shouldNotify = false) {
            var byteArray = new ByteArray(path);
            var index = 0;

            byteArray.Bytes = new byte[reader.Remaining()];

            while (!reader.Eof()) {
                byteArray.Bytes[index++] = reader.Read();
            }

            return byteArray;
        }

        public Int16 WireType(UInt16 tag) {
            return -1;
        }

        public IState Nested(UInt16 tag) {
            return null;
        }

        public bool IsAddSupported() { return false; }
        public bool IsRemoveSupported() { return false; }

        public void ReplayAdd(Reader reader) { throw new Exception("Not supported"); }
        public void ReplayRemove(UInt16 tag) { throw new Exception("Not supported"); }

        public void ReplaceAt(UInt16 tag, WireType wireType, Reader reader, bool shouldNotify = true) {
            throw new Exception("Not supported");
        }
    }
}
