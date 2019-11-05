#[cfg(test)]
mod tests {
    use std::fmt;

    use steit::{Deserialize, RawEntryKind, Runtime, Serialize, State};

    /* #[derive(State)]
    struct Good {
        runtime: Runtime,
    }

    #[derive(State)]
    union Test {}

    #[derive(State)]
    struct Test2;

    #[derive(State)]
    struct Test3 {}

    #[derive(State)]
    struct Test4 {
        runtime: Runtime,
        runtime2: Runtime,
    }

    #[derive(State)]
    struct Test5 {
        runtime: Runtime,
        #[steit(tag = "0")]
        #[steit(tag = "0")]
        x: i32,
    }

    #[derive(State)]
    struct Test6 {
        runtime: Runtime,
        #[steit(tag = 0, default = 0)]
        good: Good,
    }

    #[derive(State)]
    struct Test7 {
        runtime: Runtime,
        #[steit(tag = 0)]
        #[steit(default = "10")]
        x: i32,
    }

    #[derive(State)]
    struct Test8 {
        runtime: Runtime,
        #[steit(tag = 0)]
        x: i32,
        #[steit(tag = 0)]
        y: i32,
    }

    #[derive(State)]
    struct Test9 {
        runtime: Runtime,
        #[steit(what = 0)]
        x: i32,
    }

    #[derive(State)]
    struct Test10 {
        runtime: Runtime,
        #[steit(0)]
        x: i32,
    }

    #[derive(State)]
    struct Pos2d {
        runtime: Runtime,
        #[steit(tag = 0, default = "7")]
        x: i32,
        #[steit(tag = 1, default = "2")]
        y: i32,
    }

    #[derive(State)]
    struct Pos3d {
        runtime: Runtime,
        #[steit(tag = 7)]
        pos: Pos2d,
        #[steit(tag = 0, default = "1")]
        z: i32,
    }

    #[derive(State)]
    enum Pos {
        TwoDim(#[steit(tag = 0)] Pos2d, Runtime),
        ThreeDim(#[steit(tag = 0)] Pos3d, Runtime),
    }

    #[derive(State)]
    struct Character {
        runtime: Runtime,
        #[steit(tag = 2)]
        pos: Pos,
    }

    #[derive(Debug, State)]
    struct r#Why(
        Runtime,
        #[steit(tag = 0, default = "7")] i32,
        #[steit(tag = 1, default = "10")] i32,
    );

    #[derive(Serialize, Deserialize)]
    struct What {
        #[steit(tag = 0)]
        x: i32,
    }

    #[derive(Debug, PartialEq, State)]
    struct Point(
        Runtime,
        #[steit(tag = 2, default = "5")] i32,
        #[steit(tag = 3, default = "10")] i32,
    );

    #[derive(Debug, PartialEq, State)]
    struct Segment(Runtime, #[steit(tag = 0)] Point, #[steit(tag = 1)] Point);

    use std::fmt;

    use steit::{de::Deserialize, ser::Serialize};

    fn debug(object: &impl Serialize) {
        let mut writer = Vec::new();
        object.serialize(&mut writer).unwrap();
        println!("{:?}", writer);
    }

    fn check(object: &impl PartialEq + fmt::Debug + Serialize + Deserialize, r#new: &mut O) {
        let mut bytes = Vec::new();
        object.serialize(&mut bytes).unwrap();
        r#new.deserialize(&mut &*bytes).unwrap();
        println!("original:      {:?}", object);
        println!("over the wire: {:?}", r#new);
        println!("bytes: {:?}", bytes);
        println!();
        assert_eq!(r#new, object);
    }

    #[test]
    fn test() {
        let mut point_a = Point::new(Runtime::new());
        point_a.1 = 100;
        println!("{:?}, size = {}", point_a, point_a.size());
        debug(&point_a);

        println!("f#1 = {}", point_a.1);
        println!("f#2 = {}", point_a.2);
        point_a.set_1(137);
        println!("f#1 = {} (changed)", point_a.1);

        let mut point_b = Point::new(Runtime::new());
        point_b.2 = 200;
        println!("{:?}, size = {}", point_b, point_b.size());
        debug(&point_b);

        let mut segment = Segment::new(Runtime::new());
        segment.1 = Point(segment.0.nested(0), point_a.1, point_a.2);
        segment.2 = Point(segment.0.nested(1), point_b.1, point_b.2);
        println!("{:?}, size = {}", segment, segment.size());
        println!();
        debug(&segment);
        check(&segment, &mut Segment::new(Runtime::new()));

        segment.set_1_with(Point::new);
        segment.set_2_with(Point::new);
        debug(&segment);
        check(&segment, &mut Segment::new(Runtime::new()));
    } */

    /* #[derive(Debug, State)]
    struct Well {
        runtime: Runtime,
        #[steit(tag = 0)]
        well: i32,
    }

    #[derive(Debug, State)]
    enum Test {
        #[steit(tag = 27)]
        Foo {
            runtime: Runtime,
            #[steit(tag = 4)]
            foo: i32,
        },
        #[steit(tag = 28)]
        Bar {
            runtime: Runtime,
            #[steit(tag = 5)]
            bar: u16,
        },
        #[steit(tag = 29)]
        Qux { runtime: Runtime },
    }

    struct Qux(i32);

    fn back_and_forth(value: &mut (impl fmt::Debug + Serialize + Deserialize)) {
        let mut bytes = Vec::new();
        value.serialize(&mut bytes).unwrap();
        println!("{:?}", value);
        println!("{:?}", bytes);
        let mut bytes: &[u8] = &[3u8, 28, 40, 100];
        value.deserialize(&mut bytes).unwrap();
        println!("{:?}", value);
        println!();
    }

    #[test]
    fn test() {
        let mut test = Test::new_foo(Runtime::new());

        println!("size: {}", test.size());
        println!();

        test.set_foo_foo(20);
        back_and_forth(&mut test);

        test.set_bar_bar(10);
        back_and_forth(&mut test);

        test.set_foo_foo(50);
        back_and_forth(&mut test);

        let mut reader: &[u8] = &[6];

        test.process_log(&mut [28, 5].iter(), &RawEntryKind::Update, &mut reader)
            .unwrap();

        println!("{:?}", test);

        let mut reader: &[u8] = &[3, 27, 32, 1];

        test.process_log(&mut [].iter(), &RawEntryKind::Update, &mut reader)
            .unwrap();

        println!("{:?}", test);

        let mut well = Well {
            runtime: Runtime::new(),
            well: 27,
        };

        println!("{:?}", well);

        let mut reader: &[u8] = &[78];

        well.process_log(&mut [0].iter(), &RawEntryKind::Update, &mut reader)
            .unwrap();

        println!("{:?}", well);

        let mut reader: &[u8] = &[2, 0, 63];

        well.process_log(&mut [].iter(), &RawEntryKind::Update, &mut reader)
            .unwrap();

        println!("{:?}", well);
    } */

    #[steit::serialize2]
    enum Test {
        #[steit(tag = 27)]
        Foo {
            #[steit(tag = 4)]
            foo: i32,
        },
        #[steit(tag = 28)]
        Bar {
            #[steit(tag = 5)]
            bar: u16,
        },
        #[steit(tag = 29)]
        Qux,
    }

    #[test]
    fn test2() {
        let _ = Test::Foo { foo: -1 };
    }
}
