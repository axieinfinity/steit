using System;

using Steit.Builtins;
using Steit.Codec;
using Steit.Collections;
using Steit.State;
using Steit.State.Event;

namespace Steit.State {
    public sealed class LogEntry : IEnumState {
        public const UInt32 UpdateTag = 0;
        public const UInt32 ListPushTag = 8;
        public const UInt32 ListPopTag = 9;
        public const UInt32 MapInsertTag = 12;
        public const UInt32 MapRemoveTag = 13;

        public Path Path { get; }

        public UInt32 Tag { get; private set; }
        public IState Variant { get; private set; }

        public Update? UpdateVariant { get { return this.Variant as Update; } }
        public ListPush? ListPushVariant { get { return this.Variant as ListPush; } }
        public ListPop? ListPopVariant { get { return this.Variant as ListPop; } }
        public MapInsert? MapInsertVariant { get { return this.Variant as MapInsert; } }
        public MapRemove? MapRemoveVariant { get { return this.Variant as MapRemove; } }

        public LogEntry(Path? path = null) {
            this.Path = path ?? Path.Root;
            this.Tag = 0;
            this.Variant = new Update(this.Path.GetNested(0));
        }

        public static event EventHandler<VariantUpdateEventArgs<LogEntry>>? OnUpdate;

        public static void ClearUpdateHandlers() {
            OnUpdate = null;
        }

        public static LogEntry Deserialize(IReader reader, Path? path = null) {
            var logEntry = new LogEntry(path);
            logEntry.Replace(reader);
            return logEntry;
        }

        public WireType? GetWireType(UInt32 tag) {
            switch (tag) {
                case 0: return WireType.Sized;
                case 8: return WireType.Sized;
                case 9: return WireType.Sized;
                case 12: return WireType.Sized;
                case 13: return WireType.Sized;
                default: return null;
            }
        }

        public IState? GetNested(UInt32 tag) {
            return tag == this.Tag ? this.Variant : null;
        }

        public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
            switch (tag) {
                case 0: this.UpdateAndNotify(0, Update.Deserialize(reader, this.Path.GetNested(0)), shouldNotify); break;
                case 8: this.UpdateAndNotify(8, ListPush.Deserialize(reader, this.Path.GetNested(8)), shouldNotify); break;
                case 9: this.UpdateAndNotify(9, ListPop.Deserialize(reader, this.Path.GetNested(9)), shouldNotify); break;
                case 12: this.UpdateAndNotify(12, MapInsert.Deserialize(reader, this.Path.GetNested(12)), shouldNotify); break;
                case 13: this.UpdateAndNotify(13, MapRemove.Deserialize(reader, this.Path.GetNested(13)), shouldNotify); break;
                default: reader.SkipToEnd(); break;
            }
        }

        public bool IsList() { return false; }
        public void ReplayListPush(IReader itemReader) { throw new NotSupportedException(); }
        public void ReplayListPop() { throw new NotSupportedException(); }

        public bool IsMap() { return false; }
        public void ReplayMapInsert(IReader keyReader, IReader valueReader) { throw new NotSupportedException(); }
        public void ReplayMapRemove(IReader keyReader) { throw new NotSupportedException(); }

        private void UpdateAndNotify(UInt32 newTag, IState newVariant, bool shouldNotify) {
            if (shouldNotify) {
                var args = new VariantUpdateEventArgs<LogEntry>(newTag, newVariant, this.Tag, this.Variant, this);
                LogEntry.OnUpdate?.Invoke(this, args);
            }

            this.Tag = newTag;
            this.Variant = newVariant;
        }

        // Variant (0): Update

        public sealed class Update : IState {
            public Path Path { get; }

            public Vector<UInt32> FlattenPath { get; private set; }
            public Bytes Value { get; private set; }

            internal Update(Path? path = null) {
                this.Path = path ?? Path.Root;
                this.FlattenPath = new Vector<UInt32>(this.Path.GetNested(0));
                this.Value = new Bytes(this.Path.GetNested(1));
            }

            public static event EventHandler<FieldUpdateEventArgs<Vector<UInt32>, Update>>? OnFlattenPathUpdate;
            public static event EventHandler<FieldUpdateEventArgs<Bytes, Update>>? OnValueUpdate;

            public static void ClearFlattenPathUpdateHandlers() { OnFlattenPathUpdate = null; }
            public static void ClearValueUpdateHandlers() { OnValueUpdate = null; }

            public static void ClearUpdateHandlers() {
                OnFlattenPathUpdate = null;
                OnValueUpdate = null;
            }

            internal static Update Deserialize(IReader reader, Path? path = null) {
                var update = new Update(path);
                update.Replace(reader);
                return update;
            }

            public WireType? GetWireType(UInt32 tag) {
                switch (tag) {
                    case 0: return WireType.Sized;
                    case 1: return WireType.Sized;
                    default: return null;
                }
            }

            public IState? GetNested(UInt32 tag) {
                switch (tag) {
                    case 0: return this.FlattenPath;
                    case 1: return this.Value;
                    default: return null;
                }
            }

            public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
                switch (tag) {
                    case 0: this.FlattenPath = this.MaybeNotify(0, Vector<UInt32>.Deserialize(reader, this.Path.GetNested(0)), this.FlattenPath, OnFlattenPathUpdate, shouldNotify); break;
                    case 1: this.Value = this.MaybeNotify(1, Bytes.Deserialize(reader, this.Path.GetNested(1)), this.Value, OnValueUpdate, shouldNotify); break;
                    default: reader.SkipField(wireType); break;
                }
            }

            public bool IsList() { return false; }
            public void ReplayListPush(IReader itemReader) { throw new NotSupportedException(); }
            public void ReplayListPop() { throw new NotSupportedException(); }

            public bool IsMap() { return false; }
            public void ReplayMapInsert(IReader keyReader, IReader valueReader) { throw new NotSupportedException(); }
            public void ReplayMapRemove(IReader keyReader) { throw new NotSupportedException(); }

            private TValue MaybeNotify<TValue>(
                UInt32 tag,
                TValue newValue,
                TValue oldValue,
                EventHandler<FieldUpdateEventArgs<TValue, Update>>? handler,
                bool shouldNotify
            ) {
                if (shouldNotify) {
                    var args = new FieldUpdateEventArgs<TValue, Update>(tag, newValue, oldValue, this);
                    handler?.Invoke(this, args);
                }

                return newValue;
            }
        }

        // Variant (8): ListPush

        public sealed class ListPush : IState {
            public Path Path { get; }

            public Vector<UInt32> FlattenPath { get; private set; }
            public Bytes Item { get; private set; }

            internal ListPush(Path? path = null) {
                this.Path = path ?? Path.Root;
                this.FlattenPath = new Vector<UInt32>(this.Path.GetNested(0));
                this.Item = new Bytes(this.Path.GetNested(1));
            }

            public static event EventHandler<FieldUpdateEventArgs<Vector<UInt32>, ListPush>>? OnFlattenPathUpdate;
            public static event EventHandler<FieldUpdateEventArgs<Bytes, ListPush>>? OnItemUpdate;

            public static void ClearFlattenPathUpdateHandlers() { OnFlattenPathUpdate = null; }
            public static void ClearItemUpdateHandlers() { OnItemUpdate = null; }

            public static void ClearUpdateHandlers() {
                OnFlattenPathUpdate = null;
                OnItemUpdate = null;
            }

            internal static ListPush Deserialize(IReader reader, Path? path = null) {
                var listPush = new ListPush(path);
                listPush.Replace(reader);
                return listPush;
            }

            public WireType? GetWireType(UInt32 tag) {
                switch (tag) {
                    case 0: return WireType.Sized;
                    case 1: return WireType.Sized;
                    default: return null;
                }
            }

            public IState? GetNested(UInt32 tag) {
                switch (tag) {
                    case 0: return this.FlattenPath;
                    case 1: return this.Item;
                    default: return null;
                }
            }

            public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
                switch (tag) {
                    case 0: this.FlattenPath = this.MaybeNotify(0, Vector<UInt32>.Deserialize(reader, this.Path.GetNested(0)), this.FlattenPath, OnFlattenPathUpdate, shouldNotify); break;
                    case 1: this.Item = this.MaybeNotify(1, Bytes.Deserialize(reader, this.Path.GetNested(1)), this.Item, OnItemUpdate, shouldNotify); break;
                    default: reader.SkipField(wireType); break;
                }
            }

            public bool IsList() { return false; }
            public void ReplayListPush(IReader itemReader) { throw new NotSupportedException(); }
            public void ReplayListPop() { throw new NotSupportedException(); }

            public bool IsMap() { return false; }
            public void ReplayMapInsert(IReader keyReader, IReader valueReader) { throw new NotSupportedException(); }
            public void ReplayMapRemove(IReader keyReader) { throw new NotSupportedException(); }

            private TValue MaybeNotify<TValue>(
                UInt32 tag,
                TValue newValue,
                TValue oldValue,
                EventHandler<FieldUpdateEventArgs<TValue, ListPush>>? handler,
                bool shouldNotify
            ) {
                if (shouldNotify) {
                    var args = new FieldUpdateEventArgs<TValue, ListPush>(tag, newValue, oldValue, this);
                    handler?.Invoke(this, args);
                }

                return newValue;
            }
        }

        // Variant (9): ListPop

        public sealed class ListPop : IState {
            public Path Path { get; }
            public Vector<UInt32> FlattenPath { get; private set; }

            internal ListPop(Path? path = null) {
                this.Path = path ?? Path.Root;
                this.FlattenPath = new Vector<UInt32>(this.Path.GetNested(0));
            }

            public static event EventHandler<FieldUpdateEventArgs<Vector<UInt32>, ListPop>>? OnFlattenPathUpdate;

            public static void ClearFlattenPathUpdateHandlers() {
                OnFlattenPathUpdate = null;
            }

            public static void ClearUpdateHandlers() {
                OnFlattenPathUpdate = null;
            }

            internal static ListPop Deserialize(IReader reader, Path? path = null) {
                var listPop = new ListPop(path);
                listPop.Replace(reader);
                return listPop;
            }

            public WireType? GetWireType(UInt32 tag) {
                switch (tag) {
                    case 0: return WireType.Sized;
                    default: return null;
                }
            }

            public IState? GetNested(UInt32 tag) {
                switch (tag) {
                    case 0: return this.FlattenPath;
                    default: return null;
                }
            }

            public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
                switch (tag) {
                    case 0: this.FlattenPath = this.MaybeNotify(0, Vector<UInt32>.Deserialize(reader, this.Path.GetNested(0)), this.FlattenPath, OnFlattenPathUpdate, shouldNotify); break;
                    default: reader.SkipField(wireType); break;
                }
            }

            public bool IsList() { return false; }
            public void ReplayListPush(IReader itemReader) { throw new NotSupportedException(); }
            public void ReplayListPop() { throw new NotSupportedException(); }

            public bool IsMap() { return false; }
            public void ReplayMapInsert(IReader keyReader, IReader valueReader) { throw new NotSupportedException(); }
            public void ReplayMapRemove(IReader keyReader) { throw new NotSupportedException(); }

            private TValue MaybeNotify<TValue>(
                UInt32 tag,
                TValue newValue,
                TValue oldValue,
                EventHandler<FieldUpdateEventArgs<TValue, ListPop>>? handler,
                bool shouldNotify
            ) {
                if (shouldNotify) {
                    var args = new FieldUpdateEventArgs<TValue, ListPop>(tag, newValue, oldValue, this);
                    handler?.Invoke(this, args);
                }

                return newValue;
            }
        }

        // Variant (12): MapInsert

        public sealed class MapInsert : IState {
            public Path Path { get; }

            public Vector<UInt32> FlattenPath { get; private set; }
            public Bytes Key { get; private set; }
            public Bytes Value { get; private set; }

            internal MapInsert(Path? path = null) {
                this.Path = path ?? Path.Root;
                this.FlattenPath = new Vector<UInt32>(this.Path.GetNested(0));
                this.Key = new Bytes(this.Path.GetNested(1));
                this.Value = new Bytes(this.Path.GetNested(2));
            }

            public static event EventHandler<FieldUpdateEventArgs<Vector<UInt32>, MapInsert>>? OnFlattenPathUpdate;
            public static event EventHandler<FieldUpdateEventArgs<Bytes, MapInsert>>? OnKeyUpdate;
            public static event EventHandler<FieldUpdateEventArgs<Bytes, MapInsert>>? OnValueUpdate;

            public static void ClearFlattenPathUpdateHandlers() { OnFlattenPathUpdate = null; }
            public static void ClearKeyUpdateHandlers() { OnKeyUpdate = null; }
            public static void ClearValueUpdateHandlers() { OnValueUpdate = null; }

            public static void ClearUpdateHandlers() {
                OnFlattenPathUpdate = null;
                OnKeyUpdate = null;
                OnValueUpdate = null;
            }

            internal static MapInsert Deserialize(IReader reader, Path? path = null) {
                var mapInsert = new MapInsert(path);
                mapInsert.Replace(reader);
                return mapInsert;
            }

            public WireType? GetWireType(UInt32 tag) {
                switch (tag) {
                    case 0: return WireType.Sized;
                    case 1: return WireType.Sized;
                    case 2: return WireType.Sized;
                    default: return null;
                }
            }

            public IState? GetNested(UInt32 tag) {
                switch (tag) {
                    case 0: return this.FlattenPath;
                    case 1: return this.Key;
                    case 2: return this.Value;
                    default: return null;
                }
            }

            public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
                switch (tag) {
                    case 0: this.FlattenPath = this.MaybeNotify(0, Vector<UInt32>.Deserialize(reader, this.Path.GetNested(0)), this.FlattenPath, OnFlattenPathUpdate, shouldNotify); break;
                    case 1: this.Key = this.MaybeNotify(1, Bytes.Deserialize(reader, this.Path.GetNested(1)), this.Key, OnKeyUpdate, shouldNotify); break;
                    case 2: this.Value = this.MaybeNotify(2, Bytes.Deserialize(reader, this.Path.GetNested(2)), this.Value, OnValueUpdate, shouldNotify); break;
                    default: reader.SkipField(wireType); break;
                }
            }

            public bool IsList() { return false; }
            public void ReplayListPush(IReader itemReader) { throw new NotSupportedException(); }
            public void ReplayListPop() { throw new NotSupportedException(); }

            public bool IsMap() { return false; }
            public void ReplayMapInsert(IReader keyReader, IReader valueReader) { throw new NotSupportedException(); }
            public void ReplayMapRemove(IReader keyReader) { throw new NotSupportedException(); }

            private TValue MaybeNotify<TValue>(
                UInt32 tag,
                TValue newValue,
                TValue oldValue,
                EventHandler<FieldUpdateEventArgs<TValue, MapInsert>>? handler,
                bool shouldNotify
            ) {
                if (shouldNotify) {
                    var args = new FieldUpdateEventArgs<TValue, MapInsert>(tag, newValue, oldValue, this);
                    handler?.Invoke(this, args);
                }

                return newValue;
            }
        }

        // Variant (13): MapRemove

        public sealed class MapRemove : IState {
            public Path Path { get; }

            public Vector<UInt32> FlattenPath { get; private set; }
            public Bytes Key { get; private set; }

            internal MapRemove(Path? path = null) {
                this.Path = path ?? Path.Root;
                this.FlattenPath = new Vector<UInt32>(this.Path.GetNested(0));
                this.Key = new Bytes(this.Path.GetNested(1));
            }

            public static event EventHandler<FieldUpdateEventArgs<Vector<UInt32>, MapRemove>>? OnFlattenPathUpdate;
            public static event EventHandler<FieldUpdateEventArgs<Bytes, MapRemove>>? OnKeyUpdate;

            public static void ClearFlattenPathUpdateHandlers() { OnFlattenPathUpdate = null; }
            public static void ClearKeyUpdateHandlers() { OnKeyUpdate = null; }

            public static void ClearUpdateHandlers() {
                OnFlattenPathUpdate = null;
                OnKeyUpdate = null;
            }

            internal static MapRemove Deserialize(IReader reader, Path? path = null) {
                var mapRemove = new MapRemove(path);
                mapRemove.Replace(reader);
                return mapRemove;
            }

            public WireType? GetWireType(UInt32 tag) {
                switch (tag) {
                    case 0: return WireType.Sized;
                    case 1: return WireType.Sized;
                    default: return null;
                }
            }

            public IState? GetNested(UInt32 tag) {
                switch (tag) {
                    case 0: return this.FlattenPath;
                    case 1: return this.Key;
                    default: return null;
                }
            }

            public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {
                switch (tag) {
                    case 0: this.FlattenPath = this.MaybeNotify(0, Vector<UInt32>.Deserialize(reader, this.Path.GetNested(0)), this.FlattenPath, OnFlattenPathUpdate, shouldNotify); break;
                    case 1: this.Key = this.MaybeNotify(1, Bytes.Deserialize(reader, this.Path.GetNested(1)), this.Key, OnKeyUpdate, shouldNotify); break;
                    default: reader.SkipField(wireType); break;
                }
            }

            public bool IsList() { return false; }
            public void ReplayListPush(IReader itemReader) { throw new NotSupportedException(); }
            public void ReplayListPop() { throw new NotSupportedException(); }

            public bool IsMap() { return false; }
            public void ReplayMapInsert(IReader keyReader, IReader valueReader) { throw new NotSupportedException(); }
            public void ReplayMapRemove(IReader keyReader) { throw new NotSupportedException(); }

            private TValue MaybeNotify<TValue>(
                UInt32 tag,
                TValue newValue,
                TValue oldValue,
                EventHandler<FieldUpdateEventArgs<TValue, MapRemove>>? handler,
                bool shouldNotify
            ) {
                if (shouldNotify) {
                    var args = new FieldUpdateEventArgs<TValue, MapRemove>(tag, newValue, oldValue, this);
                    handler?.Invoke(this, args);
                }

                return newValue;
            }
        }
    }
}
