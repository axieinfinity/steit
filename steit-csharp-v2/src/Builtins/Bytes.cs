using System;
using System.Collections.ObjectModel;

using Steit.Codec;
using Steit.State;

namespace Steit.Builtins {
    public sealed class Bytes : ReadOnlyCollection<Byte>, IState {
        public Path Path { get; }

        public Bytes(Path? path = null, byte[]? bytes = null) : base(bytes ?? new byte[0]) {
            this.Path = path ?? Path.Root;
        }

        public static Bytes Deserialize(IReader reader, Path? path = null) {
            return new Bytes(path, reader.ReadToEnd());
        }

        public WireType? GetWireType(UInt32 tag) { return null; }
        public IState? GetNested(UInt32 tag) { return null; }

        public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
            throw new NotSupportedException();
        }

        public bool IsList() { return false; }
        public void ReplayListPush(IReader itemReader) { throw new NotSupportedException(); }
        public void ReplayListPop() { throw new NotSupportedException(); }

        public bool IsMap() { return false; }
        public void ReplayMapInsert(IReader keyReader, IReader valueReader) { throw new NotSupportedException(); }
        public void ReplayMapRemove(IReader keyReader) { throw new NotSupportedException(); }
    }
}
