use crate::trick::Trick;
use crate::utils::get_trick_input;
use std::collections::HashSet;

pub struct Game {
    player1_score: String,
    player2_score: String,
    landed_tricks: HashSet<String>,
    turn: u8,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player1_score: String::new(),
            player2_score: String::new(),
            landed_tricks: HashSet::new(),
            turn: 1,
        }
    }

    pub fn run(&mut self) {
        loop {
            let current_player = if self.turn % 2 == 1 {
                "Player 1"
            } else {
                "Player 2"
            };
            let other_player = if self.turn % 2 == 1 {
                "Player 2"
            } else {
                "Player 1"
            };

            println!(
                "Current score - Player 1: {}, Player 2: {}",
                self.player1_score, self.player2_score
            );
            println!(
                "{}'s turn to set a trick. Enter your trick:",
                current_player
            );

            let trick_input = get_trick_input();

            match Trick::parse(&trick_input) {
                Ok(trick) => {
                    if self.landed_tricks.contains(&trick_input) {
                        println!("This trick has already been landed. Try a different one.");
                        continue;
                    }

                    let chance = trick.calculate_chance();
                    let current_player_landed = trick.land_trick(chance);

                    println!(
                        "{} attempted {}. Chance to land: {:.2}%. {}",
                        current_player,
                        trick_input,
                        chance * 100.0,
                        if current_player_landed {
                            "Success!"
                        } else {
                            "Failed!"
                        }
                    );

                    if current_player_landed {
                        self.landed_tricks.insert(trick_input.clone());
                        println!(
                            "{} sets the trick. {}'s turn to match.",
                            current_player, other_player
                        );

                        println!("{} attempts {}.", other_player, trick_input);
                        let other_player_landed = trick.land_trick(chance);

                        println!(
                            "{}",
                            if other_player_landed {
                                "Success! No letter added."
                            } else {
                                "Failed! Adding a letter."
                            }
                        );

                        if !other_player_landed {
                            self.add_letter(other_player);
                        }
                    } else {
                        println!(
                            "{} failed to set the trick. No letter added.",
                            current_player
                        );
                    }
                }
                Err(e) => {
                    println!("Invalid trick: {}. Please try again.", e);
                    continue;
                }
            }

            if self.player1_score == "SKATE" || self.player2_score == "SKATE" {
                break;
            }

            self.turn += 1;
        }

        if self.player1_score == "SKATE" {
            println!("Player 2 wins!");
        } else {
            println!("Player 1 wins!");
        }
    }

    fn add_letter(&mut self, player: &str) {
        let score = if player == "Player 1" {
            &mut self.player1_score
        } else {
            &mut self.player2_score
        };
        let next_letter = match score.len() {
            0 => 'S',
            1 => 'K',
            2 => 'A',
            3 => 'T',
            4 => 'E',
            _ => return,
        };
        score.push(next_letter);
    }
}
