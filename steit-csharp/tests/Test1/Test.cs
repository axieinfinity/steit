using System;
using System.Collections.Generic;

using Steit.Reader;

namespace Steit.Test1 {
    public sealed class Test {
        public const UInt16 LOG_UPDATE = 0;
        public const UInt16 LOG_ADD = 1;
        public const UInt16 LOG_REMOVE = 2;

        public static void Main(string[] args) {
            Outer.OnUpdateFoo((newValue, oldValue, container) => {
                Console.WriteLine("Foo: {0} => {1}", oldValue, newValue);
            });

            Outer.OnUpdateBar((newValue, oldValue, container) => {
                Console.WriteLine("Bar: {0} => {1}", oldValue, newValue);
            });

            Outer.OnUpdateInner((newValue, oldValue, container) => {
                Console.WriteLine("Inner: {0} => {1}", oldValue, newValue);
            });

            var outer = new Outer();

            State.ReplayAll(ref outer, new StateReader(new byte[] {
                8, 0, 2, 1, 0, 10, 2, 254, 1,
                7, 0, 2, 1, 1, 10, 1, 1,
                10, 0, 2, 1, 2, 10, 4, 0, 44, 8, 1,
            }));
        }
    }
}
