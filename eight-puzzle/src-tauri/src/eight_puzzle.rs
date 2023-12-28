use std::collections::{HashSet, VecDeque};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Move {
    Left,
    Up,
    Down,
    Right,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EightPuzzleState {
    board: [u8; 9],

    target: [u8; 9],
    move_history: Vec<Move>,
    children: Vec<EightPuzzleState>,

    graphviz: bool,
}

impl ToString for EightPuzzleState {
    fn to_string(&self) -> String {
        format!(
            "     {}  {}  {}     \\n     {}  {}  {}     \\n     {}  {}  {}     ",
            self.board[0],
            self.board[1],
            self.board[2],
            self.board[3],
            self.board[4],
            self.board[5],
            self.board[6],
            self.board[7],
            self.board[8],
        )
    }
}

impl PartialEq for EightPuzzleState {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board
    }
}

impl EightPuzzleState {
    fn new(graphviz: bool, state: [u8; 9], target: [u8; 9]) -> EightPuzzleState {
        EightPuzzleState {
            board: state,
            target,
            move_history: Vec::new(),
            children: Vec::new(),
            graphviz,
        }
    }

    fn int_repr(&self) -> u64 {
        let mut repr = 0;
        for i in 0..9 {
            repr += (self.board[i] as u64) << (4 * i);
        }
        repr
    }

    fn repr(&self) -> String {
        format!(
            "{}{}{}{}{}{}{}{}{}",
            self.board[0],
            self.board[1],
            self.board[2],
            self.board[3],
            self.board[4],
            self.board[5],
            self.board[6],
            self.board[7],
            self.board[8],
        )
    }

    pub fn last_move(&self) -> Option<Move> {
        self.move_history.last().cloned()
    }

    pub fn blank_position(&self) -> usize {
        self.board.iter().position(|&x| x == 0).unwrap()
    }

    pub fn misplaced_tiles(&self) -> usize {
        let mut count = 0;
        for i in 1..9 {
            if self.board[i] != self.target[i] {
                count += 1;
            }
        }
        count
    }

    pub fn manhattan_distance(&self) -> usize {
        let source = self.board;
        let target = self.target;

        let mut distance = 0;

        for i in 1..9 {
            let source_index = source.iter().position(|&x| x == i as u8).unwrap();
            let target_index = target.iter().position(|&x| x == i as u8).unwrap();

            let source_x = source_index % 3;
            let source_y = source_index / 3;

            let target_x = target_index % 3;
            let target_y = target_index / 3;

            distance += (source_x as i8 - target_x as i8).abs() as usize;
            distance += (source_y as i8 - target_y as i8).abs() as usize;
        }

        distance
    }

    pub fn is_terminal(&self) -> bool {
        self.children == vec![]
    }

    pub fn discover_neighbours(&mut self, reverse: bool) -> Vec<&mut EightPuzzleState> {
        let mut next_states: Vec<EightPuzzleState> = Vec::new();

        let mut next_state = self.clone();
        if next_state.make_move(Move::Left) == true {
            next_states.push(next_state);
        }

        let mut next_state = self.clone();
        if next_state.make_move(Move::Up) == true {
            next_states.push(next_state)
        }

        let mut next_state = self.clone();
        if next_state.make_move(Move::Down) == true {
            next_states.push(next_state);
        }

        let mut next_state = self.clone();
        if next_state.make_move(Move::Right) == true {
            next_states.push(next_state);
        }

        self.children = next_states.clone();
        if reverse {
            return self.children.iter_mut().rev().collect();
        } else {
            return self.children.iter_mut().collect();
        }
    }

    fn is_game_complete(&self) -> bool {
        self.board == self.target
    }

    fn make_move(&mut self, kind: Move) -> bool {
        let blank_position = self.blank_position();
        let swap_position: i8 = match kind {
            Move::Up => blank_position as i8 - 3,
            Move::Down => blank_position as i8 + 3,
            Move::Left => {
                if blank_position == 0 || blank_position == 3 || blank_position == 6 {
                    -1
                } else {
                    blank_position as i8 - 1
                }
            }
            Move::Right => {
                if blank_position == 2 || blank_position == 5 || blank_position == 8 {
                    -1
                } else {
                    blank_position as i8 + 1
                }
            }
        };

        if swap_position >= 0 && swap_position < 9 {
            self.board[blank_position] = self.board[swap_position as usize];
            self.board[swap_position as usize] = 0;
            self.move_history.push(kind);
            return true;
        }

        return false;
    }

    pub fn build_tree_depth_first(
        visualize: bool,
        limit: usize,
        state: [u8; 9],
        target: [u8; 9],
    ) -> (Vec<String>, EightPuzzleState) {
        let mut game = EightPuzzleState::new(visualize, state, target);
        let mut graphviz = vec![String::new()];

        let mut queue: VecDeque<(&mut EightPuzzleState, usize)> = VecDeque::new();
        let mut visited: HashSet<u64> = HashSet::new();

        if visualize {
            graphviz.last_mut().unwrap().push_str(&format!(
                "    {} [label=\"{}\", fillcolor=blue];\n",
                game.repr(),
                game.to_string(),
            ));
        }

        queue.push_back((&mut game, 0));

        if visualize {
            graphviz.push(graphviz.last().unwrap().clone());
        }

        let mut current_state;
        let mut depth;
        let mut neighbours;

        while let Some((state, d)) = queue.pop_front() {
            current_state = state;
            depth = d;

            if depth > limit {
                continue;
            }

            let current_state_clone = current_state.clone();

            visited.insert(current_state.int_repr());

            if current_state.is_game_complete() {
                println!("{:?}", current_state);
                break;
            }

            neighbours = current_state.discover_neighbours(false);

            for neighbour in neighbours {
                // println!("{:?}", visited.len());

                let neighbour_clone = neighbour.clone();

                if visited.contains(&neighbour.int_repr()) {
                    graphviz.push(graphviz.last().unwrap().clone());
                    continue;
                } else {
                    queue.push_back((neighbour, depth + 1));
                }

                if visualize {
                    graphviz.last_mut().unwrap().push_str(&format!(
                        "    {} [label=\"{}\", fillcolor=blue];\n",
                        neighbour_clone.repr(),
                        neighbour_clone.to_string(),
                    ));

                    graphviz.last_mut().unwrap().push_str(&format!(
                        "    {} -> {} [label=\"{:?}\", color=red];\n",
                        current_state_clone.repr(),
                        neighbour_clone.repr(),
                        neighbour_clone.move_history.last().unwrap()
                    ));

                    graphviz.push(graphviz.last().unwrap().clone());
                }
            }
        }

        // println!("{:?}", graphviz);

        return (
            graphviz
                .iter()
                .map(|x| format!("digraph {{\n{}}}", x))
                .collect(),
            game,
        );
    }

    pub fn astar_evaluation(&self, heuristic_fn: fn(&EightPuzzleState) -> usize) -> usize {
        println!("{:?}", self.to_string());
        println!(
            "EVALS: {:?}, {:?}, {:?}",
            self.move_history.len(),
            heuristic_fn(&self),
            self.move_history.len() + heuristic_fn(&self)
        );
        self.move_history.len() + heuristic_fn(&self)
    }

    pub fn astar_search(
        visualize: bool,
        heuristic_fn: fn(&EightPuzzleState) -> usize,
        state: [u8; 9],
        target: [u8; 9],
    ) -> (Vec<String>, EightPuzzleState) {
        let mut game = EightPuzzleState::new(visualize, state, target);
        let mut graphviz = vec![String::new()];

        let mut queue: Vec<&mut EightPuzzleState> = Vec::new();
        let mut visited: HashSet<u64> = HashSet::new();

        if visualize {
            graphviz.last_mut().unwrap().push_str(&format!(
                "    {} [label=\"{}\", fillcolor=blue];\n",
                game.repr(),
                format!(
                    "{}\\n\\nh={}",
                    game.to_string(),
                    game.astar_evaluation(heuristic_fn)
                ),
            ));

            graphviz.push(graphviz.last().unwrap().clone());
        }

        queue.push(&mut game);

        let mut current_state;
        let mut neighbours;
        let mut found = false;

        while queue.len() > 0 {
            current_state = queue.remove(0);

            let current_state_clone = current_state.clone();

            visited.insert(current_state.int_repr());

            neighbours = current_state.discover_neighbours(false);

            for neighbour in neighbours {
                println!("{:?}", visited.len());

                let neighbour_clone = neighbour.clone();

                if visited.contains(&neighbour.int_repr()) {
                    if visualize {
                        graphviz.push(graphviz.last().unwrap().clone());
                    }
                    continue;
                } else if queue.contains(&neighbour) {
                    if visualize {
                        graphviz.push(graphviz.last().unwrap().clone());
                    }
                    continue;
                } else {
                    queue.push(neighbour);
                }

                if visualize {
                    graphviz.last_mut().unwrap().push_str(&format!(
                        "    {} [label=\"{}\", fillcolor=blue];\n",
                        neighbour_clone.repr(),
                        format!(
                            "{}\\n\\nh={}",
                            neighbour_clone.to_string(),
                            neighbour_clone.astar_evaluation(heuristic_fn)
                        ),
                    ));

                    graphviz.last_mut().unwrap().push_str(&format!(
                        "    {} -> {} [label=\"{:?}\", color=red];\n",
                        current_state_clone.repr(),
                        neighbour_clone.repr(),
                        neighbour_clone.move_history.last().unwrap()
                    ));

                    graphviz.push(graphviz.last().unwrap().clone());
                }

                if neighbour_clone.is_game_complete() {
                    println!("{:?}", neighbour_clone);
                    found = true;
                    break;
                }
            }

            if found {
                break;
            }

            // Reorder queue based on heuristic_fn
            queue.sort_by(|a, b| {
                let a_heuristic = a.astar_evaluation(heuristic_fn);
                let b_heuristic = b.astar_evaluation(heuristic_fn);

                a_heuristic.cmp(&b_heuristic)
            });
        }

        // println!("{:?}", graphviz);

        return (
            graphviz
                .iter()
                .map(|x| format!("digraph {{\n{}}}", x))
                .collect(),
            game,
        );
    }

    pub fn build_tree_breadth_first(
        visualize: bool,
        state: [u8; 9],
        target: [u8; 9],
    ) -> (Vec<String>, EightPuzzleState) {
        let mut game = EightPuzzleState::new(visualize, state, target);
        let mut graphviz = vec![String::new()];

        let mut queue: Vec<&mut EightPuzzleState> = Vec::new();
        let mut visited: HashSet<u64> = HashSet::new();

        if visualize {
            graphviz.last_mut().unwrap().push_str(&format!(
                "    {} [label=\"{}\", fillcolor=blue];\n",
                game.repr(),
                game.to_string(),
            ));

            graphviz.push(graphviz.last().unwrap().clone());
        }

        queue.push(&mut game);

        let mut current_state;
        let mut neighbours;
        let mut found = false;

        while queue.len() > 0 {
            current_state = queue.remove(0);

            let current_state_clone = current_state.clone();

            visited.insert(current_state.int_repr());

            neighbours = current_state.discover_neighbours(false);

            for neighbour in neighbours {
                // println!("{:?}", visited.len());

                let neighbour_clone = neighbour.clone();

                if visited.contains(&neighbour.int_repr()) {
                    if visualize {
                        graphviz.push(graphviz.last().unwrap().clone());
                    }
                    continue;
                } else if queue.contains(&neighbour) {
                    if visualize {
                        graphviz.push(graphviz.last().unwrap().clone());
                    }
                    continue;
                } else {
                    queue.push(neighbour);
                }

                if visualize {
                    graphviz.last_mut().unwrap().push_str(&format!(
                        "    {} [label=\"{}\", fillcolor=blue];\n",
                        neighbour_clone.repr(),
                        neighbour_clone.to_string(),
                    ));

                    graphviz.last_mut().unwrap().push_str(&format!(
                        "    {} -> {} [label=\"{:?}\", color=red];\n",
                        current_state_clone.repr(),
                        neighbour_clone.repr(),
                        neighbour_clone.move_history.last().unwrap()
                    ));

                    graphviz.push(graphviz.last().unwrap().clone());
                }

                if neighbour_clone.is_game_complete() {
                    println!("{:?}", neighbour_clone);
                    found = true;
                    break;
                }
            }

            if found {
                break;
            }
        }

        // println!("{:?}", graphviz);

        return (
            graphviz
                .iter()
                .map(|x| format!("digraph {{\n{}}}", x))
                .collect(),
            game,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_tree() {
        // let test = EightPuzzleState {
        //     board: [0, 2, 3, 1, 4, 6, 7, 5, 8],
        //     target: [1, 2, 3, 4, 5, 6, 7, 8, 0],
        //     move_history: Vec::new(),
        //     children: Vec::new(),
        //     graphviz: false,
        // };
        //
        // println!("Manhattan: {:?}", test.manhattan_distance());
        // return;

        let (ret, _) = EightPuzzleState::astar_search(
            true,
            EightPuzzleState::manhattan_distance,
            [1, 2, 3, 0, 4, 6, 7, 5, 8],
            [1, 2, 3, 4, 5, 6, 7, 8, 0],
        );

        println!("{:?}", ret);

        // let (ret, _) = EightPuzzleState::build_tree_breadth_first(
        //
        //     false,
        //     [6, 0, 8, 3, 1, 5, 2, 7, 4],
        //     [1, 2, 3, 4, 5, 6, 7, 8, 0],
        // );
        //
        // println!("{:?}", ret);
    }
}
