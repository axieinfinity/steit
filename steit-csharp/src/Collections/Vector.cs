using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;

using Steit.Codec;
using Steit.State;

namespace Steit.Collections {
    public sealed class Vector<T> : ReadOnlyCollection<T>, IState {
        public Path Path { get; }

        // public Vector(Path? path = null, IList<T>? items = null) : base(items ?? new List<T>()) {
        public Vector(Path path = null, IList<T> items = null) : base(items ?? new List<T>()) {
            StateFactory.ValidateType(typeof(T));
            this.Path = path ?? Path.Root;
        }

        // public static Vector<T> Deserialize(IReader reader, Path? path = null) {
        public static Vector<T> Deserialize(IReader reader, Path path = null) {
            // path ??= Path.Root;
            path = path ?? Path.Root;

            var items = new List<T>();
            var tag = 0U;

            while (!reader.EndOfStream()) {
                // Though Vector doesn't support nested states, passing tags to its children is still helpful.
                items.Add(reader.ReadValue<T>(path, tag++));
            }

            return new Vector<T>(path, items);
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
}
