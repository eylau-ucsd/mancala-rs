use std::fmt;

// a pocket on the board (aliased as a u8)
pub type Pocket = usize;
pub type Move = Vec<Pocket>;
pub type Score = i32;

pub const BOARD_SIZE: Pocket = 14;
pub const STONES: Score = 4;
pub const WHITE_POCKET: Pocket = 6;
pub const BLACK_POCKET: Pocket = 13;

// either White's or Black's turn
#[derive(Debug, Clone, PartialEq)]
pub enum Player {
    White,
    Black,
}

impl Player {
    fn toggled(&self) -> Self {
        match *self {
            Player::White => Player::Black,
            Player::Black => Player::White
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Player::White => "White",
            Player::Black => "Black"
        })
    }
}

pub enum Error {
    IndexError, // pocket number not within valid range
    EmptyError // pocket chosen is empty
}

// used to represent board positions, including ones in the "middle" of a move
// we may get multiple "sub-moves" if we "land" on our own pocket
#[derive(Debug, Clone)]
pub struct Node {
    board: Vec<Score>,
    turn: Player,
}

impl Node {
    // note: no error checking since this is an internal helper method.
    fn opposite(&self, pocket: Pocket) -> Pocket {
        (BOARD_SIZE - 2) - pocket
    }

    pub fn sub_move(&mut self, pocket: Pocket) -> Result<(), Error> {
        let (own_pocket, enemy_pocket, start_index, end_index) =
        match self.turn {
            Player::White => (WHITE_POCKET, BLACK_POCKET, (BLACK_POCKET + 1) % BOARD_SIZE, WHITE_POCKET),
            Player::Black => (BLACK_POCKET, WHITE_POCKET, (WHITE_POCKET + 1) % BOARD_SIZE, BLACK_POCKET)
        };
        if (pocket < start_index) || (pocket >= end_index) {
            return Err(Error::IndexError);
        }
        if self.board[pocket] == 0 {
            return Err(Error::EmptyError);
        }
        let mut cursor = pocket;
        let mut count = self.board[pocket];
        self.board[pocket] = 0;
        while count > 0 {
            cursor = (cursor + 1) % BOARD_SIZE;
            if cursor != enemy_pocket {
                self.board[cursor] += 1;
                count -= 1;
            }
        }
        // if we land in our own pocket, we get another turn, so "next-to-move" doesn't change
        // otherwise it does change
        if cursor == own_pocket { return Ok(()); }

        // capture rule - if we "land" on an empty zone that belongs to us,
        // and it isn't our scoring pocket, then we capture everything on the opposite pocket
        // (and also the stone we captured with)
        if self.board[cursor] == 1 && start_index <= cursor && cursor < end_index {
            let opp = self.opposite(cursor);
            self.board[own_pocket] += self.board[opp] + 1;
            self.board[opp] = 0;
            self.board[cursor] = 0;
        }

        self.turn = self.turn.toggled();
        Ok(())
    }

    fn sub_children(&self) -> Vec<(Pocket, Node)> {
        let mut result = Vec::new();
        for pocket in 0..BOARD_SIZE {
            let mut new_sub_node = self.clone();
            match new_sub_node.sub_move(pocket) {
                Ok(_) => {
                    result.push((pocket, new_sub_node));
                },
                _ => {}
            };
        }
        result
    }

    fn children_from_sub_node(sub_node: &Node) -> Vec<(Move, Node)> {
        let mut result = Vec::new();
        for (pocket, sub_child) in sub_node.sub_children() {
            // if the sub node toggled the turn, that means the turn ended with that sub-node
            if sub_child.turn != sub_node.turn {
                let full_move = vec![pocket];
                result.push((full_move, sub_child));
            }
            // if turn is not ended sub-node yet, then keep on going via recursion 
            else {
                for (mut move_fragment, node) in Self::children_from_sub_node(&sub_child) {
                    // note: this makes it so that the move is in reverse-order
                    move_fragment.push(pocket);
                    result.push((move_fragment, node))
                }
            }
        }
        result
    }

    pub fn children(&self) -> Vec<(Move, Node)> {
        Self::children_from_sub_node(&self).into_iter().map(
            |(full_move, node)| {
                (full_move.into_iter().rev().collect(), node)
            }).collect()
    }

    pub fn full_move(&mut self, mv: &Move) -> Result<(), Error> {
        for sub_move in mv {
            self.sub_move(*sub_move)?;
        }
        Ok(())
    }

    pub fn get_turn(&self) -> &Player {
        &self.turn
    }

    pub fn eval(&self) -> Score {
        self.board[WHITE_POCKET] - self.board[BLACK_POCKET]
    }

    pub fn final_score(&self) -> Score {
        let white_score: Score = self.board[(BLACK_POCKET + 1) % BOARD_SIZE..WHITE_POCKET + 1].iter().sum();
        let black_score: Score = self.board[(WHITE_POCKET + 1) % BOARD_SIZE..BLACK_POCKET + 1].iter().sum();
        white_score - black_score
    }
}

impl Default for Node {
    fn default() -> Self {
        let mut new_board = vec![0; BOARD_SIZE.into()];
        for i in 0..BOARD_SIZE {
            match i {
                WHITE_POCKET | BLACK_POCKET => {}
                _ => { new_board[i] = STONES; }
            };
        }
        Node {
            board: new_board,
            turn: Player::White
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // the white/black sides of the board respectively, not including the scoring pockets
        let board_white = &(self.board)[0..(WHITE_POCKET)];
        let board_black = &(self.board)[WHITE_POCKET+1..BLACK_POCKET];
        // we display White side on bottom, Black side on top
        let board_top = board_black.iter().rev().map(
            |pocket| {
                format!("( {} )", pocket.to_string())
            }
        ).collect::<Vec<String>>().join("  ");
        let board_bottom = board_white.iter().map(
            |pocket| {
                format!("( {} )", pocket.to_string())
            }
        ).collect::<Vec<String>>().join("  ");
        write!(f, "[ {} ]  {}\n\n       {}  [ {} ]\n{} to move", self.board[BLACK_POCKET], board_top, board_bottom, self.board[WHITE_POCKET], self.turn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_children() {
        let node = Node::default();
        let stones: Score = node.board.iter().sum();
        let children = node.children();
        assert_eq!(children.len(), 10);
        for (_, child) in children {
            let child_stones: Score = child.board.iter().sum();
            assert_eq!(child_stones, stones);
        }
    }

    #[test]
    fn test_display() {
        let node = Node::default();
        let default_string =
"[ 0 ]  ( 4 )  ( 4 )  ( 4 )  ( 4 )  ( 4 )  ( 4 )

       ( 4 )  ( 4 )  ( 4 )  ( 4 )  ( 4 )  ( 4 )  [ 0 ]\nWhite to move";
        assert_eq!(node.to_string(), default_string);
    }
}