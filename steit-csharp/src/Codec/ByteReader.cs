using System;
using System.Collections.Generic;
using System.IO;

namespace Steit.Codec {
    public sealed class ByteReader : IReader {
        private readonly IReadOnlyList<Byte> bytes;
        private int offset;

        // public ByteReader(byte[]? bytes = null) : this(Array.AsReadOnly(bytes ?? new byte[0])) { }
        public ByteReader(byte[] bytes = null) : this(Array.AsReadOnly(bytes ?? new byte[0])) { }

        public ByteReader(IReadOnlyList<Byte> bytes) {
            this.bytes = bytes;
            this.offset = 0;
        }

        public int Remaining() {
            return this.bytes.Count - this.offset;
        }

        public byte Read() {
            if (this.Remaining() <= 0) {
                throw new EndOfStreamException();
            }

            return this.bytes[this.offset++];
        }

        public byte[] Read(int count) {
            if (this.Remaining() < count) {
                throw new EndOfStreamException();
            }

            var bytes = new byte[count];

            for (var i = 0; i < count; i++) {
                bytes[i] = this.bytes[this.offset++];
            }

            return bytes;
        }

        public void Skip(int count) {
            if (this.Remaining() < count) {
                throw new EndOfStreamException();
            }

            this.offset += count;
        }
    }
}
