using System;
using System.Collections.Generic;

using Steit.Encoding;

namespace Steit.State {
    public static class State {
        public static T ReadNested<T>(this IState state, Reader reader, UInt16 tag) {
            var type = typeof(T);

            switch (type.FullName) {
                case "System.Byte": return (T) (object) reader.ReadUInt8();
                case "System.UInt16": return (T) (object) reader.ReadUInt16();
                case "System.UInt32": return (T) (object) reader.ReadUInt32();
                case "System.UInt64": return (T) (object) reader.ReadUInt64();
                case "System.SByte": return (T) (object) reader.ReadInt8();
                case "System.Int16": return (T) (object) reader.ReadInt16();
                case "System.Int32": return (T) (object) reader.ReadInt32();
                case "System.Int64": return (T) (object) reader.ReadInt64();
                case "System.Boolean": return (T) (object) reader.ReadBoolean();

                default:
                    var method = type.GetMethod("Deserialize");
                    var path = state.Path.Nested(tag);
                    var arguments = new object[] { reader, path, /* shouldNotify: */ true };
                    return (T) method.Invoke(null, arguments);
            }
        }

        public static IState Nested(this IState state, IList<UInt16> path) {
            foreach (var tag in path) {
                if (state != null) {
                    state = state.Nested(tag);
                }
            }

            return state;
        }

        public static void Replace(this IState state, Reader reader, bool shouldNotify) {
            var (tag, wireType) = reader.ReadKey();
            var expectedWireType = state.WireType(tag);

            if (expectedWireType >= 0 && wireType != (WireType) expectedWireType) {
                Console.Error.WriteLine("Unexpected tag {0} or wire type {1}", tag, wireType);
                reader.SkipWireTyped(wireType);
                return;
            }

            state.ReplaceAt(tag, wireType, reader, shouldNotify);
        }

        public static void Replace(this IEnumState state, Reader reader, bool shouldNotify) {
            state.ReplaceAt(reader.ReadUInt16(), WireType.Sized, reader, shouldNotify);
        }

        public static void ReplaceAll(this IState state, Reader reader, bool shouldNotify) {
            while (!reader.Eof()) {
                state.Replace(reader, shouldNotify);
            }
        }

        public static void ReplaceAll(this IEnumState state, Reader reader, bool shouldNotify) {
            state.Replace(reader, shouldNotify);
        }
    }
}
