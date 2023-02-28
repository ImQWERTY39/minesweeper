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
