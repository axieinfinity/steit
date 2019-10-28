#[allow(unused_imports)]
#[macro_use]
extern crate vector_state_derive;

#[cfg(test)]
mod tests {
    use vector_state::path::Path;

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
    } */

    #[derive(Debug, State)]
    struct Point(
        Path,
        #[state(tag = 0, default = "7")] i32,
        #[state(tag = 1, default = "10")] i32,
    );

    #[test]
    fn test() {
        println!("{:?}", Point::new(Path::new()));
    }
}
