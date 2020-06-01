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
                Console.WriteLine("StateList<Inner>, update #{0}: {1} => {2}", e.Tag, InnerToString(e.OldValue), InnerToString(e.NewValue));
            };

            list1.OnPush += (sender, e) => {
                Console.WriteLine("StateList<Inner>, add #{0}: {1}", e.Tag, InnerToString(e.Item));
            };

            list1.OnPop += (sender, e) => {
                Console.WriteLine("StateList<Inner>, remove #{0}: {1}", e.Tag, InnerToString(e.Item));
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
                Console.WriteLine("StateList<SByte>, update #{0}: {1} => {2}", e.Tag, e.OldValue, e.NewValue);
            };

            list2.OnPush += (sender, e) => {
                Console.WriteLine("StateList<SByte>, add #{0}: {1}", e.Tag, e.Item);
            };

            list2.OnPop += (sender, e) => {
                Console.WriteLine("StateList<SByte>, remove #{0}: {1}", e.Tag, e.Item);
            };

            StateReplayer.Replay<StateList<SByte>>(ref list2, new ByteReader(new byte[] {
                4, 8, 10, 1, 20,
                4, 8, 10, 1, 22,
                4, 8, 10, 1, 0,
                1, 9,
                7, 0, 2, 1, 1, 10, 1, 0,
            }));

            var map1 = new StateMap<Inner>();

            map1.OnUpdate += (sender, e) => {
                Console.WriteLine("StateMap<Inner>, update #{0}: {1} => {2}", e.Tag, InnerToString(e.OldValue), InnerToString(e.NewValue));
            };

            map1.OnInsert += (sender, e) => {
                Console.WriteLine("StateMap<Inner>, insert #{0}: {1}", e.Tag, InnerToString(e.Value));
            };

            map1.OnRemove += (sender, e) => {
                Console.WriteLine("StateMap<Inner>, remove #{0}: {1}", e.Tag, InnerToString(e.Value));
            };

            StateReplayer.Replay<StateMap<Inner>>(ref map1, new ByteReader(new byte[] {
                8, 0, 2, 1, 5, 10, 2, 0, 12,
                11, 0, 2, 1, 1, 10, 5, 0, 154, 1, 8, 1,
                4, 0, 2, 1, 0,
                10, 0, 2, 1, 0, 10, 4, 0, 84, 8, 1,
                9, 0, 2, 2, 1, 0, 10, 2, 136, 1,
                3, 12, 8, 0,
             }));

            var map2 = new StateMap<SByte>();

            map2.OnUpdate += (sender, e) => {
                Console.WriteLine("StateMap<SByte>, update #{0}: {1} => {2}", e.Tag, e.OldValue, e.NewValue);
            };

            map2.OnInsert += (sender, e) => {
                Console.WriteLine("StateMap<SByte>, insert #{0}: {1}", e.Tag, e.Value);
            };

            map2.OnRemove += (sender, e) => {
                Console.WriteLine("StateMap<SByte>, remove #{0}: {1}", e.Tag, e.Value);
            };

            StateReplayer.Replay<StateMap<SByte>>(ref map2, new ByteReader(new byte[] {
                7, 0, 2, 1, 1, 10, 1, 20,
                7, 0, 2, 1, 3, 10, 1, 22,
                7, 0, 2, 1, 7, 10, 1, 0,
                7, 0, 2, 1, 7, 10, 1, 1,
                3, 12, 8, 1,
            }));

            OldAction.OnUpdate += (sender, e) => {
                Console.WriteLine("Action: variant {0} ({1}) => variant {2} ({3}", e.OldTag, e.OldVariant, e.NewTag, e.NewVariant);
            };

            OldAction.Attack.OnAttackerUpdate += (sender, e) => {
                Console.WriteLine("Action / Attack / Attacker: {0} => {1}", e.OldValue, e.NewValue);
            };

            OldAction.Attack.OnDefenderUpdate += (sender, e) => {
                Console.WriteLine("Action / Attack / Defender: {0} => {1}", e.OldValue, e.NewValue);
            };

            OldAction.Attack.OnHitsUpdate += (sender, e) => {
                Console.WriteLine("Action / Attack / Hits:");

                if (e.OldValue.Count > 0) {
                    Console.WriteLine("Old Hits:");
                    foreach (var hit in e.OldValue) Console.WriteLine(HitToString(hit));
                } else {
                    Console.WriteLine("Old Hits:\n<empty>");
                }

                if (e.NewValue.Count > 0) {
                    Console.WriteLine("New Hits:");
                    foreach (var hit in e.NewValue) Console.WriteLine(HitToString(hit));
                } else {
                    Console.WriteLine("New Hits:\n<empty>");
                }
            };

            var action = new OldAction();

            StateReplayer.Replay<OldAction>(ref action, new ByteReader(new byte[] {
                // Set variant from to `Action::Attack`
                4, 0, 10, 1, 1,
                // Set attacker to 1
                8, 0, 2, 2, 1, 0, 10, 1, 1,
                // Set defender to 2
                8, 0, 2, 2, 1, 1, 10, 1, 2,
                // Add 4 hits with dummy values from 6 to 9, inclusive
                79, 0, 2, 2, 1, 2, 10, 72,
                    17, 2, 1, 0, 10, 1, 0, 18, 1, 0, 26, 1, 0, 34, 1, 0, 40, 12,
                    17, 2, 1, 0, 10, 1, 0, 18, 1, 0, 26, 1, 0, 34, 1, 0, 40, 14,
                    17, 2, 1, 0, 10, 1, 0, 18, 1, 0, 26, 1, 0, 34, 1, 0, 40, 16,
                    17, 2, 1, 0, 10, 1, 0, 18, 1, 0, 26, 1, 0, 34, 1, 0, 40, 18,
            }));

            Console.WriteLine("{0}", StateFactory.Deserialize<String>(new ByteReader(new byte[] { 2, 51, 51 })));
            Console.WriteLine("{0}", StateFactory.Deserialize<String>(new ByteReader(new byte[] { 2, 207, 128 })));
        }

        private static String InnerToString(Inner inner) {
            return inner != null ? string.Format("{{ Foo: {0}, Bar: {1} }}", inner.Foo, inner.Bar) : "<null>";
        }

        private static String HitToString(OldHit hit) {
            return hit != null ? string.Format("{{ Dummy: {0} }}", hit.Dummy) : "<null>";
        }
    }
}
