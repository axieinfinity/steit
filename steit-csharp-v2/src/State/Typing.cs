using System;

namespace Steit.State {
    public static class Typing {
        public static bool IsPrimitiveType(Type type) {
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
                    return true;

                default:
                    return false;
            }
        }

        public static bool IsStateType(Type type) {
            return typeof(IState).IsAssignableFrom(type);
        }

        public static void CheckPrimitiveOrStateType(Type type) {
            if (!Typing.IsStateType(type) && !Typing.IsPrimitiveType(type)) {
                throw new Exception(String.Format("{0} is expected to be a primitive or a IState type.", type.FullName));
            }
        }

        public static T New<T>(Path path, UInt32 tag) {
            var type = typeof(T);

            if (Typing.IsStateType(type)) {
                var method = type.GetConstructor(new Type[] { typeof(Path) });

                if (method == null) {
                    throw new MissingMethodException(String.Format("Constructor method which accepts a `Path` on `{0}` is missing.", type.FullName));
                }

                var value = (T) method.Invoke(null, new object?[] { path });

                if (value == null) {
                    throw new Exception(String.Format("Constructor method which accepts a `Path` on `{0}` returned `null`.", type.FullName));
                }

                return value;
            } else if (Typing.IsPrimitiveType(type)) {
                var value = default(T);

                if (value == null) {
                    throw new Exception(String.Format("`default({})` returned `null`.", type.FullName));
                }

                return value;
            } else {
                throw new Exception(String.Format("{0} is expected to be a primitive or a IState type.", type.FullName));
            }
        }
    }
}
