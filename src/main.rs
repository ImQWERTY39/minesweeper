use minesweeper::{Difficulty, Grid, Status};
use std::io::{stdout, Write};

fn main() {
    let mut board = Grid::new(Difficulty::Easy).test();
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
                Status::CannotOpenFlaggedCell => println!("Cannot open a flagged cell\n\n"),
                Status::CannotOpenOpenedCell => println!("Cell already opened\n\n"),
                Status::PositionOutOfBounds => println!("Position out of bounds\n\n"),
                Status::GameOver => break,
                _ => unreachable!(),
            }
        } else {
            match board.flag(row, col) {
                Status::Success => {}
                Status::CannotFlagOpenedCell => println!("Cell already opened \n\n"),
                Status::PositionOutOfBounds => println!("Position out of bounds\n\n"),
                Status::FlagLimitReached => println!("Flag limit reached\n\n"),
                Status::GameWon => break,
                _ => unreachable!(),
            }
        }
    }

    if board.has_won() {
        println!("{board:?}\n\nCongratulations, you won :D");
    } else {
        println!("{board:?}\n\nGame over LMAO");
    }
}

fn get_coords() -> (usize, usize) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let input_coord = input
        .trim()
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap_or_default())
        .collect::<Vec<usize>>();

    let row = input_coord.get(0).unwrap_or(&0).to_owned();
    let col = input_coord.get(1).unwrap_or(&0).to_owned();

    (row, col)
}

fn get_action() -> char {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input.to_ascii_lowercase().chars().next().unwrap_or('F')
}
