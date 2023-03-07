use crate::library::{Difficulty, Grid, Status};
use std::io::{self, Write};

pub fn run() {
    let mut board = Grid::new(match get_difficulty() {
        1 => Difficulty::Easy,
        2 => Difficulty::Medium,
        3 => Difficulty::Hard,
        _ => unreachable!(),
    });
    println!("\n\n");

    loop {
        game_loop(&mut board);

        if board.has_won() {
            println!("\n\n{board:?}\nCongratulations, you won :D");
        } else {
            println!("\n\n{board:?}\nGame over, you lost :<");
        }

        print!("\nPlay again? (y/N) ");
        std::io::stdout().flush().unwrap();

        if get_input()
            .to_ascii_lowercase()
            .chars()
            .next()
            .unwrap_or('n')
            == 'y'
        {
            continue;
        }

        break;
    }
}

fn game_loop(board: &mut Grid) {
    loop {
        println!("{board}");

        print!("\nEnter coords[row col]: ");
        io::stdout().flush().unwrap();
        let (row, col) = get_coords();

        print!("Enter action: ");
        io::stdout().flush().unwrap();
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
}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input
}

fn get_difficulty() -> u8 {
    print!(
        r#"Enter difficulty:
1. Easy (Default)
2. Medium
3. Hard

> "#
    );

    io::stdout().flush().unwrap();

    let mut number = get_input().trim().parse::<u8>().unwrap_or(1);

    if number > 3 {
        number = 3;
    }

    number
}

fn get_coords() -> (usize, usize) {
    let input_coord = get_input()
        .trim()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap_or_default())
        .collect::<Vec<usize>>();

    let mut row = input_coord.first().unwrap_or(&0).to_owned();
    let mut col = input_coord.get(1).unwrap_or(&0).to_owned();

    if row > 0 {
        row -= 1;
    }

    if col > 0 {
        col -= 1;
    }

    (row, col)
}

fn get_action() -> char {
    get_input()
        .to_ascii_lowercase()
        .chars()
        .next()
        .unwrap_or('F')
}
