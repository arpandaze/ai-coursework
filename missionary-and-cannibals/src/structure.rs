use colored::Colorize;
use colored::*;
use serde::{Deserialize, Serialize};

const N: u8 = 3;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Move {
    Missionary,
    Cannibal,
    TwoMissionaries,
    TwoCannibals,
    MissionaryCannibal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MissionaryCannibals {
    missionaries: u8,
    cannibals: u8,
    boat: bool,

    sail_history: Vec<Move>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Node {
    state: (u8, u8, bool),
    terminal: bool,
    accepted: bool,
    transitions: Vec<Move>,
    children: Vec<Node>,
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

    pub fn is_terminal(&self) -> bool {
        self.discover_neighbours() == [None, None, None, None, None]
    }

    pub fn discover_neighbours(&self) -> [Option<MissionaryCannibals>; 5] {
        let mut next_states: [Option<MissionaryCannibals>; 5] = [None, None, None, None, None];

        let mut next_state = self.clone();
        if next_state.sail(Move::Missionary) == true {
            next_states[0] = Some(next_state);
        }

        let mut next_state = self.clone();
        if next_state.sail(Move::Cannibal) == true {
            next_states[1] = Some(next_state);
        }

        let mut next_state = self.clone();
        if next_state.sail(Move::TwoMissionaries) == true {
            next_states[2] = Some(next_state);
        }

        let mut next_state = self.clone();
        if next_state.sail(Move::TwoCannibals) == true {
            next_states[3] = Some(next_state);
        }

        let mut next_state = self.clone();
        if next_state.sail(Move::MissionaryCannibal) == true {
            next_states[4] = Some(next_state);
        }

        next_states
    }

    fn breadth_first_search(&mut self) -> Option<(MissionaryCannibals, Vec<Node>)> {
        let mut nodes: Vec<Node> = Vec::new();

        let mut queue: Vec<MissionaryCannibals> = Vec::new();
        let mut visited: Vec<MissionaryCannibals> = Vec::new();

        queue.push(self.clone());

        while !queue.is_empty() {
            let front = queue.remove(0);

            visited.push(front.clone());

            nodes.push(Node {
                state: (front.missionaries, front.cannibals, front.boat),
                transitions: front.sail_history.clone(),
                terminal: true,
                accepted: front.is_game_complete(),
                children: Vec::new(),
            });

            if front.is_game_complete() {
                return Some((front, nodes));
            }

            for next_state in front.discover_neighbours() {
                if next_state.is_some() {
                    if !visited.contains(&next_state.clone().unwrap()) {
                        queue.push(next_state.clone().unwrap());
                    }
                }
            }
        }

        return None;
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

        if next_state.missionaries != 0 && (next_state.missionaries < next_state.cannibals) {
            valid = false;
        }

        let missionary_next_side = N as i8 - next_state.missionaries as i8;
        let cannibal_next_side = N as i8 - next_state.cannibals as i8;

        if next_state.cannibals > N
            || next_state.missionaries > N
            || missionary_next_side < 0
            || cannibal_next_side < 0
        {
            valid = false;
        }

        if missionary_next_side != 0 && (missionary_next_side < cannibal_next_side) {
            valid = false;
        }

        if valid {
            self.boat = !self.boat;
            self.missionaries = next_state.missionaries;
            self.cannibals = next_state.cannibals;
            self.sail_history.push(kind);
            // self.neighbour = [None, None, None, None, None];
        } else {
            // println!("Invalid move");
        }
        return valid;
    }
}

#[test]
fn test_game() {
    let mut game = MissionaryCannibals::new();

    let (final_game, nodes) = game.breadth_first_search().unwrap();

    let mut prev_transition_n = 0;
    for node in nodes {
        if node.transitions.len() != prev_transition_n {
            println!("------------------");
            println!("    BACKTRACK     ");
            println!("------------------");
            println!("------------------");
            println!("      {}", "##".yellow());
            prev_transition_n = node.transitions.len();
        }

        println!("      {}", "||".yellow());
        println!("      {}", "||".yellow());
        println!("{:?}", node.transitions.last());
        println!("      {}", "||".yellow());
        println!("      {}", "||".yellow());
        println!("      {}", "\\/".yellow());

        if node.accepted {
            println!("{}", "------------------".green(),);
            println!(
                "{} {}, {}, {} {}",
                "<<".green(),
                node.state.0.to_string().green(),
                node.state.1.to_string().green(),
                node.state.2.to_string().green(),
                ">>".green(),
            );
            println!("{}", "------------------".green(),);
        } else if node.terminal {
            println!("{}", "------------------".red(),);
            println!(
                "{} {}, {}, {} {}",
                "<<".red(),
                node.state.0.to_string().red(),
                node.state.1.to_string().red(),
                node.state.2.to_string().red(),
                ">>".red(),
            );
            println!("{}", "------------------".red(),);
        } else {
            println!("{}", "------------------".blue(),);
            println!(
                "{} {}, {}, {} {}",
                "<<".blue(),
                node.state.0.to_string().blue(),
                node.state.1.to_string().blue(),
                node.state.2.to_string().blue(),
                ">>".blue(),
            );
            println!("{}", "------------------".blue(),);
        }
    }

    println!("{}", "------------------".magenta(),);
    println!("{}", "      Actions     ".magenta(),);
    println!("{}", "------------------".magenta(),);
    for action in final_game.sail_history {
        println!("{}", format!("{:?}", action).to_string().green());
    }
}
