use rand::Rng;
use std::fmt;

fn rand_coord(range: &usize) -> (usize, usize) {
    let mut random = rand::thread_rng();

    (
        random.gen_range(0..range.to_owned()),
        random.gen_range(0..range.to_owned()),
    )
}

fn in_range(num1: isize, num2: isize, upper_limit: isize) -> bool {
    num1 >= 0 && num1 <= upper_limit && num2 >= 0 && num2 <= upper_limit
}

pub enum Status {
    Success,
    CannotOpenFlaggedCell,
    CannotOpenOpenedCell,
    CannotFlagOpenedCell,
    PositionOutOfBounds,
    GameWon,
    GameOver,
    FlagLimitReached,
}

#[derive(Clone, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    fn get_board_size(&self) -> usize {
        match self {
            Difficulty::Easy => 10,
            Difficulty::Medium => 20,
            Difficulty::Hard => 40,
        }
    }

    fn get_mine_count(&self) -> usize {
        match self {
            Difficulty::Easy => 10,
            Difficulty::Medium => 40,
            Difficulty::Hard => 160,
        }
    }
}

#[derive(Default, Clone, PartialEq)]
struct NonMined {
    mine_count: u8,
    is_open: bool,
    is_flagged: bool,
}

#[derive(Default, Clone, PartialEq)]
struct Mined {
    is_flagged: bool,
}

#[derive(Clone, PartialEq)]
enum Cell {
    NonMined(NonMined),
    Mined(Mined),
}

impl Cell {
    fn open(&mut self) -> (Status, bool) {
        match self {
            Cell::NonMined(i) => {
                if i.is_flagged {
                    (Status::CannotOpenFlaggedCell, false)
                } else if i.is_open {
                    (Status::CannotOpenOpenedCell, false)
                } else {
                    i.is_open = true;
                    (Status::Success, i.mine_count == 0)
                }
            }

            Cell::Mined(_) => (Status::GameOver, false),
        }
    }

    fn flag(&mut self) -> (Status, bool) {
        match self {
            Cell::NonMined(i) => {
                if i.is_open {
                    return (Status::CannotFlagOpenedCell, false);
                }

                if i.is_flagged {
                    i.is_flagged = false;
                    return (Status::Success, false);
                } else {
                    i.is_flagged = true;
                    return (Status::Success, true);
                }
            }

            Cell::Mined(i) => {
                if i.is_flagged {
                    i.is_flagged = false;
                    return (Status::Success, false);
                } else {
                    i.is_flagged = true;
                    return (Status::Success, true);
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Grid {
    board: Vec<Vec<Cell>>,
    difficulty: Difficulty,
    flags_left: usize,
}

impl Grid {
    fn init(&mut self) -> &mut Self {
        self.set_mines().set_count()
    }

    fn set_mines(&mut self) -> &mut Self {
        let board_size = self.difficulty.get_board_size();
        let mut mine_count = self.difficulty.get_mine_count();

        let non_mined_default = Cell::NonMined(NonMined::default());

        while mine_count > 0 {
            let (row, col) = rand_coord(&board_size);

            if self.board[row][col] == non_mined_default {
                self.board[row][col] = Cell::Mined(Mined::default());
                mine_count -= 1;
            }
        }

        self
    }

    fn set_count(&mut self) -> &mut Self {
        let board_size = self.board.len();

        for i in 0..board_size {
            for j in 0..board_size {
                if self.board[i][j] != Cell::Mined(Mined::default()) {
                    self.board[i][j] = Cell::NonMined(NonMined {
                        mine_count: self.get_mine_count(i, j, board_size),
                        is_open: false,
                        is_flagged: false,
                    })
                }
            }
        }

        self
    }

    fn get_mine_count(&mut self, i: usize, j: usize, board_size: usize) -> u8 {
        self.get_neighbor_cells_coords(i, j, board_size - 1)
            .into_iter()
            .filter(|c| self.board[c.0][c.1] == Cell::Mined(Mined::default()))
            .count() as u8
    }

    fn get_neighbor_cells_coords(
        &mut self,
        i: usize,
        j: usize,
        board_size: usize,
    ) -> Vec<(usize, usize)> {
        let mut neighbor_cells_coords = Vec::<(usize, usize)>::new();

        if in_range((i as isize) - 1, (j as isize) - 1, board_size as isize) {
            neighbor_cells_coords.push((i - 1, j - 1));
        }

        if in_range((i as isize) - 1, j as isize, board_size as isize) {
            neighbor_cells_coords.push((i - 1, j));
        }

        if in_range((i as isize) - 1, (j as isize) + 1, board_size as isize) {
            neighbor_cells_coords.push((i - 1, j + 1));
        }

        if in_range(i as isize, (j as isize) - 1, board_size as isize) {
            neighbor_cells_coords.push((i, j - 1));
        }

        if in_range(i as isize, (j as isize) + 1, board_size as isize) {
            neighbor_cells_coords.push((i, j + 1));
        }

        if in_range((i as isize) + 1, (j as isize) - 1, board_size as isize) {
            neighbor_cells_coords.push((i + 1, j - 1));
        }

        if in_range((i as isize) + 1, j as isize, board_size as isize) {
            neighbor_cells_coords.push((i + 1, j));
        }

        if in_range((i as isize) + 1, (j as isize) + 1, board_size as isize) {
            neighbor_cells_coords.push((i + 1, j + 1));
        }

        neighbor_cells_coords
    }

    fn has_won_helper(&self) -> bool {
        for i in 0..self.difficulty.get_board_size() {
            for j in 0..self.difficulty.get_board_size() {
                if let Cell::NonMined(k) = &self.board[i][j] {
                    if k.is_flagged {
                        return false;
                    }
                }

                if let Cell::Mined(k) = &self.board[i][j] {
                    if !k.is_flagged {
                        return false;
                    }
                }
            }
        }

        true
    }
}

impl Grid {
    pub fn new(difficulty: Difficulty) -> Self {
        let board_size = difficulty.get_board_size();
        let mine_count = difficulty.get_mine_count();

        Self {
            board: vec![vec![Cell::NonMined(NonMined::default()); board_size]; board_size],
            difficulty,
            flags_left: mine_count,
        }
        .init()
        .to_owned()
    }

    pub fn open(&mut self, row: usize, col: usize) -> Status {
        let board_size_max = self.difficulty.get_board_size() - 1;

        if row > board_size_max || col > board_size_max {
            return Status::PositionOutOfBounds;
        }

        let status = self.board[row][col].open();

        if status.1 {
            let neighbor_cells_vec =
                self.get_neighbor_cells_coords(row, col, self.difficulty.get_board_size() - 1);

            for (r, c) in neighbor_cells_vec {
                self.open(r, c);
            }
        }

        if self.has_won_helper() {
            return Status::GameWon;
        }

        status.0
    }

    pub fn flag(&mut self, row: usize, col: usize) -> Status {
        if self.flags_left == 0 {
            return Status::FlagLimitReached;
        }

        let board_size_max = self.difficulty.get_board_size() - 1;
        if row > board_size_max || col > board_size_max {
            return Status::PositionOutOfBounds;
        }

        let status = self.board[row][col].flag();

        if status.1 {
            self.flags_left -= 1;
        } else {
            self.flags_left += 1;
        }

        if self.has_won_helper() {
            return Status::GameWon;
        }

        status.0
    }

    pub fn has_won(&self) -> bool {
        self.has_won_helper()
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board_str = String::new();
        let board_size = self.difficulty.get_board_size();

        for _ in 0..board_size {
            board_str += "|---"
        }

        board_str += "|\n";

        for i in 0..board_size {
            for j in 0..board_size {
                let temp: String;

                match &self.board[i][j] {
                    Cell::NonMined(i) => {
                        if i.mine_count == 0 {
                            temp = "|   ".to_string();
                        } else {
                            temp = format!("| {} ", i.mine_count)
                        }
                    }
                    Cell::Mined(i) => {
                        if i.is_flagged {
                            temp = "| F ".to_string();
                        } else {
                            temp = "| M ".to_string();
                        }
                    }
                }

                board_str += temp.as_str();
            }

            board_str += "|\n";

            for _ in 0..board_size {
                board_str += "|---"
            }

            board_str += "|\n";
        }

        write!(f, "{board_str}")
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board_str = String::from("    ");
        let board_size = self.difficulty.get_board_size();

        for i in 0..board_size {
            board_str += format!("{i}   ").as_str();
        }

        board_str += "\n  ";

        for _ in 0..board_size {
            board_str += "|---"
        }

        board_str += "|\n";

        for i in 0..board_size {
            board_str += format!("{i} ").as_str();

            for j in 0..board_size {
                let temp: String;

                match &self.board[i][j] {
                    Cell::NonMined(i) => {
                        if i.is_open {
                            if i.mine_count == 0 {
                                temp = "|   ".to_string();
                            } else {
                                temp = format!("| {} ", i.mine_count)
                            }
                        } else if i.is_flagged {
                            temp = "| F ".to_string()
                        } else {
                            temp = "| • ".to_string()
                        }
                    }
                    Cell::Mined(i) => {
                        if i.is_flagged {
                            temp = "| F ".to_string()
                        } else {
                            temp = "| • ".to_string()
                        }
                    }
                }

                board_str += temp.as_str();
            }

            board_str += "|\n  ";

            for _ in 0..board_size {
                board_str += "|---"
            }

            board_str += "|\n";
        }

        write!(f, "{board_str}Remaining flags: {}", self.flags_left)
    }
}
