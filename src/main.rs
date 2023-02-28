use minesweeper::{Difficulty, Grid, Status};
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

fn game_loop(board: &mut Grid) {
    loop {
        println!("{board}");

        print!("\nEnter coords[row col]: ");
        stdout().flush().unwrap();
        let (row, col) = get_coords();

        print!("Enter action: ");
        stdout().flush().unwrap();
        let action = get_action();

        if action == 'o' {
            match board.open(row, col) {
                Status::Success => {}
                Status::CannotOpenFlaggedCell => println!("Cannot open a flagged cell"),
                Status::CannotOpenOpenedCell => println!("Cell already opened"),
                Status::PositionOutOfBounds => println!("Position out of bounds"),
                Status::GameOver | Status::GameWon => break,
                _ => unreachable!(),
            }
        } else {
            match board.flag(row, col) {
                Status::Success => {}
                Status::CannotFlagOpenedCell => println!("Cell already opened "),
                Status::PositionOutOfBounds => println!("Position out of bounds"),
                Status::FlagLimitReached => println!("Flag limit reached"),
                Status::GameWon => break,
                _ => unreachable!(),
            }
        }

        println!("\n");
    }

    if board.has_won() {
        println!("\n\n{board:?}\nCongratulations, you won :D");
    } else {
        println!("\n\n{board:?}\nGame over LMAO");
    }
}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input
}

fn get_coords() -> (usize, usize) {
    let input_coord = get_input()
        .trim()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap_or_default())
        .collect::<Vec<usize>>();

    let row = input_coord.first().unwrap_or(&0) - 1;
    let col = input_coord.get(1).unwrap_or(&0) - 1;

    (row, col)
}

fn get_action() -> char {
    get_input()
        .to_ascii_lowercase()
        .chars()
        .next()
        .unwrap_or('F')
}
