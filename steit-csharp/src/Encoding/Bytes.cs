using System;

namespace Steit.Encoding {
    public sealed class Bytes : IReader {
        private readonly byte[] bytes;
        private int index;

        public Bytes(byte[] bytes) {
            this.bytes = bytes;
            this.index = 0;
        }

        public bool Eof() {
            return this.index >= this.bytes.Length;
        }

        public byte Read() {
            if (this.index < this.bytes.Length) {
                return this.bytes[this.index++];
            } else {
                throw new Exception("Unexpected EOF");
            }
        }

        public void Skip(int length) {
            if (this.index + length <= this.bytes.Length) {
                this.index += length;
            } else {
                throw new Exception("Unexpected EOF");
            }
        }
    }
}
