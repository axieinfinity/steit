using System;
// using System.Linq.Expressions;
using System.Reflection;

using Steit.Codec;

namespace Steit.State {
    public static class StateFactory {
        // private static Trie<Func<IReader, object>?> Deserializers;
        private static Trie<Func<IReader, object>> Deserializers;

        static StateFactory() {
            // Deserializers = new Trie<Func<IReader, object>?>();
            Deserializers = new Trie<Func<IReader, object>>();
            Deserializers["System.Byte"] = reader => reader.ReadByte();
            Deserializers["System.UInt16"] = reader => reader.ReadUInt16();
            Deserializers["System.UInt32"] = reader => reader.ReadUInt32();
            Deserializers["System.UInt64"] = reader => reader.ReadUInt64();
            Deserializers["System.SByte"] = reader => reader.ReadSByte();
            Deserializers["System.Int16"] = reader => reader.ReadInt16();
            Deserializers["System.Int32"] = reader => reader.ReadInt32();
            Deserializers["System.Int64"] = reader => reader.ReadInt64();
            Deserializers["System.Boolean"] = reader => reader.ReadBoolean();
            Deserializers["System.String"] = reader => reader.ReadString();
        }

        public static bool IsPrimitiveType(Type type) {
            // return Deserializers[type.FullName!] != null;
            return Deserializers[type.FullName] != null;
        }

        public static bool IsStateType(Type type) {
            return typeof(IState).IsAssignableFrom(type);
        }

        public static void ValidateType(Type type) {
            if (!IsStateType(type) && !IsPrimitiveType(type)) {
                throw new InvalidOperationException(String.Format("{0} is expected to be a primitive or an IState type.", type.FullName));
            }
        }

        // public static T Construct<T>(Path? path = null) {
        public static T Construct<T>(Path path = null) {
            if (IsStateType(typeof(T))) {
                return Constructor<T>.Construct(path);
            } else {
                // return default!;
                return default;
            }
        }

        // public static T Deserialize<T>(IReader reader, Path? path = null) {
        public static T Deserialize<T>(IReader reader, Path path = null) {
            if (IsStateType(typeof(T))) {
                return DeserializeState<T>(reader, path);
            } else {
                return DeserializePrimitive<T>(reader);
            }
        }

        public static T Deserialize<T>(IReader reader, Path path, UInt32 tag) {
            if (IsStateType(typeof(T))) {
                return DeserializeState<T>(reader, path.GetNested(tag));
            } else {
                return DeserializePrimitive<T>(reader);
            }
        }

        public static T DeserializeNested<T>(IReader reader, Path path, UInt32 tag) {
            if (IsStateType(typeof(T))) {
                return DeserializeState<T>(reader.GetNested(), path.GetNested(tag));
            } else {
                return DeserializePrimitive<T>(reader);
            }
        }

        private static T DeserializePrimitive<T>(IReader reader) {
            // var deserializer = Deserializers[typeof(T).FullName!];
            var deserializer = Deserializers[typeof(T).FullName];

            if (deserializer == null) {
                throw new NotSupportedException(String.Format("`{0}` deserialization is not supported.", typeof(T).FullName));
            }

            return (T) deserializer(reader);
        }

        // private static T DeserializeState<T>(IReader reader, Path? path) /* where T : IState */{
        private static T DeserializeState<T>(IReader reader, Path path) /* where T : IState */{
            var state = Deserializer<T>.Deserialize(reader, path);

            if (state == null) {
                throw new NullReferenceException(String.Format(
                    "`Deserialize(IReader, Path)` method on `{0}` returned `null`.",
                    typeof(T).FullName
                ));
            }

            return state;
        }

        // private sealed class Constructor<T> /* where T : IState */ {
        //     // public static Func<Path?, T> Construct { get; private set; }
        //     public static Func<Path, T> Construct { get; private set; }

        //     static Constructor() {
        //         var constructor = typeof(T).GetConstructor(new Type[] { typeof(Path) });

        //         if (constructor == null) {
        //             throw new MissingMethodException(String.Format(
        //                 "Constructor which takes a `Path` on `{0}` is missing.",
        //                 typeof(T).FullName
        //             ));
        //         }

        //         var parameter = Expression.Parameter(typeof(Path));

        //         Construct = Expression
        //             // .Lambda<Func<Path?, T>>(Expression.New(constructor, parameter), parameter)
        //             .Lambda<Func<Path, T>>(Expression.New(constructor, parameter), parameter)
        //             .Compile();
        //     }
        // }

        private sealed class Constructor<T> /* where T : IState */ {
            private static ConstructorInfo constructor;

            static Constructor() {
                constructor = typeof(T).GetConstructor(new Type[] { typeof(Path) });

                if (constructor == null) {
                    throw new MissingMethodException(String.Format(
                        "Constructor which takes a `Path` on `{0}` is missing.",
                        typeof(T).FullName
                    ));
                }
            }

            public static T Construct(Path path) {
                return (T) constructor.Invoke(null, new object[] { path });
            }
        }

        // private sealed class Deserializer<T> /* where T : IState */ {
        //     // public static Func<IReader, Path?, T> Deserialize { get; private set; }
        //     public static Func<IReader, Path, T> Deserialize { get; private set; }

        //     static Deserializer() {
        //         var deserializer = typeof(T).GetMethod("Deserialize", new Type[] { typeof(IReader), typeof(Path) });

        //         if (deserializer == null) {
        //             throw new MissingMethodException(String.Format(
        //                 "`Deserialize(IReader, Path)` method on `{0}` is missing.",
        //                 typeof(T).FullName
        //             ));
        //         }

        //         var parameters = new ParameterExpression[] {
        //             Expression.Parameter(typeof(IReader)),
        //             Expression.Parameter(typeof(Path)),
        //         };

        //         Deserialize = Expression
        //             // .Lambda<Func<IReader, Path?, T>>(Expression.Call(deserializer, parameters), parameters)
        //             .Lambda<Func<IReader, Path, T>>(Expression.Call(deserializer, parameters), parameters)
        //             .Compile();
        //     }
        // }

        private sealed class Deserializer<T> /* where T : IState */ {
            private static MethodInfo deserializer;

            static Deserializer() {
                deserializer = typeof(T).GetMethod("Deserialize", new Type[] { typeof(IReader), typeof(Path) });

                if (deserializer == null) {
                    throw new MissingMethodException(String.Format(
                        "`Deserialize(IReader, Path)` method on `{0}` is missing.",
                        typeof(T).FullName
                    ));
                }
            }

            public static T Deserialize(IReader reader, Path path) {
                return (T) deserializer.Invoke(null, new object[] { reader, path });
            }
        }

        private sealed class Trie<T> {
            private Node root = new Node();

            public T this[string index] {
                get {
                    var node = this.root;

                    foreach (var character in index) {
                        if (node.children[character] == null) {
                            // return default!;
                            return default;
                        } else {
                            // node = node.children[character]!;
                            node = node.children[character];
                        }
                    }

                    return node.value;
                }

                set {
                    var node = this.root;

                    foreach (var character in index) {
                        // node.children[character] ??= new Node();
                        node.children[character] = node.children[character] ?? new Node();
                        // node = node.children[character]!;
                        node = node.children[character];
                    }

                    node.value = value;
                }
            }

            private sealed class Node {
                // internal Node?[] children = new Node?[256];
                internal Node[] children = new Node[256];
                // internal T value = default!;
                internal T value = default;
            }
        }
    }
}
