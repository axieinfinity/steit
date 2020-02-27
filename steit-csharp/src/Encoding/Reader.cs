using System;

namespace Steit.Encoding {
    public sealed class Reader : IReader {
        private IReader reader;
        private int remaining;

        public Reader() : this(new Bytes(new byte[] {}), 0) {
        }

        public Reader(byte[] bytes) : this(new Bytes(bytes), bytes.Length) {
        }

        private Reader(IReader reader, int size) {
            this.reader = reader;
            this.remaining = size;
        }

        public Reader Nested() {
            return new Reader(this, (int) this.ReadUInt32());
        }

        public bool Eof() {
            return this.remaining <= 0 || this.reader.Eof();
        }

        public byte Read() {
            if (this.remaining > 0) {
                this.remaining--;
                return this.reader.Read();
            } else {
                throw new Exception("Unexpected EOF");
            }
        }

        public Byte ReadByte() { return (Byte) this.ReadUnsignedVarint(); }
        public UInt16 ReadUInt16() { return (UInt16) this.ReadUnsignedVarint(); }
        public UInt32 ReadUInt32() { return (UInt32) this.ReadUnsignedVarint(); }
        public UInt64 ReadUInt64() { return this.ReadUnsignedVarint(); }

        public SByte ReadSByte() { return (SByte) this.ReadSignedVarint(); }
        public Int16 ReadInt16() { return (Int16) this.ReadSignedVarint(); }
        public Int32 ReadInt32() { return (Int32) this.ReadSignedVarint(); }
        public Int64 ReadInt64() { return this.ReadSignedVarint(); }

        public Boolean ReadBoolean() {
            bool value = false;
            byte bite;

            while (true) {
                bite = this.Read();
                value |= (bite & 0x7f) != 0;

                if ((bite & 0x80) == 0) {
                    return value;
                }
            }
        }

        public (UInt16 Tag, WireType WireType) ReadKey() {
            var key = this.ReadUInt32();
            var tag = (UInt16) (key >> 3);
            var wireType = (WireType) (key & 7);
            return (tag, wireType);
        }

        public void Skip(int length) {
            if (length <= this.remaining) {
                this.reader.Skip(length);
                this.remaining -= length;
            } else {
                throw new Exception("Unexpected EOF");
            }
        }

        public void SkipWireTyped(WireType wireType) {
            switch (wireType) {
                case WireType.Varint: this.ReadBoolean(); break;
                case WireType.Sized: this.Skip((int) this.ReadUInt32()); break;
                default: throw new Exception(string.Format("Invalid wire type {0}", wireType));
            }
        }

        public void Exhaust() {
            this.reader.Skip(this.remaining);
            this.remaining = 0;
        }

        private ulong ReadUnsignedVarint() {
            ulong value = 0;

            int offset = 0;
            byte bite;

            while (true) {
                bite = this.Read();
                value |= (ulong) (bite & 0x7f) << offset;

                if ((bite & 0x80) == 0) {
                    return value;
                }

                offset += 7;
            }
        }

        private long ReadSignedVarint() {
            long value = (long) this.ReadUnsignedVarint();
            return (value >> 1) ^ -(value & 1);
        }
    }
}
