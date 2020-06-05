using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;

using Steit.Codec;
using Steit.State;
using Steit.State.Event;

namespace Steit.Collections {
    public sealed class StateMap<T> : ReadOnlyDictionary<UInt32, T>, IState {
        public Path Path { get; }

        // public StateMap(Path? path = null, IDictionary<UInt32, T>? items = null) : base(items ?? new Dictionary<UInt32, T>()) {
        public StateMap(Path path = null, IDictionary<UInt32, T> items = null) : base(items ?? new Dictionary<UInt32, T>()) {
            StateFactory.ValidateType(typeof(T));
            this.Path = path ?? Path.Root;
        }

        // public event EventHandler<FieldUpdateEventArgs<T, StateMap<T>>>? OnUpdate;
        public event EventHandler<FieldUpdateEventArgs<T, StateMap<T>>> OnUpdate;
        // public event EventHandler<MapInsertEventArgs<T, StateMap<T>>>? OnInsert;
        public event EventHandler<MapInsertEventArgs<T, StateMap<T>>> OnInsert;
        // public event EventHandler<MapRemoveEventArgs<T, StateMap<T>>>? OnRemove;
        public event EventHandler<MapRemoveEventArgs<T, StateMap<T>>> OnRemove;

        public void ClearUpdateHandlers() { this.OnUpdate = null; }
        public void ClearInsertHandlers() { this.OnInsert = null; }
        public void ClearRemoveHandlers() { this.OnRemove = null; }

        // public static StateMap<T> Deserialize(IReader reader, Path? path = null) {
        public static StateMap<T> Deserialize(IReader reader, Path path = null) {
            // path ??= Path.Root;
            path = path ?? Path.Root;

            var entries = new Dictionary<UInt32, T>();

            while (!reader.EndOfStream()) {
                var tag = reader.ReadKey().Tag;
                // TODO: Wire type should be considered here.
                entries[tag] = reader.ReadValue<T>(path, tag);
            }

            return new StateMap<T>(path, entries);
        }

        public WireType? GetWireType(UInt32 tag) {
            return StateFactory.IsStateType(typeof(T)) ? WireType.Sized : WireType.Varint;
        }

        // public IState? GetNested(UInt32 tag) {
        public IState GetNested(UInt32 tag) {
            return this.ContainsKey(tag) ? this[tag] as IState : null;
        }

        public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
            var newValue = StateFactory.Deserialize<T>(reader, this.Path, tag);

            if (shouldNotify) {
                if (this.ContainsKey(tag)) {
                    var oldValue = this[tag];
                    var args = new FieldUpdateEventArgs<T, StateMap<T>>(tag, newValue, oldValue, this);
                    this.OnUpdate?.Invoke(this, args);
                } else {
                    var args = new MapInsertEventArgs<T, StateMap<T>>(tag, newValue, this);
                    this.OnInsert?.Invoke(this, args);
                }
            }

            this.Dictionary[tag] = newValue;
        }

        public void ReplayListPush(IReader reader) { throw new NotSupportedException(); }
        public void ReplayListPop() { throw new NotSupportedException(); }

        public void ReplayMapRemove(UInt32 key) {
            if (!this.ContainsKey(key)) {
                throw new KeyNotFoundException();
            }

            var args = new MapRemoveEventArgs<T, StateMap<T>>(key, this[key], this);
            this.OnRemove?.Invoke(this, args);

            this.Dictionary.Remove(key);
        }
    }
}
