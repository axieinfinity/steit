using System;

using Steit.Codec;
using Steit.State;

namespace Steit.Builtins {
    public sealed class Option<T> : IState {
        public Path Path { get; }

        public bool IsSome { get; }
        public bool IsNone { get { return !this.IsSome; } }

        public T ValueOrDefault { get; }

        public Option(Path path = null) {
            this.Path = path ?? Path.Root;
        }

        public Option(Path path, T value) : this(path) {
            this.IsSome = true;
            this.ValueOrDefault = value;
        }

        public static Option<T> None(Path path = null) {
            return new Option<T>(path);
        }

        public static Option<T> Some(Path path, T value) {
            return new Option<T>(path, value);
        }

        public static Option<T> Deserialize(IReader reader, Path path = null) {
            if (!reader.EndOfStream()) {
                return Some(path, StateFactory.DeserializeNested<T>(reader, path, 0));
            } else {
                return None(path);
            }
        }

        public WireType? GetWireType(UInt32 tag) { return null; }
        public IState GetNested(UInt32 tag) { return null; }

        public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
            throw new NotSupportedException();
        }

        public void ReplayListPush(IReader reader) { throw new NotSupportedException(); }
        public void ReplayListPop() { throw new NotSupportedException(); }
        public void ReplayMapRemove(UInt32 key) { throw new NotSupportedException(); }

        public override string ToString() {
            if (this.IsSome) {
                return String.Format("Some({0})", this.ValueOrDefault);
            } else {
                return "None";
            }
        }
    }
}
