use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Error, Write};
use std::{fmt::Display, fs::File};

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
pub struct State {
    missionaries: u8,
    cannibals: u8,
    boat: bool,

    sail_history: Vec<Move>,
    children: Vec<State>,
}

impl ToString for State {
    fn to_string(&self) -> String {
        format!(
            "{}{}{}",
            self.missionaries,
            self.cannibals,
            if self.boat { 1 } else { 0 }
        )
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.missionaries == other.missionaries
            && self.cannibals == other.cannibals
            && self.boat == other.boat
    }
}

impl State {
    fn new() -> State {
        State {
            missionaries: N,
            cannibals: N,
            boat: true,
            sail_history: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn last_move(&self) -> Option<Move> {
        self.sail_history.last().cloned()
    }

    pub fn is_terminal(&self) -> bool {
        self.children == vec![]
    }

    pub fn discover_neighbours(&mut self) -> Vec<&mut State> {
        let mut next_states: Vec<State> = Vec::new();

        let mut next_state = self.clone();
        if next_state.sail(Move::Missionary) == true {
            next_states.push(next_state)
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

        self.children = next_states.clone();

        return self.children.iter_mut().collect();
    }

    fn is_game_complete(&self) -> bool {
        self.missionaries == 0 && self.cannibals == 0
    }

    fn repr(&self) -> u64 {
        return self.missionaries as u64
            + (self.cannibals as u64 * 10)
            + ((self.boat as u64) * 100);
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

    pub fn build_breadth_first_tree() -> Vec<String> {
        let mut game = State::new();
        // let mut graphviz_nodes = String::new();
        // let mut graphviz_edges = String::new();
        let mut graphviz = vec![String::new()];

        let mut queue: VecDeque<&mut State> = VecDeque::new();
        let mut visited: Vec<u64> = Vec::new();

        graphviz.last_mut().unwrap().push_str(&format!(
            "    {} [label=\"<< {}, {}, {}>>\", fillcolor=blue, style=filled];\n",
            game.to_string(),
            game.missionaries,
            game.cannibals,
            game.boat
        ));

        graphviz.push(graphviz.last().unwrap().clone());

        queue.push_back(&mut game);

        let mut current_state;
        let mut neighbours;

        let mut found = false;

        while queue.len() > 0 {
            current_state = queue.pop_front().unwrap();

            let current_state_clone = current_state.clone();

            visited.push(current_state.repr());

            neighbours = current_state.discover_neighbours();

            for neighbour in neighbours {
                let neighbour_clone = neighbour.clone();

                if visited.iter().find(|x| **x == neighbour.repr()).is_some() {
                    graphviz.push(graphviz.last().unwrap().clone());
                    continue;
                } else if queue.contains(&neighbour) {
                    graphviz.push(graphviz.last().unwrap().clone());
                    continue;
                } else {
                    queue.push_back(neighbour);
                }

                let color = if neighbour_clone.is_game_complete() {
                    "green"
                } else {
                    "white"
                };

                graphviz.last_mut().unwrap().push_str(&format!(
                    "    {} [label=\"<< {}, {}, {}>>\", fillcolor={}, style=filled];\n",
                    neighbour_clone.to_string(),
                    neighbour_clone.missionaries,
                    neighbour_clone.cannibals,
                    neighbour_clone.boat,
                    color,
                ));

                graphviz.last_mut().unwrap().push_str(&format!(
                    "    {} -> {} [label=\"{:?}\", color=red];\n",
                    current_state_clone.to_string(),
                    neighbour_clone.to_string(),
                    neighbour_clone.sail_history.last().unwrap()
                ));

                graphviz.push(graphviz.last().unwrap().clone());

                if neighbour_clone.is_game_complete() {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }

        return graphviz
            .iter()
            .map(|x| format!("digraph {{\n{}}}", x))
            .collect();
    }

    pub fn build_depth_first_tree() -> Vec<String> {
        let mut game = State::new();
        // let mut graphviz_nodes = String::new();
        // let mut graphviz_edges = String::new();
        let mut graphviz = vec![String::new()];

        let mut stack: Vec<&mut State> = Vec::new();
        let mut visited: Vec<State> = Vec::new();

        graphviz.last_mut().unwrap().push_str(&format!(
            "    {} [label=\"<< {}, {}, {}>>\", fillcolor=blue];\n",
            game.to_string(),
            game.missionaries,
            game.cannibals,
            game.boat
        ));

        graphviz.push(graphviz.last().unwrap().clone());

        stack.push(&mut game);

        let mut current_state;
        let mut neighbours;
        while stack.len() > 0 {
            current_state = stack.pop().unwrap();

            let current_state_clone = current_state.clone();

            visited.push(current_state.clone());

            neighbours = current_state.discover_neighbours();

            for neighbour in neighbours {
                let neighbour_clone = neighbour.clone();

                if visited.iter().find(|x| *x == neighbour).is_some() {
                    graphviz.push(graphviz.last().unwrap().clone());
                    continue;
                } else if stack.contains(&neighbour) {
                    // graphviz.push(graphviz.last().unwrap().clone());
                    // continue;
                } else {
                    stack.push(neighbour);
                }

                graphviz.last_mut().unwrap().push_str(&format!(
                    "    {} [label=\"<< {}, {}, {}>>\", fillcolor=blue];\n",
                    neighbour_clone.to_string(),
                    neighbour_clone.missionaries,
                    neighbour_clone.cannibals,
                    neighbour_clone.boat
                ));

                graphviz.last_mut().unwrap().push_str(&format!(
                    "    {} -> {} [label=\"{:?}\", color=red];\n",
                    current_state_clone.to_string(),
                    neighbour_clone.to_string(),
                    neighbour_clone.sail_history.last().unwrap()
                ));

                graphviz.push(graphviz.last().unwrap().clone());
            }
        }

        return graphviz
            .iter()
            .map(|x| format!("digraph {{\n{}}}", x))
            .collect();
    }
}
