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
}

impl ToString for EightPuzzleState {
    fn to_string(&self) -> String {
        format!(
            "{} {} {}\\n{} {} {}\\n{} {} {}",
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
    fn new(state: [u8; 9], target: [u8; 9]) -> EightPuzzleState {
        EightPuzzleState {
            board: state,
            target,
            move_history: Vec::new(),
            children: Vec::new(),
        }
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

    pub fn is_terminal(&self) -> bool {
        self.children == vec![]
    }

    pub fn discover_neighbours(&mut self) -> Vec<&mut EightPuzzleState> {
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

        return self.children.iter_mut().collect();
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

    pub fn build_tree_breadth_first(state: [u8; 9], target: [u8; 9]) -> Vec<String> {
        let mut game = EightPuzzleState::new(state, target);
        // let mut graphviz = vec![String::new()];

        let mut queue: Vec<&mut EightPuzzleState> = Vec::new();
        let mut visited: Vec<EightPuzzleState> = Vec::new();

        // graphviz.last_mut().unwrap().push_str(&format!(
        //     "    {} [label=\"{}\", fillcolor=blue];\n",
        //     game.repr(),
        //     game.to_string(),
        // ));

        // graphviz.push(graphviz.last().unwrap().clone());

        queue.push(&mut game);

        let mut current_state;
        let mut neighbours;
        while queue.len() > 0 {
            current_state = queue.remove(0);

            let current_state_clone = current_state.clone();

            if current_state.is_game_complete() {
                println!("{:?}", current_state);
                break;
            }

            visited.push(current_state.clone());

            neighbours = current_state.discover_neighbours();

            for neighbour in neighbours {
                println!("{:?}", visited.len());

                let neighbour_clone = neighbour.clone();

                if visited.contains(&neighbour) {
                    // graphviz.push(graphviz.last().unwrap().clone());
                    continue;
                } else if queue.contains(&neighbour) {
                    // graphviz.push(graphviz.last().unwrap().clone());
                    // continue;
                } else {
                    queue.push(neighbour);
                }

                // graphviz.last_mut().unwrap().push_str(&format!(
                //     "    {} [label=\"{}\", fillcolor=blue];\n",
                //     neighbour_clone.repr(),
                //     neighbour_clone.to_string(),
                // ));
                //
                // graphviz.last_mut().unwrap().push_str(&format!(
                //     "    {} -> {} [label=\"{:?}\", color=red];\n",
                //     current_state_clone.repr(),
                //     neighbour_clone.repr(),
                //     neighbour_clone.move_history.last().unwrap()
                // ));
                //
                // graphviz.push(graphviz.last().unwrap().clone());
            }
        }

        // return graphviz
        //     .iter()
        //     .map(|x| format!("digraph {{\n{}}}", x))
        //     .collect();
        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_tree() {
        let mut state = EightPuzzleState::build_tree_breadth_first(
            [1, 3, 5, 8, 0, 7, 4, 6, 2],
            [1, 2, 3, 4, 5, 6, 7, 8, 0],
        );

        // println!("{:?}", state.make_move(Move::Down));
        // println!("{:?}", state.is_terminal());
        // println!("{:?}", state);
    }
}
