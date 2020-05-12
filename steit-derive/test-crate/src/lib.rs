#[cfg(test)]
mod tests {
    use std::path::Path;

    use steit::{
        gen::{
            generators::{CSharpGenerator, CSharpSetting},
            Generator, Setting,
        },
        log::{loggers::WriterLogger, LogEntry},
        rt::Runtime,
        ser::Serialize,
        state::State,
        steit_derive,
        types::{List, Map},
    };

    #[steit_derive(Debug, State)]
    enum Foo {
        #[steit(tag = 0, default)]
        FirstCase {
            #[steit(tag = 0)]
            counter: i32,
            #[steit(tag = 1)]
            enabled: bool,
        },
        #[steit(tag = 1)]
        SecondCase {
            #[steit(tag = 0)]
            counter: i32,
            #[steit(tag = 1)]
            enabled: bool,
        },
        #[steit(tag = 2)]
        ThirdCase {},
        #[steit(tag = 3)]
        FourthCase(),
        #[steit(tag = 4)]
        FifthCase,
    }

    #[steit_derive(Debug, Serialize, Deserialize)]
    struct Bar {
        #[steit(tag = 0)]
        counter: i32,
        #[steit(tag = 1)]
        enabled: bool,
    }

    #[steit_derive(Debug, State)]
    enum Maybe<T> {
        #[steit(tag = 0, default)]
        None,
        #[steit(tag = 1)]
        Some(#[steit(tag = 0)] T),
    }

    #[steit_derive(Debug, State)]
    enum Animal<A, B, C> {
        #[steit(tag = 0, default)]
        Alligator(#[steit(tag = 0)] Maybe<A>),
        #[steit(tag = 1)]
        Bear(#[steit(tag = 0)] i32, #[steit(tag = 1)] B),
        #[steit(tag = 2)]
        Cat(#[steit(tag = 0)] i32, #[steit(tag = 1)] Maybe<Maybe<C>>),
        #[steit(tag = 3)]
        Donkey(#[steit(tag = 0)] Maybe<u8>),
        #[steit(tag = 4)]
        Elephant,
    }

    #[steit_derive(Debug, Serialize, Deserialize)]
    pub struct Sure<T>(#[steit(tag = 0)] T);

    #[steit_derive(Debug, Serialize, Deserialize)]
    pub enum ActionsOr<T> {
        #[steit(tag = 0, default)]
        Actions(#[steit(tag = 0)] Vec<Action>),
        #[steit(tag = 1)]
        Value(#[steit(tag = 0)] T),
    }

    #[steit_derive(Debug, State)]
    pub enum OldAction {
        #[steit(tag = 0, default)]
        Raw {
            #[steit(tag = 0)]
            log_entries: List<u8>,
        },
        #[steit(tag = 1)]
        Attack {
            #[steit(tag = 0)]
            attacker: u8,
            #[steit(tag = 1)]
            defender: u8,
            #[steit(tag = 2)]
            hits: List<OldHit>,
        },
    }

    #[steit_derive(Debug, State)]
    pub struct OldHit {
        #[steit(tag = 0)]
        before_attacking: Box<OldAction>,
        #[steit(tag = 1)]
        before_damaging: Box<OldAction>,
        #[steit(tag = 2)]
        damaging: Box<OldAction>,
        #[steit(tag = 3)]
        after_damaging: Box<OldAction>,
        #[steit(tag = 4)]
        after_attacking: Box<OldAction>,
        #[steit(tag = 5)]
        dummy: i32,
    }

    #[steit_derive(Debug, Serialize, Deserialize)]
    pub enum Action {
        #[steit(tag = 0, default)]
        Raw {},
        #[steit(tag = 1)]
        CardDraw {
            #[steit(tag = 0)]
            player_index: u16,
            #[steit(tag = 1)]
            draw: Vec<Action>,
            #[steit(tag = 2)]
            post_draw: Vec<Action>,
        },
        #[steit(tag = 2)]
        CardDiscard {},
        #[steit(tag = 3)]
        Attack {
            #[steit(tag = 0)]
            attacker_index: u16,
            #[steit(tag = 1)]
            card_id: u32, // TODO: Make this optional
            #[steit(tag = 2)]
            before_attacks: Vec<Action>,
            #[steit(tag = 3)]
            attacks: ActionsOr<Vec<ActionsOr<Attack>>>,
            #[steit(tag = 4)]
            after_attacks: Vec<Action>,
        },
        #[steit(tag = 4)]
        Skill {
            #[steit(tag = 0)]
            caster_index: u16,
            #[steit(tag = 1)]
            card_id: u32,
            #[steit(tag = 2)]
            before_skills: Vec<Action>,
            #[steit(tag = 3)]
            skills: ActionsOr<Vec<ActionsOr<Skill>>>,
            #[steit(tag = 4)]
            after_skills: Vec<Action>,
        },
    }

    #[steit_derive(Debug, Serialize, Deserialize)]
    pub struct Attack {
        #[steit(tag = 0)]
        pub target_index: u16,
        #[steit(tag = 1)]
        pub before_hits: Vec<Action>,
        #[steit(tag = 2)]
        pub hits: ActionsOr<Vec<ActionsOr<Hit>>>,
        #[steit(tag = 3)]
        pub after_hits: Vec<Action>,
    }

    #[steit_derive(Debug, Serialize, Deserialize)]
    pub struct Hit {
        #[steit(tag = 0)]
        pub is_miss: bool,
        #[steit(tag = 1)]
        pub pre_damage: Vec<Action>,
        #[steit(tag = 2)]
        pub damage: Vec<Action>,
        #[steit(tag = 3)]
        pub post_damage: Vec<Action>,
    }

    #[steit_derive(Debug, Serialize, Deserialize)]
    pub struct Skill {
        #[steit(tag = 0)]
        pub target_index: u16,
        #[steit(tag = 1)]
        pub pre_cast: Vec<Action>,
        #[steit(tag = 2)]
        pub cast: Vec<Action>,
        #[steit(tag = 3)]
        pub post_cast: Vec<Sure<Action>>,
    }

    #[steit_derive(State)]
    struct Hello {
        #[steit(tag = 0)]
        numbers: List<i32>,
        #[steit(tag = 1, no_state)]
        others: Vec<i32>,
    }

    #[steit_derive(Debug, State)]
    struct Outer {
        #[steit(tag = 0)]
        foo: i32,
        #[steit(tag = 1)]
        bar: bool,
        #[steit(tag = 2)]
        inner: Inner,
    }

    #[steit_derive(Debug, State)]
    struct Inner {
        #[steit(tag = 0)]
        foo: i32,
        #[steit(tag = 1)]
        bar: bool,
    }

    #[steit_derive(Debug, State)]
    enum Multicase {
        #[steit(tag = 0, default)]
        FirstCase {
            #[steit(tag = 0)]
            counter: i32,
            #[steit(tag = 1)]
            enabled: bool,
        },
        #[steit(tag = 1)]
        SecondCase {
            #[steit(tag = 0)]
            counter: i32,
            #[steit(tag = 1)]
            enabled: bool,
        },
    }

    #[steit_derive(Debug, State)]
    #[steit(no_hash)]
    struct Woof {
        #[steit(tag = 0)]
        map: Map<u16, i32>,
    }

    #[test]
    fn test() {
        let generator = CSharpGenerator;

        let setting = CSharpSetting::new("Steit.State");
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../steit-csharp/src/State");
        let setting = Setting::new(&path, false, setting);

        generator.generate::<LogEntry>(&setting).unwrap();

        let setting = CSharpSetting::new("Just.To.Test");
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../steit-csharp/tests");
        let setting = Setting::new(&path, true, setting);

        generator.generate::<OldAction>(&setting).unwrap();
        generator.generate::<Action>(&setting).unwrap();
        generator.generate::<Hello>(&setting).unwrap();
        generator.generate::<Outer>(&setting).unwrap();
        generator.generate::<Multicase>(&setting).unwrap();
        generator.generate::<Woof>(&setting).unwrap();

        println!("\nHELLO!");

        let runtime = Runtime::with_logger(WriterLogger::stdout());
        let mut hello = Hello::empty(runtime);

        hello
            .set_numbers_with(|runtime| {
                let mut list = List::new(runtime);
                list.push(1);
                list.push(2);
                list.push(1337);
                list
            })
            .set_others(vec![-1, -2, 1337]);

        let mut bytes = Vec::new();
        hello.serialize(&mut bytes).unwrap();
        println!("serialized: {:?}", bytes);

        println!("\nOUTER");

        let runtime = Runtime::with_logger(WriterLogger::stdout());
        let mut outer = Outer::empty(runtime);

        outer.set_foo(127).set_bar(true).set_inner_with(|runtime| {
            let mut inner = Inner::empty(runtime);
            inner.set_foo(22).set_bar(true);
            inner
        });

        outer.inner.set_foo(160);
        outer.set_inner_with(Inner::empty);

        println!("{:?}", outer);

        let runtime = Runtime::with_logger(WriterLogger::stdout());
        outer.set_runtime(runtime.nested(10));

        println!("{:?}", outer);

        println!("\nENUM");

        let runtime = Runtime::with_logger(WriterLogger::stdout());
        let mut multicase = Multicase::empty(runtime);

        multicase.set_second_case_counter(68);

        println!("{:?}", multicase);

        let runtime = Runtime::with_logger(WriterLogger::stdout());
        multicase.set_runtime(runtime.nested(10));

        println!("{:?}", multicase);

        println!("\nLIST #1");

        let runtime = Runtime::with_logger(WriterLogger::stdout());
        let mut list = List::new(runtime);

        list.push_with(|runtime| {
            let mut inner = Inner::empty(runtime);
            inner.set_foo(6);
            inner
        });

        list.push_with(|runtime| {
            let mut inner = Inner::empty(runtime);
            inner.set_foo(77).set_bar(true);
            inner
        });

        list.push_with(Inner::empty);
        list.get_mut(1).unwrap().set_foo(68);
        list.swap_remove(0);

        println!("\nLIST #2");

        let runtime = Runtime::with_logger(WriterLogger::stdout());
        let mut list = List::new(runtime);

        list.push(10i8);
        list.push(11);
        list.push(0);
        list.swap_remove(1);

        println!("\nMAP #1");

        let runtime = Runtime::with_logger(WriterLogger::stdout());
        let mut map = Map::new(runtime);

        map.insert_with(5u16, |runtime| {
            let mut inner = Inner::empty(runtime);
            inner.set_foo(6);
            inner
        });

        map.insert_with(1, |runtime| {
            let mut inner = Inner::empty(runtime);
            inner.set_foo(77).set_bar(true);
            inner
        });

        map.insert_with(0, Inner::empty);
        map.get_mut(&1).unwrap().set_foo(68);
        map.remove(&0);

        println!("\nMAP #2");

        let runtime = Runtime::with_logger(WriterLogger::stdout());
        let mut map = Map::new(runtime);

        map.insert(1u32, 10i8);
        map.insert(3, 11);
        map.insert(7, 0);
        map.remove(&1);

        println!("\nACTION!");

        let runtime = Runtime::with_logger(WriterLogger::stdout());
        let mut action = OldAction::empty(runtime);

        action.set_attack_attacker(1);
        action.set_attack_defender(2);

        action.set_attack_hits_with(|runtime| {
            let mut hits = List::new(runtime);

            for dummy in 6..=9 {
                hits.push_with(|runtime| {
                    let mut hit = OldHit::empty(runtime);
                    hit.set_dummy(dummy);
                    hit
                })
            }

            hits
        });

        let mut b1 = Bar::empty();
        b1.counter = 10;
        b1.enabled = true;

        let mut b2 = Bar::empty();

        assert_ne!(b1, b2);

        b2.counter = 10;
        b2.enabled = true;

        assert_eq!(b1, b2);
    }
}
