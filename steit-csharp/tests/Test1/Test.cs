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

            Multicase.OnUpdate((newValue, newVariant, oldValue, oldVariant, container) => {
                Console.WriteLine("Multicase: variant {0} ({1}) => variant {2} ({3}", oldVariant, oldValue, newVariant, newValue);
            });

            Multicase.FirstCase.OnUpdateFoo((newValue, oldValue, container) => {
                Console.WriteLine("Multicase / FirstCase / Foo: {0} => {1}", oldValue, newValue);
            });

            Multicase.FirstCase.OnUpdateBar((newValue, oldValue, container) => {
                Console.WriteLine("Multicase / FirstCase / Bar: {0} => {1}", oldValue, newValue);
            });

            Multicase.SecondCase.OnUpdateFoo((newValue, oldValue, container) => {
                Console.WriteLine("Multicase / SecondCase / Foo: {0} => {1}", oldValue, newValue);
            });

            Multicase.SecondCase.OnUpdateBar((newValue, oldValue, container) => {
                Console.WriteLine("Multicase / SecondCase / Bar: {0} => {1}", oldValue, newValue);
            });

            var multicase = new Multicase();

            State.ReplayAll(ref multicase, new StateReader(new byte[] {
                6, 0, 2, 0, 10, 1, 1,
                9, 0, 2, 2, 1, 0, 10, 2, 136, 1,
            }));
        }

        private static String InnerToString(Inner inner) {
            return string.Format("{{ Foo: {0}, Bar: {1} }}", inner.Foo, inner.Bar);
        }
    }
}
