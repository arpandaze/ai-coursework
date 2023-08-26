const N: u8 = 4;

#[derive(Debug, Clone)]
pub enum Move {
    Missionary,
    Cannibal,
    TwoMissionaries,
    TwoCannibals,
    MissionaryCannibal,
}

#[derive(Debug, Clone)]
struct MissionaryCannibals {
    missionaries: u8,
    cannibals: u8,
    boat: bool,

    sail_history: Vec<Move>,
}

impl PartialEq for MissionaryCannibals {
    fn eq(&self, other: &Self) -> bool {
        self.missionaries == other.missionaries
            && self.cannibals == other.cannibals
            && self.boat == other.boat
    }
}

impl MissionaryCannibals {
    fn new() -> MissionaryCannibals {
        MissionaryCannibals {
            missionaries: N,
            cannibals: N,
            boat: true,
            sail_history: Vec::new(),
        }
    }

    pub fn get_next_states(&self) -> Vec<MissionaryCannibals> {
        let mut next_states: Vec<MissionaryCannibals> = Vec::new();

        let mut next_state = self.clone();
        if next_state.sail(Move::Missionary) == true {
            next_states.push(next_state);
        }

        let mut next_state = self.clone();
        if next_state.sail(Move::Cannibal) == true {
            next_states.push(next_state);
        }

        let mut next_state = self.clone();
        if next_state.sail(Move::TwoMissionaries) == true {
            next_states.push(next_state);
        }

        let mut next_state = self.clone();
        if next_state.sail(Move::TwoCannibals) == true {
            next_states.push(next_state);
        }

        let mut next_state = self.clone();
        if next_state.sail(Move::MissionaryCannibal) == true {
            next_states.push(next_state);
        }

        next_states
    }

    fn breadth_first_search(&self) -> Option<MissionaryCannibals> {
        let mut states: Vec<MissionaryCannibals> = Vec::new();

        states.push(self.clone());

        loop {
            let current_state = states.remove(0);

            if current_state.is_game_complete() {
                return Some(current_state);
            }

            for next_state in current_state.get_next_states() {
                if !states.contains(&next_state) {
                    states.push(next_state);
                }
            }
        }
    }

    fn is_game_complete(&self) -> bool {
        self.missionaries == 0 && self.cannibals == 0
    }

    fn sail(&mut self, kind: Move) -> bool {
        let mut valid = true;

        let mut next_state = self.clone();

        match kind {
            Move::Missionary => {
                if next_state.missionaries > 0 && next_state.boat {
                    next_state.missionaries -= 1;
                } else if next_state.missionaries < N && !next_state.boat {
                    next_state.missionaries += 1;
                } else {
                    valid = false;
                }
            }
            Move::Cannibal => {
                if next_state.cannibals > 0 && next_state.boat {
                    next_state.cannibals -= 1;
                } else if next_state.cannibals < N && !next_state.boat {
                    next_state.cannibals += 1;
                } else {
                    valid = false;
                }
            }
            Move::TwoMissionaries => {
                if next_state.missionaries > 1 && next_state.boat {
                    next_state.missionaries -= 2;
                } else if next_state.missionaries < N && !next_state.boat {
                    next_state.missionaries += 2;
                } else {
                    valid = false;
                }
            }
            Move::TwoCannibals => {
                if next_state.cannibals > 1 && next_state.boat {
                    next_state.cannibals -= 2;
                } else if next_state.cannibals < N && !next_state.boat {
                    next_state.cannibals += 2;
                } else {
                    valid = false;
                }
            }
            Move::MissionaryCannibal => {
                if next_state.missionaries > 0 && next_state.cannibals > 0 && next_state.boat {
                    next_state.missionaries -= 1;
                    next_state.cannibals -= 1;
                } else if next_state.missionaries < N
                    && next_state.cannibals < N
                    && !next_state.boat
                {
                    next_state.missionaries += 1;
                    next_state.cannibals += 1;
                } else {
                    valid = false;
                }
            }
        }

        if next_state.missionaries > 0 && next_state.cannibals > next_state.missionaries {
            valid = false;
        }

        if next_state.missionaries < N && next_state.cannibals < next_state.missionaries {
            valid = false;
        }

        if valid {
            self.boat = !self.boat;
            self.missionaries = next_state.missionaries;
            self.cannibals = next_state.cannibals;
            self.sail_history.push(kind);
        }
        return valid;
    }
}

#[test]
fn test_game() {
    let game = MissionaryCannibals::new();
    // game.sail(Move::TwoCannibals);
    // game.sail(Move::Cannibal);
    // game.sail(Move::TwoCannibals);
    // game.sail(Move::Cannibal);
    // game.sail(Move::TwoMissionaries);
    // game.sail(Move::Missionary);

    println!("{:?}", game.breadth_first_search());
    // println!("{:?}", game.is_game_complete());
}
