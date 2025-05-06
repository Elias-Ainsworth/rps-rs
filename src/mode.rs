use std::io::{self, Write};

use colored::Colorize;

use crate::{Hand, State};

pub enum Mode {
    Single,
    BestOf3,
    BestOf5,
    BestOf7,
}

impl Mode {
    pub fn max_rounds(&self) -> u8 {
        match self {
            Self::Single => 1,
            Self::BestOf3 => 3,
            Self::BestOf5 => 5,
            Self::BestOf7 => 7,
        }
    }
    pub fn required_wins(&self) -> u8 {
        (self.max_rounds() / 2) + 1
    }
    pub fn final_state(
        &self,
        player_wins: u8,
        computer_wins: u8,
        rounds_played: u8,
    ) -> Option<State> {
        let max = self.max_rounds();
        let required = self.required_wins();

        if player_wins >= required {
            Some(State::Win)
        } else if computer_wins >= required {
            Some(State::Loss)
        } else if rounds_played >= max {
            if player_wins > computer_wins {
                Some(State::Win)
            } else if computer_wins > player_wins {
                Some(State::Loss)
            } else {
                Some(State::Tie)
            }
        } else {
            None
        }
    }
    pub fn play_rounds(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut player_wins: u8 = 0;
        let mut computer_wins: u8 = 0;
        let mut rounds_played: u8 = 0;
        let mut ties: u8 = 0;

        loop {
            print!("Rock, Paper, Scissors...GO! ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let player = match Hand::from_input(&input) {
                Some(hand) => hand,
                None => {
                    println!();
                    println!(
                        "{}",
                        "Invalid hand. Try rock, paper, or scissors.".red().bold()
                    );
                    println!();
                    continue;
                }
            };

            let computer = Hand::random();

            println!();
            println!("Your choice: {}", player.to_string().blue().bold());
            println!(
                "Computer's choice: {}",
                computer.to_string().magenta().bold()
            );
            println!();

            rounds_played += 1;

            if player == computer {
                ties += 1;
            } else if player.beats(&computer) {
                player_wins += 1;
            } else {
                computer_wins += 1;
            }
            println!(
                "Round {}:\n| Player: {} | Computer: {} | Ties: {} |",
                rounds_played.to_string().cyan(),
                player_wins.to_string().green(),
                computer_wins.to_string().red(),
                ties.to_string().yellow()
            );
            println!();
            println!("---------------------");
            println!();

            if let Some(state) = self.final_state(player_wins, computer_wins, rounds_played) {
                match state {
                    State::Win => {
                        println!(
                            "{} {}",
                            "GAME OVER".bold().black().on_green(),
                            state.to_string().green().bold()
                        )
                    }
                    State::Loss => println!(
                        "{} {}",
                        "GAME OVER".bold().black().on_red(),
                        state.to_string().red().bold()
                    ),
                    State::Tie => {
                        println!(
                            "{} {}",
                            "GAME OVER".bold().black().on_yellow(),
                            state.to_string().yellow().bold()
                        )
                    }
                }
                break;
            }
        }
        Ok(())
    }
    pub fn prompt_mode() -> Result<Self, Box<dyn std::error::Error>> {
        println!(
            "{}",
            r#"Select Game Mode:
                1) Single
                2) Best of 3
                3) Best of 5
                4) Best of 7"#
        );

        print!("{}", "Mode: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let mode: Self = match input.trim() {
            "1" => Self::Single,
            "2" => Self::BestOf3,
            "3" => Self::BestOf5,
            "4" => Self::BestOf7,
            _ => return Err("Invalid input".into()),
        };

        println!();
        println!("---------------------");
        println!();

        Ok(mode)
    }
}
