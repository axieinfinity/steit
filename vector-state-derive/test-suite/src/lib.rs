#[allow(unused_imports)]
#[macro_use]
extern crate vector_state_derive;

#[cfg(test)]
mod tests {
    use vector_state::path::Path;

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
}
