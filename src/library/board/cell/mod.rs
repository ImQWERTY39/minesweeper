use crate::Status;

#[derive(Default, Clone, PartialEq)]
pub(super) struct NonMined {
    pub(super) mine_count: u8,
    pub(super) is_open: bool,
    pub(super) is_flagged: bool,
}

#[derive(Default, Clone, PartialEq)]
pub(super) struct Mined {
    pub(super) is_flagged: bool,
}

#[derive(Clone, PartialEq)]
pub(super) enum Cell {
    NonMined(NonMined),
    Mined(Mined),
}

impl Cell {
    pub(super) fn open(&mut self) -> (Status, bool) {
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

    pub(super) fn flag(&mut self) -> (Status, bool) {
        match self {
            Cell::NonMined(i) => {
                if i.is_open {
                    return (Status::CannotFlagOpenedCell, false);
                }

                if i.is_flagged {
                    i.is_flagged = false;
                    (Status::Success, false)
                } else {
                    i.is_flagged = true;
                    (Status::Success, true)
                }
            }

            Cell::Mined(i) => {
                if i.is_flagged {
                    i.is_flagged = false;
                    (Status::Success, false)
                } else {
                    i.is_flagged = true;
                    (Status::Success, true)
                }
            }
        }
    }
}
