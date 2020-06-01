using System;

using Steit.Codec;
using Steit.State;

namespace Steit.Builtins {
    public abstract class Option<T> : IState {
        public Path Path { get; protected set; }

        // public static Option<T> Deserialize(IReader reader, Path? path = null) {
        public static Option<T> Deserialize(IReader reader, Path path = null) {
            path = path ?? Path.Root;

            if (!reader.EndOfStream()) {
                return new Some<T>(path, StateFactory.DeserializeNested<T>(reader, path, 0));
            } else {
                return new None<T>(path);
            }
        }

        public WireType? GetWireType(UInt32 tag) { return null; }
        // public IState? GetNested(UInt32 tag) { return null; }
        public IState GetNested(UInt32 tag) { return null; }

        public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
            throw new NotSupportedException();
        }

        public void ReplayListPush(IReader reader) { throw new NotSupportedException(); }
        public void ReplayListPop() { throw new NotSupportedException(); }
        public void ReplayMapRemove(UInt32 key) { throw new NotSupportedException(); }
    }

    public sealed class Some<T> : Option<T> {
        public T Value { get; }

        public Some(Path path, T value) {
            this.Path = path;
            this.Value = value;
        }

        public override string ToString() {
            return String.Format("Some({0})", this.Value);
        }
    }

    public sealed class None<T> : Option<T> {
        public None(Path path) {
            this.Path = path;
        }

        public override string ToString() {
            return "None";
        }
    }
}
