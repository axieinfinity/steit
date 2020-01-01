using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;

using Steit.Encoding;
using Steit.State;

namespace Steit.Collections {
    public sealed class StateList<T> : ReadOnlyCollection<T>, IState {
        public Path Path { get; private set; }

        private IList<UpdateListener> updateListeners = new List<UpdateListener>();
        private IList<AddListener> addListeners = new List<AddListener>();
        private IList<RemoveListener> removeListeners = new List<RemoveListener>();

        public StateList(Path path = null) : base(new List<T>()) {
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
                        throw new Exception("StateList can only contain primitive or IState elements.");
                    }

                    break;
            }

            this.Path = path != null ? path : Path.Root;
        }

        public delegate void UpdateListener(T newItem, T oldItem, UInt16 tag, StateList<T> container);
        public delegate void AddListener(T item, UInt16 tag, StateList<T> container);
        public delegate void RemoveListener(T item, UInt16 tag, StateList<T> container);

        public static StateList<T> Deserialize(Reader reader, Path path = null, bool shouldNotify = false) {
            reader = reader.Nested((int) reader.ReadUInt32());
            var list = new StateList<T>(path);

            while (!reader.Eof()) {
                list.Add(reader, shouldNotify);
            }

            return list;
        }

        public int OnUpdate(UpdateListener listener) { return Utilities.Add(updateListeners, listener); }
        public int OnAdd(AddListener listener) { return Utilities.Add(addListeners, listener); }
        public int OnRemove(RemoveListener listener) { return Utilities.Add(removeListeners, listener); }

        public void RemoveUpdateListener(UpdateListener listener) { updateListeners.Remove(listener); }
        public void RemoveAddListener(AddListener listener) { addListeners.Remove(listener); }
        public void RemoveRemoveListener(RemoveListener listener) { removeListeners.Remove(listener); }

        public void RemoveUpdateListenerAt(int index) { updateListeners.RemoveAt(index); }
        public void RemoveAddListenerAt(int index) { addListeners.RemoveAt(index); }
        public void RemoveRemoveListenerAt(int index) { removeListeners.RemoveAt(index); }

        public void ClearUpdateListeners() { updateListeners.Clear(); }
        public void ClearAddListeners() { addListeners.Clear(); }
        public void ClearRemoveListeners() { removeListeners.Clear(); }

        public void ClearAllListeners() {
            updateListeners.Clear();
            addListeners.Clear();
            removeListeners.Clear();
        }

        public Int16 WireType(UInt16 tag) {
            if (tag >= this.Items.Count) {
                return -1;
            }

            return (Int16) (this.AreItemsState() ? Encoding.WireType.Sized : Encoding.WireType.Varint);
        }

        public IState Nested(UInt16 tag) {
            if (this.AreItemsState()) {
                return tag < this.Items.Count ? (IState) this.Items[tag] : null;
            }

            return null;
        }

        public bool IsAddSupported() { return true; }
        public bool IsRemoveSupported() { return true; }

        public void ReplayAdd(Reader reader) {
            if (!this.AreItemsState()) {
                reader.ReadUInt32();
            }

            this.Add(reader, shouldNotify: true);
        }

        public void ReplayRemove(UInt16 tag) {
            this.Remove(tag, shouldNotify: true);
        }

        public void ReplaceAt(UInt16 tag, WireType wireType, Reader reader, bool shouldNotify = true) {
            if (tag >= this.Items.Count) {
                return;
            }

            var oldItem = this.Items[tag];
            var newItem = this.ReadNested<T>(reader, tag);

            if (shouldNotify) {
                foreach (var listener in this.updateListeners) {
                    listener(newItem, oldItem, tag, this);
                }
            }

            this.Items[tag] = newItem;
        }

        public void Add(Reader reader, bool shouldNotify = true) {
            var tag = (UInt16) this.Items.Count;
            var item = this.ReadNested<T>(reader, tag);

            if (shouldNotify) {
                foreach (var listener in this.addListeners) {
                    listener(item, tag, this);
                }
            }

            this.Items.Add(item);
        }

        public void Remove(UInt16 tag, bool shouldNotify = true) {
            if (tag >= this.Items.Count) {
                return;
            }

            var item = this.Items[tag];

            if (shouldNotify) {
                foreach (var listener in this.removeListeners) {
                    listener(item, tag, this);
                }
            }

            this.Items[tag] = default(T);
        }

        private bool AreItemsState() {
            return typeof(IState).IsAssignableFrom(typeof(T));
        }
    }
}
