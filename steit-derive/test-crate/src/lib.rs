#[allow(unused_imports)]
#[macro_use]
extern crate steit_derive;

#[cfg(test)]
mod tests {
    use steit::path::Path;

    /* #[derive(State)]
    struct Good {
        path: Path,
    }

    #[derive(State)]
    union Test {}

    #[derive(State)]
    struct Test2;

    #[derive(State)]
    struct Test3 {}

    #[derive(State)]
    struct Test4 {
        path: Path,
        path2: Path,
    }

    #[derive(State)]
    struct Test5 {
        path: Path,
        #[state(tag = "0")]
        #[state(tag = "0")]
        x: i32,
    }

    #[derive(State)]
    struct Test6 {
        path: Path,
        #[state(tag = 0, default = 0)]
        good: Good,
    }

    #[derive(State)]
    struct Test7 {
        path: Path,
        #[state(tag = 0)]
        #[state(default = "10")]
        x: i32,
    }

    #[derive(State)]
    struct Test8 {
        path: Path,
        #[state(tag = 0)]
        x: i32,
        #[state(tag = 0)]
        y: i32,
    }

    #[derive(State)]
    struct Test9 {
        path: Path,
        #[state(what = 0)]
        x: i32,
    }

    #[derive(State)]
    struct Test10 {
        path: Path,
        #[state(0)]
        x: i32,
    }

    #[derive(State)]
    struct Pos2d {
        path: Path,
        #[state(tag = 0, default = "7")]
        x: i32,
        #[state(tag = 1, default = "2")]
        y: i32,
    }

    #[derive(State)]
    struct Pos3d {
        path: Path,
        #[state(tag = 7)]
        pos: Pos2d,
        #[state(tag = 0, default = "1")]
        z: i32,
    }

    #[derive(State)]
    enum Pos {
        TwoDim(#[state(tag = 0)] Pos2d, Path),
        ThreeDim(#[state(tag = 0)] Pos3d, Path),
    }

    #[derive(State)]
    struct Character {
        path: Path,
        #[state(tag = 2)]
        pos: Pos,
    }

    #[derive(Debug, State)]
    struct r#Why(
        Path,
        #[state(tag = 0, default = "7")] i32,
        #[state(tag = 1, default = "10")] i32,
    ); */

    #[derive(Debug, PartialEq, State)]
    struct Point(
        Path,
        #[state(tag = 0, default = "5")] i32,
        #[state(tag = 1, default = "10")] i32,
    );

    #[derive(Debug, PartialEq, State)]
    struct Segment(Path, #[state(tag = 0)] Point, #[state(tag = 1)] Point);

    use std::fmt;

    use steit::{de::Deserialize, ser::Serialize};

    fn debug<O: Serialize>(object: &O) {
        let mut writer = Vec::new();
        object.serialize(&mut writer).unwrap();
        println!("{:?}", writer);
    }

    fn check<O: fmt::Debug + PartialEq + Serialize + Deserialize>(object: &O, r#new: &mut O) {
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
        let mut point_a = Point::new(Path::root());
        point_a.1 = 100;
        println!("{:?}, size = {}", point_a, point_a.size());
        debug(&point_a);

        println!("f#1 = {}", point_a.1);
        println!("f#2 = {}", point_a.2);
        point_a.set_1(137);
        println!("f#1 = {} (changed)", point_a.1);

        let mut point_b = Point::new(Path::root());
        point_b.2 = 200;
        println!("{:?}, size = {}", point_b, point_b.size());
        debug(&point_b);

        let mut segment = Segment::new(Path::root());
        segment.1 = Point(segment.0.child(0), point_a.1, point_a.2);
        segment.2 = Point(segment.0.child(1), point_b.1, point_b.2);
        println!("{:?}, size = {}", segment, segment.size());
        println!();
        debug(&segment);
        check(&segment, &mut Segment::new(Path::root()));

        segment.set_1_with(Point::new);
        segment.set_2_with(Point::new);
        debug(&segment);
        check(&segment, &mut Segment::new(Path::root()));
    }
}
