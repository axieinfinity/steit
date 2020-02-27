using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;

using Steit.Encoding;
using Steit.State;

namespace Steit.Collections {
    public sealed class FixedList<T> : ReadOnlyCollection<T>, IState {
        public Path Path { get; private set; }

        public FixedList(Path path = null) : base(new List<T>()) {
            var type = typeof(T);

            switch (type.FullName) {
                case "System.Byte":
                case "System.UInt16":
                case "System.UInt32":
                case "System.UInt64":
                case "System.SByte":
                case "System.Int16":
                case "System.Int32":
                case "System.Int64":
                case "System.Boolean":
                    break;

                default:
                    if (!this.AreItemsState()) {
                        throw new Exception("FixedList can only contain primitive or IState elements.");
                    }

                    break;
            }

            this.Path = path != null ? path : Path.Root;
        }

        public static FixedList<T> Deserialize(Reader reader, Path path = null, bool shouldNotify = false) {
            var list = new FixedList<T>(path);
            UInt16 tag = 0;

            while (!reader.Eof()) {
                // Though FixedList is not really state-enabled, passing tags to its children is still helpful.
                list.Items.Add(list.ReadNested<T>(reader, tag++));
            }

            return list;
        }

        public Int16 WireType(UInt16 tag) {
            return -1;
        }

        public IState Nested(UInt16 tag) {
            return null;
        }

        public bool IsAddSupported() { return false; }
        public bool IsRemoveSupported() { return false; }

        public void ReplayAdd(Reader reader) { throw new Exception("Not supported"); }
        public void ReplayRemove(UInt16 tag) { throw new Exception("Not supported"); }

        public void ReplaceAt(UInt16 tag, WireType wireType, Reader reader, bool shouldNotify = true) {
            throw new Exception("Not supported");
        }

        private bool AreItemsState() {
            return typeof(IState).IsAssignableFrom(typeof(T));
        }
    }
}
