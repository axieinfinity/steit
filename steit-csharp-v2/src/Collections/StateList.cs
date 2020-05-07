using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;

using Steit.Codec;
using Steit.State;
using Steit.State.Event;

namespace Steit.Collections {
    public sealed class StateList<T> : ReadOnlyCollection<T>, IState {
        public Path Path { get; }

        public StateList(Path? path = null, IList<T>? items = null) : base(items ?? new List<T>()) {
            StateFactory.ValidateType(typeof(T));
            this.Path = path ?? Path.Root;
        }

        public event EventHandler<FieldUpdateEventArgs<T, StateList<T>>>? OnUpdate;
        public event EventHandler<ListPushEventArgs<T, StateList<T>>>? OnPush;
        public event EventHandler<ListPopEventArgs<T, StateList<T>>>? OnPop;

        public void ClearUpdateHandlers() { this.OnUpdate = null; }
        public void ClearPushHandlers() { this.OnPush = null; }
        public void ClearPopHandlers() { this.OnPop = null; }

        public static StateList<T> Deserialize(IReader reader, Path? path = null) {
            path ??= Path.Root;

            var items = new List<T>();
            var tag = 0U;

            while (!reader.EndOfStream()) {
                items.Add(reader.ReadValue<T>(path, tag++));
            }

            return new StateList<T>(path, items);
        }

        public WireType? GetWireType(UInt32 tag) {
            if (tag < this.Items.Count) {
                return StateFactory.IsStateType(typeof(T)) ? WireType.Sized : WireType.Varint;
            } else {
                return null;
            }
        }

        public IState? GetNested(UInt32 tag) {
            return tag < this.Items.Count ? this.Items[(int) tag] as IState : null;
        }

        public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
            if (tag >= this.Items.Count) {
                throw new IndexOutOfRangeException();
            }

            var newItem = StateFactory.Deserialize<T>(reader, this.Path, tag);
            var oldItem = this.Items[(int) tag];

            if (shouldNotify) {
                var args = new FieldUpdateEventArgs<T, StateList<T>>(tag, newItem, oldItem, this);
                this.OnUpdate?.Invoke(this, args);
            }

            this.Items[(int) tag] = newItem;
        }

        public bool IsList() {
            return true;
        }

        public void ReplayListPush(IReader itemReader) {
            var tag = (UInt32) this.Items.Count;
            var item = StateFactory.Deserialize<T>(itemReader, this.Path, tag);

            var args = new ListPushEventArgs<T, StateList<T>>(tag, item, this);
            this.OnPush?.Invoke(this, args);

            this.Items.Add(item);
        }

        public void ReplayListPop() {
            if (this.Items.Count <= 0) {
                throw new InvalidOperationException("Cannot pop from an empty `StateList`.");
            }

            var tag = (UInt32) this.Items.Count - 1;
            var item = this.Items[(int) tag];

            var args = new ListPopEventArgs<T, StateList<T>>(tag, item, this);
            this.OnPop?.Invoke(this, args);

            this.Items.RemoveAt(this.Items.Count - 1);
        }

        public bool IsMap() { return false; }
        public void ReplayMapInsert(IReader keyReader, IReader valueReader) { throw new NotSupportedException(); }
        public void ReplayMapRemove(IReader keyReader) { throw new NotSupportedException(); }
    }
}
