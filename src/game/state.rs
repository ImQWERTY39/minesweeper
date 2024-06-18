#[derive(Clone, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn get_board_size(&self) -> usize {
        match self {
            Difficulty::Easy => 10,
            Difficulty::Medium => 20,
            Difficulty::Hard => 40,
        }
    }

    pub fn get_mine_count(&self) -> usize {
        match self {
            Difficulty::Easy => 10,
            Difficulty::Medium => 40,
            Difficulty::Hard => 160,
        }
    }
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
