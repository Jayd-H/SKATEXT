use rand::Rng;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    Fs,
    Bs,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Angle {
    A180,
    A360,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Stance {
    Ollie,
    Nollie,
    Fakie,
    Switch,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum FlipType {
    Heelflip,
    Kickflip,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CatchType {
    North,
    South,
}

#[derive(Debug, Clone)]
pub struct Turn {
    direction: Direction,
    angle: Angle,
}

#[derive(Debug, Clone)]
pub struct Shuv {
    direction: Direction,
    angle: Angle,
}

#[derive(Debug)]
pub struct Trick {
    turn: Option<Turn>,
    stance: Stance,
    shuv: Option<Shuv>,
    flip: Option<FlipType>,
    catch: Option<CatchType>,
    revert: bool,
}

impl Trick {
    pub fn parse(input: &str) -> Result<Self, String> {
        let lowercase_input = input.to_lowercase();
        let components: Vec<&str> = lowercase_input.split_whitespace().collect();
        let mut turn: Option<Turn> = None;
        let mut stance: Option<Stance> = None;
        let mut shuv: Option<Shuv> = None;
        let mut flip: Option<FlipType> = None;
        let mut catch: Option<CatchType> = None;
        let mut revert = false;

        let mut used_modifiers: HashSet<&str> = HashSet::new();

        let mut i = 0;
        while i < components.len() {
            match components[i] {
                "fs" | "bs" => {
                    if i + 1 < components.len() {
                        match components[i + 1] {
                            "180" | "360" => {
                                let direction = if components[i] == "fs" {
                                    Direction::Fs
                                } else {
                                    Direction::Bs
                                };
                                let angle = if components[i + 1] == "180" {
                                    Angle::A180
                                } else {
                                    Angle::A360
                                };

                                if i + 2 < components.len() && components[i + 2] == "shuv" {
                                    if used_modifiers.contains("shuv") {
                                        return Err("Only one shuv modifier allowed".to_string());
                                    }
                                    shuv = Some(Shuv { direction, angle });
                                    used_modifiers.insert("shuv");
                                    i += 2;
                                } else {
                                    if used_modifiers.contains("turn") {
                                        return Err("Only one turn modifier allowed".to_string());
                                    }
                                    turn = Some(Turn { direction, angle });
                                    used_modifiers.insert("turn");
                                    i += 1;
                                }
                            }
                            _ => {
                                return Err(
                                    "Turn or shuv must have both direction and angle".to_string()
                                )
                            }
                        }
                    } else {
                        return Err("Turn or shuv must have both direction and angle".to_string());
                    }
                }
                "ollie" | "nollie" | "fakie" | "switch" => {
                    if used_modifiers.contains("stance") {
                        return Err("Only one stance allowed".to_string());
                    }
                    stance = Some(match components[i] {
                        "ollie" => Stance::Ollie,
                        "nollie" => Stance::Nollie,
                        "fakie" => Stance::Fakie,
                        "switch" => Stance::Switch,
                        _ => unreachable!(),
                    });
                    used_modifiers.insert("stance");
                }
                "heelflip" | "kickflip" => {
                    if used_modifiers.contains("flip") {
                        return Err("Only one flip type allowed".to_string());
                    }
                    flip = Some(if components[i] == "heelflip" {
                        FlipType::Heelflip
                    } else {
                        FlipType::Kickflip
                    });
                    used_modifiers.insert("flip");
                }
                "north" | "south" => {
                    if used_modifiers.contains("catch") {
                        return Err("Only one catch type allowed".to_string());
                    }
                    catch = Some(if components[i] == "north" {
                        CatchType::North
                    } else {
                        CatchType::South
                    });
                    used_modifiers.insert("catch");
                }
                "revert" => {
                    if revert {
                        return Err("Only one revert allowed".to_string());
                    }
                    revert = true;
                }
                _ => return Err(format!("Unknown component: {}", components[i])),
            }
            i += 1;
        }

        // ensure a stance is provided
        let stance = stance
            .ok_or_else(|| "A stance (ollie, nollie, fakie, or switch) is required".to_string())?;

        Ok(Trick {
            turn,
            stance,
            shuv,
            flip,
            catch,
            revert,
        })
    }

    pub fn calculate_chance(&self) -> f64 {
        let mut base_chance = 1.0;
        let modifiers = [
            (matches!(self.stance, Stance::Ollie), 0.95),
            (matches!(self.stance, Stance::Nollie), 0.9),
            (matches!(self.stance, Stance::Switch), 0.7),
            (matches!(self.stance, Stance::Fakie), 0.9),
            (self.turn.is_some(), 0.9),
            (self.shuv.is_some(), 0.9),
            (self.flip.is_some(), 0.7),
            (self.catch.is_some(), 0.8),
            (self.revert, 0.85),
        ];

        for (condition, chance) in modifiers.iter() {
            if *condition {
                base_chance *= chance;
            }
        }

        // additional discount for 360s
        if let Some(Turn {
            angle: Angle::A360, ..
        }) = self.turn
        {
            base_chance *= 0.8;
        }
        if let Some(Shuv {
            angle: Angle::A360, ..
        }) = self.shuv
        {
            base_chance *= 0.8;
        }

        base_chance
    }

    pub fn land_trick(&self, chance: f64) -> bool {
        let mut rng = rand::thread_rng();
        rng.gen::<f64>() < chance
    }
}

impl std::fmt::Display for Trick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::new();

        if let Some(Turn { direction, angle }) = &self.turn {
            parts.push(format!("{:?} {:?}", direction, angle));
        }

        parts.push(format!("{:?}", self.stance));

        if let Some(Shuv { direction, angle }) = &self.shuv {
            parts.push(format!("{:?} {:?} shuv", direction, angle));
        }

        if let Some(flip) = &self.flip {
            parts.push(format!("{:?}", flip));
        }

        if let Some(catch) = &self.catch {
            parts.push(format!("{:?}", catch));
        }

        if self.revert {
            parts.push("revert".to_string());
        }

        write!(f, "{}", parts.join(" "))
    }
}
