using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;

using Steit.Encoding;
using Steit.State;

namespace Steit.Collections {
    public sealed class StateDictionary<T> : ReadOnlyDictionary<UInt16, T>, IState {
        public Path Path { get; private set; }

        private IList<UpdateListener> updateListeners = new List<UpdateListener>();
        private IList<AddListener> addListeners = new List<AddListener>();
        private IList<RemoveListener> removeListeners = new List<RemoveListener>();

        public StateDictionary(Path path = null) : base(new Dictionary<UInt16, T>()) {
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
                        throw new Exception("StateDictionary can only contain primitive or IState elements.");
                    }

                    break;
            }

            this.Path = path != null ? path : Path.Root;
        }

        public delegate void UpdateListener(T newItem, T oldItem, UInt16 tag, StateDictionary<T> container);
        public delegate void AddListener(T item, UInt16 tag, StateDictionary<T> container);
        public delegate void RemoveListener(T item, UInt16 tag, StateDictionary<T> container);

        public static StateDictionary<T> Deserialize(Reader reader, Path path = null, bool shouldNotify = false) {
            var list = new StateDictionary<T>(path);
            list.ReplaceAll(reader, shouldNotify);
            return list;
        }

        public int OnUpdate(UpdateListener listener) { return Utilities.Add(updateListeners, listener); }
        public int OnRemove(RemoveListener listener) { return Utilities.Add(removeListeners, listener); }

        public void RemoveUpdateListener(UpdateListener listener) { updateListeners.Remove(listener); }
        public void RemoveRemoveListener(RemoveListener listener) { removeListeners.Remove(listener); }

        public void RemoveUpdateListenerAt(int index) { updateListeners.RemoveAt(index); }
        public void RemoveRemoveListenerAt(int index) { removeListeners.RemoveAt(index); }

        public void ClearUpdateListeners() { updateListeners.Clear(); }
        public void ClearRemoveListeners() { removeListeners.Clear(); }

        public void ClearAllListeners() {
            updateListeners.Clear();
            removeListeners.Clear();
        }

        public Int16 WireType(UInt16 tag) {
            return (Int16) (this.AreItemsState() ? Encoding.WireType.Sized : Encoding.WireType.Varint);
        }

        public IState Nested(UInt16 tag) {
            if (this.AreItemsState()) {
                return this.ContainsKey(tag) ? (IState) this.Dictionary[tag] : null;
            }

            return null;
        }

        public bool IsAddSupported() { return false; }
        public bool IsRemoveSupported() { return true; }

        public void ReplayAdd(Reader reader) {
            throw new Exception("Not supported");
        }

        public void ReplayRemove(UInt16 tag) {
            this.Remove(tag, shouldNotify: true);
        }

        public void ReplaceAt(UInt16 tag, WireType wireType, Reader reader, bool shouldNotify = true) {
            var oldItem = this.ContainsKey(tag) ? this.Dictionary[tag] : default(T);
            var newItem = this.ReadNested<T>(reader, tag);

            if (shouldNotify) {
                foreach (var listener in this.updateListeners) {
                    listener(newItem, oldItem, tag, this);
                }
            }

            this.Dictionary[tag] = newItem;
        }

        public void Remove(UInt16 tag, bool shouldNotify = true) {
            if (!this.ContainsKey(tag)) {
                return;
            }

            var item = this[tag];

            if (shouldNotify) {
                foreach (var listener in this.removeListeners) {
                    listener(item, tag, this);
                }
            }

            this.Dictionary[tag] = default(T);
        }

        private bool AreItemsState() {
            return typeof(IState).IsAssignableFrom(typeof(T));
        }
    }
}
