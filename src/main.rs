use minesweeper::{Difficulty, Grid};

fn main() {
    let board = Grid::new(Difficulty::Medium);

    println!("{board:?}");
}
