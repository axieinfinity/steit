using System;

using Steit.Collections;
using Steit.Encoding;

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

            State.State.ReplayAll(ref outer, new Reader(new byte[] {
                8, 0, 2, 1, 0, 10, 2, 254, 1,
                7, 0, 2, 1, 1, 10, 1, 1,
                10, 0, 2, 1, 2, 10, 4, 0, 44, 8, 1,
                9, 0, 2, 2, 2, 0, 10, 2, 192, 2,
                4, 0, 2, 1, 2,
                1, 0,
            }));

            Console.WriteLine("{0} {1} {2}", outer.Foo, outer.Bar, InnerToString(outer.Inner));

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

            State.State.ReplayAll(ref multicase, new Reader(new byte[] {
                4, 0, 10, 1, 1,
                9, 0, 2, 2, 1, 0, 10, 2, 136, 1,
            }));

            var list1 = new StateList<Inner>();

            list1.OnUpdate((newItem, oldItem, tag, container) => {
                Console.WriteLine("List<Inner>, update #{0}: {1} => {2}", tag, InnerToString(oldItem), InnerToString(newItem));
            });

            list1.OnAdd((item, tag, container) => {
                Console.WriteLine("List<Inner>, add #{0}: {1}", tag, InnerToString(item));
            });

            list1.OnRemove((item, tag, container) => {
                Console.WriteLine("List<Inner>, remove #{0}: {1}", tag, InnerToString(item));
            });

            State.State.ReplayAll(ref list1, new Reader(new byte[] {
                5, 1, 10, 2, 0, 12,
                8, 1, 10, 5, 0, 154, 1, 8, 1,
                1, 1,
                9, 0, 2, 2, 1, 0, 10, 2, 136, 1,
                4, 2, 2, 1, 0,
                4, 2, 2, 1, 1,
            }));

            var list2 = new StateList<SByte>();

            list2.OnUpdate((newItem, oldItem, tag, container) => {
                Console.WriteLine("List<Inner>, update #{0}: {1} => {2}", tag, oldItem, newItem);
            });

            list2.OnAdd((item, tag, container) => {
                Console.WriteLine("List<Inner>, add #{0}: {1}", tag, item);
            });

            list2.OnRemove((item, tag, container) => {
                Console.WriteLine("List<Inner>, remove #{0}: {1}", tag, item);
            });

            State.State.ReplayAll(ref list2, new Reader(new byte[] {
                4, 1, 10, 1, 20,
                4, 1, 10, 1, 22,
                4, 1, 10, 1, 0,
                4, 2, 2, 1, 1,
            }));
        }

        private static String InnerToString(Inner inner) {
            return string.Format("{{ Foo: {0}, Bar: {1} }}", inner.Foo, inner.Bar);
        }
    }
}
