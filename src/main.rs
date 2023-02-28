use minesweeper::{game_loop, get_input, Difficulty, Grid};
use std::io::{stdout, Write};

fn main() {
    print!(
        r#"Enter difficulty:
1. Easy (Default)
2. Medium
3. Hard

> "#
    );

    stdout().flush().unwrap();

    let mut board = Grid::new(match get_input().trim().parse::<u8>().unwrap_or(1) {
        2 => Difficulty::Medium,
        3 => Difficulty::Hard,
        _ => Difficulty::Easy,
    });

    println!("\n\n");
    game_loop(&mut board);
}
