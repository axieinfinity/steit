using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;

using Steit.Codec;
using Steit.State;

namespace Steit.Collections {
    public sealed class Vector<T> : ReadOnlyCollection<T>, IState {
        public Path Path { get; }

        public Vector(Path? path = null, IList<T>? items = null) : base(items ?? new List<T>()) {
            Typing.CheckPrimitiveOrStateType(typeof(T));
            this.Path = path ?? Path.Root;
        }

        public static Vector<T> Deserialize(IReader reader, Path? path = null) {
            path ??= Path.Root;

            var items = new List<T>();
            var tag = 0U;

            while (!reader.EndOfStream()) {
                // Though Vector doesn't support nested states, passing tags to its children is still helpful.
                items.Add(reader.ReadValue<T>(path, tag++));
            }

            return new Vector<T>(path, items);
        }

        public WireType? GetWireType(UInt32 tag) { return null; }
        public IState? GetNested(UInt32 tag) { return null; }

        public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
            throw new NotSupportedException();
        }

        public bool IsList() { return false; }
        public void ReplayListPush(IReader reader) { throw new NotSupportedException(); }
        public void ReplayListPop() { throw new NotSupportedException(); }

        public bool IsMap() { return false; }
        public void ReplayMapInsert(IReader reader) { throw new NotSupportedException(); }
        public void ReplayMapRemove(IReader reader) { throw new NotSupportedException(); }
    }
}
