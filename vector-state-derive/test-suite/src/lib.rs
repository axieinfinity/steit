#[allow(unused_imports)]
#[macro_use]
extern crate vector_state_derive;

#[cfg(test)]
mod tests {
    #[derive(State)]
    struct Pos2d {
        /* #[state(index = 0, default = "7")]
        x: i32,
        #[state(index = 1, default = "2")]
        y: i32, */
    }

    #[derive(State)]
    struct Pos3d {
        /* #[state(index = 7, default = "2")]
        pos: Pos2d,
        #[state(index = 0, default = "1")]
        z: i32, */
    }
}
