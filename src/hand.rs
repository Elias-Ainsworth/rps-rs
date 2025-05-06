use std::fmt::{self, Display};

use rand::Rng;

#[derive(Debug, PartialEq, Eq)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Rock => write!(f, "rock"),
            Self::Paper => write!(f, "paper"),
            Self::Scissors => write!(f, "scissors"),
        }
    }
}

impl Hand {
    pub fn random() -> Self {
        match rand::rng().random_range(1..=3) {
            1 => Self::Rock,
            2 => Self::Paper,
            _ => Self::Scissors,
        }
    }
    pub fn from_input(input: &str) -> Option<Self> {
        match input.trim().to_lowercase().as_str() {
            "rock" => Some(Self::Rock),
            "paper" => Some(Self::Paper),
            "scissors" => Some(Self::Scissors),
            _ => None,
        }
    }
    pub fn beats(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissors)
                | (Self::Scissors, Self::Paper)
                | (Self::Paper, Self::Rock)
        )
    }
}
