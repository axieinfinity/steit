using System;

using Steit.Codec;
using Steit.Collections;
using Steit.State;

namespace Just.To.Test {
    public sealed class Test {
        public static void Main(string[] args) {
            var hello = Hello.Deserialize(new ByteReader(new byte[] {
                // Numbers: 1, 2, 1337
                2, 4, 2, 4, 242, 20,
                // Others: -1, -2, 1337
                10, 4, 1, 3, 242, 20,
            }));

            Console.WriteLine("Numbers: {0}", String.Join(", ", hello.Numbers));
            Console.WriteLine("Others: {0}", String.Join(", ", hello.Others));

            Outer.OnFooUpdate += (sender, e) => {
                Console.WriteLine("Outer / Foo: {0} => {1}", e.OldValue, e.NewValue);
            };

            Outer.OnBarUpdate += (sender, e) => {
                Console.WriteLine("Outer / Bar: {0} => {1}", e.OldValue, e.NewValue);
            };

            Outer.OnInnerUpdate += (sender, e) => {
                Console.WriteLine("Outer / Inner: {0} => {1}", InnerToString(e.OldValue), InnerToString(e.NewValue));
            };

            Inner.OnFooUpdate += (sender, e) => {
                Console.WriteLine("Outer / Inner / Foo: {0} => {1}", e.OldValue, e.NewValue);
            };

            Inner.OnBarUpdate += (sender, e) => {
                Console.WriteLine("Outer / Inner / Bar: {0} => {1}", e.OldValue, e.NewValue);
            };

            var outer = new Outer();

            StateReplayer.Replay<Outer>(ref outer, new ByteReader(new byte[] {
                8, 0, 2, 1, 0, 10, 2, 254, 1,
                7, 0, 2, 1, 1, 10, 1, 1,
                10, 0, 2, 1, 2, 10, 4, 0, 44, 8, 1,
                9, 0, 2, 2, 2, 0, 10, 2, 192, 2,
                4, 0, 2, 1, 2,
                1, 0,
            }));

            Console.WriteLine("{0} {1} {2}", outer.Foo, outer.Bar, InnerToString(outer.Inner));

            Multicase.OnUpdate += (sender, e) => {
                Console.WriteLine("Multicase: variant {0} ({1}) => variant {2} ({3}", e.OldTag, e.OldVariant, e.NewTag, e.NewVariant);
            };

            Multicase.FirstCase.OnCounterUpdate += (sender, e) => {
                Console.WriteLine("Multicase / FirstCase / Counter: {0} => {1}", e.OldValue, e.NewValue);
            };

            Multicase.FirstCase.OnEnabledUpdate += (sender, e) => {
                Console.WriteLine("Multicase / FirstCase / Enabled: {0} => {1}", e.OldValue, e.NewValue);
            };

            Multicase.SecondCase.OnCounterUpdate += (sender, e) => {
                Console.WriteLine("Multicase / SecondCase / Counter: {0} => {1}", e.OldValue, e.NewValue);
            };

            Multicase.SecondCase.OnEnabledUpdate += (sender, e) => {
                Console.WriteLine("Multicase / SecondCase / Enabled: {0} => {1}", e.OldValue, e.NewValue);
            };

            var multicase = new Multicase();

            StateReplayer.Replay<Multicase>(ref multicase, new ByteReader(new byte[] {
                4, 0, 10, 1, 1,
                9, 0, 2, 2, 1, 0, 10, 2, 136, 1,
            }));

            var list1 = new StateList<Inner>();

            list1.OnUpdate += (sender, e) => {
                Console.WriteLine("List<Inner>, update #{0}: {1} => {2}", e.Tag, InnerToString(e.OldValue), InnerToString(e.NewValue));
            };

            list1.OnPush += (sender, e) => {
                Console.WriteLine("List<Inner>, add #{0}: {1}", e.Tag, InnerToString(e.Item));
            };

            list1.OnPop += (sender, e) => {
                Console.WriteLine("List<Inner>, remove #{0}: {1}", e.Tag, InnerToString(e.Item));
            };

            StateReplayer.Replay<StateList<Inner>>(ref list1, new ByteReader(new byte[] {
                5, 8, 10, 2, 0, 12,
                8, 8, 10, 5, 0, 154, 1, 8, 1,
                1, 8,
                9, 0, 2, 2, 1, 0, 10, 2, 136, 1,
                1, 9,
                4, 0, 2, 1, 0,
                4, 0, 2, 1, 1,
            }));

            var list2 = new StateList<SByte>();

            list2.OnUpdate += (sender, e) => {
                Console.WriteLine("List<SByte>, update #{0}: {1} => {2}", e.Tag, e.OldValue, e.NewValue);
            };

            list2.OnPush += (sender, e) => {
                Console.WriteLine("List<SByte>, add #{0}: {1}", e.Tag, e.Item);
            };

            list2.OnPop += (sender, e) => {
                Console.WriteLine("List<SByte>, remove #{0}: {1}", e.Tag, e.Item);
            };

            StateReplayer.Replay<StateList<SByte>>(ref list2, new ByteReader(new byte[] {
                4, 8, 10, 1, 20,
                4, 8, 10, 1, 22,
                4, 8, 10, 1, 0,
                1, 9,
                7, 0, 2, 1, 1, 10, 1, 0,
            }));

            // var map1 = new StateDictionary<Inner>();

            // map1.OnUpdate((newItem, oldItem, tag, container) => {
            //     Console.WriteLine("Dictionary<Inner>, update #{0}: {1} => {2}", tag, InnerToString(oldItem), InnerToString(newItem));
            // });

            // map1.OnRemove((item, tag, container) => {
            //     Console.WriteLine("Dictionary<Inner>, remove #{0}: {1}", tag, InnerToString(item));
            // });

            // Replayer.Replay(ref map1, new Reader(new byte[] {
            //     8, 0, 2, 1, 5, 10, 2, 0, 12,
            //     11, 0, 2, 1, 1, 10, 5, 0, 154, 1, 8, 1,
            //     4, 0, 2, 1, 0,
            //     9, 0, 2, 2, 1, 0, 10, 2, 136, 1,
            //     4, 2, 2, 1, 0,
            // }));

            // var map2 = new StateDictionary<SByte>();

            // map2.OnUpdate((newItem, oldItem, tag, container) => {
            //     Console.WriteLine("Dictionary<SByte>, update #{0}: {1} => {2}", tag, oldItem, newItem);
            // });

            // map2.OnRemove((item, tag, container) => {
            //     Console.WriteLine("Dictionary<SByte>, remove #{0}: {1}", tag, item);
            // });

            // Replayer.Replay(ref map2, new Reader(new byte[] {
            //     7, 0, 2, 1, 1, 10, 1, 20,
            //     7, 0, 2, 1, 3, 10, 1, 22,
            //     7, 0, 2, 1, 7, 10, 1, 0,
            //     4, 2, 2, 1, 1,
            // }));

            // Action.OnUpdate((newValue, newVariant, oldValue, oldVariant, container) => {
            //     Console.WriteLine("Action: variant {0} ({1}) => variant {2} ({3}", oldVariant, oldValue, newVariant, newValue);
            // });

            // Action.Attack.OnUpdateAttacker((newValue, oldValue, container) => {
            //     Console.WriteLine("Action / Attack / Attacker: {0} => {1}", oldValue, newValue);
            // });

            // Action.Attack.OnUpdateDefender((newValue, oldValue, container) => {
            //     Console.WriteLine("Action / Attack / Defender: {0} => {1}", oldValue, newValue);
            // });

            // Action.Attack.OnUpdateHits((newValue, oldValue, container) => {
            //     Console.WriteLine("Action / Attack / Hits:");

            //     if (oldValue.Count > 0) {
            //         Console.WriteLine("Old Hits:");
            //         foreach (var hit in oldValue) Console.WriteLine(HitToString(hit));
            //     } else {
            //         Console.WriteLine("Old Hits:\n<empty>");
            //     }

            //     if (newValue.Count > 0) {
            //         Console.WriteLine("New Hits:");
            //         foreach (var hit in newValue) Console.WriteLine(HitToString(hit));
            //     } else {
            //         Console.WriteLine("New Hits:\n<empty>");
            //     }
            // });

            // var action = new Action();

            // Replayer.Replay(ref action, new Reader(new byte[] {
            //     // Set variant from to `Action::Attack`
            //     4, 0, 10, 1, 1,
            //     // Set attacker to 1
            //     8, 0, 2, 2, 1, 0, 10, 1, 1,
            //     // Set defender to 2
            //     8, 0, 2, 2, 1, 1, 10, 1, 2,
            //     // Add 4 hits with dummy values from 6 to 9, inclusive
            //     83, 0, 2, 2, 1, 2, 10, 76,
            //         2, 17, 2, 1, 0, 10, 1, 0, 18, 1, 0, 26, 1, 0, 34, 1, 0, 40, 12,
            //         10, 17, 2, 1, 0, 10, 1, 0, 18, 1, 0, 26, 1, 0, 34, 1, 0, 40, 14,
            //         18, 17, 2, 1, 0, 10, 1, 0, 18, 1, 0, 26, 1, 0, 34, 1, 0, 40, 16,
            //         26, 17, 2, 1, 0, 10, 1, 0, 18, 1, 0, 26, 1, 0, 34, 1, 0, 40, 18,
            // }));
        }

        private static String InnerToString(Inner inner) {
            return inner != null ? string.Format("{{ Foo: {0}, Bar: {1} }}", inner.Foo, inner.Bar) : "<null>";
        }

        // private static String HitToString(Hit hit) {
        //     return hit != null ? string.Format("{{ Dummy: {0} }}", hit.Dummy) : "<null>";
        // }
    }
}
