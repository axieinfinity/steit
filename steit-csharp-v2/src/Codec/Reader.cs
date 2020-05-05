using System;

using Steit.State;

namespace Steit.Codec {
    public static class Reader {
        // Wire type occupies three bits.
        public const int WireTypeBits = 3;

        // This mask can be applied to obtain wire type.
        public const UInt32 WireTypeMask = (1U << WireTypeBits) - 1;

        public static bool EndOfStream(this IReader reader) {
            return reader.Remaining() <= 0;
        }

        public static byte[] ReadToEnd(this IReader reader) {
            return reader.Read(reader.Remaining());
        }

        public static ulong ReadUnsignedVarint(this IReader reader) {
            ulong value = 0;

            int offset = 0;
            byte octet;

            while (true) {
                octet = reader.Read();
                value |= (ulong) (octet & 0x7f) << offset;

                if ((octet & 0x80) == 0) {
                    return value;
                }

                offset += 7;
            }
        }

        public static long ReadSignedVarint(this IReader reader) {
            long value = (long) reader.ReadUnsignedVarint();
            return (value >> 1) ^ -(value & 1);
        }

        public static Byte ReadByte(this IReader reader) { return (Byte) reader.ReadUnsignedVarint(); }
        public static UInt16 ReadUInt16(this IReader reader) { return (UInt16) reader.ReadUnsignedVarint(); }
        public static UInt32 ReadUInt32(this IReader reader) { return (UInt32) reader.ReadUnsignedVarint(); }
        public static UInt64 ReadUInt64(this IReader reader) { return reader.ReadUnsignedVarint(); }

        public static SByte ReadSByte(this IReader reader) { return (SByte) reader.ReadSignedVarint(); }
        public static Int16 ReadInt16(this IReader reader) { return (Int16) reader.ReadSignedVarint(); }
        public static Int32 ReadInt32(this IReader reader) { return (Int32) reader.ReadSignedVarint(); }
        public static Int64 ReadInt64(this IReader reader) { return reader.ReadSignedVarint(); }

        public static Boolean ReadBoolean(this IReader reader) {
            bool value = false;
            byte octet;

            while (true) {
                octet = reader.Read();
                value |= (octet & 0x7f) != 0;

                if ((octet & 0x80) == 0) {
                    return value;
                }
            }
        }

        public static (UInt32 Tag, WireType WireType) ReadKey(this IReader reader) {
            var key = reader.ReadUInt32();
            var tag = key >> WireTypeBits;
            var wireType = WireTypeHelper.New(key & WireTypeMask);
            return (tag, wireType);
        }

        public static int ReadSize(this IReader reader) {
            return (int) reader.ReadUInt32();
        }

        public static T ReadValue<T>(this IReader reader, Path path, UInt32 tag) {
            var type = typeof(T);

            switch (type.FullName) {
                case "System.Byte": return (T) (object) reader.ReadByte();
                case "System.UInt16": return (T) (object) reader.ReadUInt16();
                case "System.UInt32": return (T) (object) reader.ReadUInt32();
                case "System.UInt64": return (T) (object) reader.ReadUInt64();
                case "System.SByte": return (T) (object) reader.ReadSByte();
                case "System.Int16": return (T) (object) reader.ReadInt16();
                case "System.Int32": return (T) (object) reader.ReadInt32();
                case "System.Int64": return (T) (object) reader.ReadInt64();
                case "System.Boolean": return (T) (object) reader.ReadBoolean();

                default:
                    var method = type.GetMethod("Deserialize");

                    if (method == null) {
                        throw new MissingMethodException(String.Format("`Deserialize` method on `{0}` is missing.", type.FullName));
                    }

                    var value = (T) method.Invoke(null, new object[] { reader.GetNested(), path.GetNested(tag) });

                    if (value == null) {
                        throw new Exception(String.Format("`Deserialize` method on `{0}` returned `null`.", type.FullName));
                    }

                    return value;
            }
        }

        public static void SkipToEnd(this IReader reader) {
            reader.Skip(reader.Remaining());
        }

        public static void SkipField(this IReader reader, WireType wireType) {
            switch (wireType) {
                case WireType.Varint:
                    reader.ReadBoolean();
                    break;

                case WireType.Sized:
                    reader.Skip(reader.ReadSize());
                    break;

                default:
                    throw new NotSupportedException(String.Format("Unsupported wire type: {0}", wireType));
            }
        }

        public static IReader GetNested(this IReader reader) {
            var bytes = reader.Read(reader.ReadSize());
            return new ByteReader(bytes);
        }
    }
}
