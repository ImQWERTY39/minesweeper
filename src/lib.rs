use rand::Rng;

fn rand_coord(range: &usize) -> (usize, usize) {
    let mut random = rand::thread_rng();

    (
        random.gen_range(0..range.to_owned()),
        random.gen_range(0..range.to_owned()),
    )
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
        self
    }
}

impl Grid {
    pub fn new(difficulty: Difficulty) -> Self {
        let mine_count = difficulty.get_mine_count();

        Self {
            board: Vec::new(),
            difficulty,
            flags_left: mine_count,
        }
        .init()
        .to_owned()
    }
}
