using System;

using Steit.Collections;
using Steit.Encoding;
using Steit.State;

namespace Test1 {
    public sealed class Test {
        public static void Main(string[] args) {
            var hello = Hello.Deserialize(new Reader(new byte[] {
                // Numbers: 1, 2, 1337
                2, 7, 0, 2, 8, 4, 16, 242, 20,
                // Others: -1, -2, 1337
                10, 4, 1, 3, 242, 20,
            }));

            Console.WriteLine("Numbers: {0}", String.Join(", ", hello.Numbers));
            Console.WriteLine("Others: {0}", String.Join(", ", hello.Others));

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

            Replayer.Replay(ref outer, new Reader(new byte[] {
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

            Replayer.Replay(ref multicase, new Reader(new byte[] {
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

            Replayer.Replay(ref list1, new Reader(new byte[] {
                5, 1, 10, 2, 0, 12,
                8, 1, 10, 5, 0, 154, 1, 8, 1,
                1, 1,
                9, 0, 2, 2, 1, 0, 10, 2, 136, 1,
                4, 2, 2, 1, 0,
                4, 2, 2, 1, 1,
            }));

            var list2 = new StateList<SByte>();

            list2.OnUpdate((newItem, oldItem, tag, container) => {
                Console.WriteLine("List<SByte>, update #{0}: {1} => {2}", tag, oldItem, newItem);
            });

            list2.OnAdd((item, tag, container) => {
                Console.WriteLine("List<SByte>, add #{0}: {1}", tag, item);
            });

            list2.OnRemove((item, tag, container) => {
                Console.WriteLine("List<SByte>, remove #{0}: {1}", tag, item);
            });

            Replayer.Replay(ref list2, new Reader(new byte[] {
                4, 1, 10, 1, 20,
                4, 1, 10, 1, 22,
                4, 1, 10, 1, 0,
                4, 2, 2, 1, 1,
            }));

            var map1 = new StateDictionary<Inner>();

            map1.OnUpdate((newItem, oldItem, tag, container) => {
                Console.WriteLine("Dictionary<Inner>, update #{0}: {1} => {2}", tag, InnerToString(oldItem), InnerToString(newItem));
            });

            map1.OnRemove((item, tag, container) => {
                Console.WriteLine("Dictionary<Inner>, remove #{0}: {1}", tag, InnerToString(item));
            });

            Replayer.Replay(ref map1, new Reader(new byte[] {
                8, 0, 2, 1, 5, 10, 2, 0, 12,
                11, 0, 2, 1, 1, 10, 5, 0, 154, 1, 8, 1,
                4, 0, 2, 1, 0,
                9, 0, 2, 2, 1, 0, 10, 2, 136, 1,
                4, 2, 2, 1, 0,
            }));

            var map2 = new StateDictionary<SByte>();

            map2.OnUpdate((newItem, oldItem, tag, container) => {
                Console.WriteLine("Dictionary<SByte>, update #{0}: {1} => {2}", tag, oldItem, newItem);
            });

            map2.OnRemove((item, tag, container) => {
                Console.WriteLine("Dictionary<SByte>, remove #{0}: {1}", tag, item);
            });

            Replayer.Replay(ref map2, new Reader(new byte[] {
                7, 0, 2, 1, 1, 10, 1, 20,
                7, 0, 2, 1, 3, 10, 1, 22,
                7, 0, 2, 1, 7, 10, 1, 0,
                4, 2, 2, 1, 1,
            }));

            Action.OnUpdate((newValue, newVariant, oldValue, oldVariant, container) => {
                Console.WriteLine("Action: variant {0} ({1}) => variant {2} ({3}", oldVariant, oldValue, newVariant, newValue);
            });

            Action.Attack.OnUpdateAttacker((newValue, oldValue, container) => {
                Console.WriteLine("Action / Attack / Attacker: {0} => {1}", oldValue, newValue);
            });

            Action.Attack.OnUpdateDefender((newValue, oldValue, container) => {
                Console.WriteLine("Action / Attack / Defender: {0} => {1}", oldValue, newValue);
            });

            Action.Attack.OnUpdateHits((newValue, oldValue, container) => {
                Console.WriteLine("Action / Attack / Hits:");

                if (oldValue.Count > 0) {
                    Console.WriteLine("Old Hits:");
                    foreach (var hit in oldValue) Console.WriteLine(HitToString(hit));
                } else {
                    Console.WriteLine("Old Hits:\n<empty>");
                }

                if (newValue.Count > 0) {
                    Console.WriteLine("New Hits:");
                    foreach (var hit in newValue) Console.WriteLine(HitToString(hit));
                } else {
                    Console.WriteLine("New Hits:\n<empty>");
                }
            });

            var action = new Action();

            Replayer.Replay(ref action, new Reader(new byte[] {
                // Set variant from to `Action::Attack`
                4, 0, 10, 1, 1,
                // Set attacker to 1
                8, 0, 2, 2, 1, 0, 10, 1, 1,
                // Set defender to 2
                8, 0, 2, 2, 1, 1, 10, 1, 2,
                // Add 4 hits with dummy values from 6 to 9, inclusive
                83, 0, 2, 2, 1, 2, 10, 76,
                    2, 17, 2, 1, 0, 10, 1, 0, 18, 1, 0, 26, 1, 0, 34, 1, 0, 40, 12,
                    10, 17, 2, 1, 0, 10, 1, 0, 18, 1, 0, 26, 1, 0, 34, 1, 0, 40, 14,
                    18, 17, 2, 1, 0, 10, 1, 0, 18, 1, 0, 26, 1, 0, 34, 1, 0, 40, 16,
                    26, 17, 2, 1, 0, 10, 1, 0, 18, 1, 0, 26, 1, 0, 34, 1, 0, 40, 18,
            }));
        }

        private static String InnerToString(Inner inner) {
            return inner != null ? string.Format("{{ Foo: {0}, Bar: {1} }}", inner.Foo, inner.Bar) : "<null>";
        }

        private static String HitToString(Hit hit) {
            return hit != null ? string.Format("{{ Dummy: {0} }}", hit.Dummy) : "<null>";
        }
    }
}
