using System;
using System.Collections.Generic;

using Steit;
using Steit.Reader;

namespace Steit.Test1 {
    public sealed class Test {
        public static void Main(string[] args) {
            Outer.OnUpdateFoo((newValue, oldValue, container) => {
                Console.WriteLine("Outer / Foo: {0} => {1}", oldValue, newValue);
            });

            Outer.OnUpdateBar((newValue, oldValue, container) => {
                Console.WriteLine("Outer / Bar: {0} => {1}", oldValue, newValue);
            });

            Outer.OnUpdateInner((newValue, oldValue, container) => {
                Console.WriteLine("Outer / Inner: {0} => {1}", InnerToString(oldValue), InnerToString(newValue));
            });

            Inner.OnUpdateFoo((newValue, oldValue, container) => {
                Console.WriteLine("Outer / Inner / Foo: {0} => {1}", oldValue, newValue);
            });

            Inner.OnUpdateBar((newValue, oldValue, container) => {
                Console.WriteLine("Outer / Inner / Bar: {0} => {1}", oldValue, newValue);
            });

            var outer = new Outer();

            State.ReplayAll(ref outer, new StateReader(new byte[] {
                8, 0, 2, 1, 0, 10, 2, 254, 1,
                7, 0, 2, 1, 1, 10, 1, 1,
                10, 0, 2, 1, 2, 10, 4, 0, 44, 8, 1,
                9, 0, 2, 2, 2, 0, 10, 2, 192, 2,
            }));
        }

        private static String InnerToString(Inner inner) {
            return string.Format("{{ Foo: {0}, Bar: {1} }}", inner.Foo, inner.Bar);
        }
    }
}
