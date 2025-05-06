use std::fmt::{self, Display};

pub enum State {
    Win,
    Loss,
    Tie,
}

impl Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Win => write!(f, "You Win!"),
            Self::Loss => write!(f, "You Lose!"),
            Self::Tie => write!(f, "It's a Tie!"),
        }
    }
}
