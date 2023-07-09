const ENGINE_DEPTH: u8 = 10;
const BOARD_SIZE: u8 = 14;
const STONES: u8 = 4;
const WHITE_POCKET: u8 = 6;
const BLACK_POCKET: u8 = 13;

// a pocket on the board (aliased as a u8)
type Pocket = u8;

// either White's or Black's turn
#[derive(Debug, Clone, PartialEq)]
pub enum Turn {
    White,
    Black,
}

impl Turn {
    fn toggled(&self) -> Self {
        match *self {
            Turn::White => Turn::Black,
            Turn::Black => Turn::White
        }
    }
}

pub enum MancalaError {
    IndexError, // pocket number not within valid range
    EmptyError // pocket chosen is empty
}

// used to represent board positions, including ones in the "middle" of a move
// we may get multiple "sub-moves" if we "land" on our own pocket
#[derive(Debug, Clone)]
struct SubNode {
    board: Vec<Pocket>,
    turn: Turn,
}

impl SubNode {
    // note: no error checking since this is an internal helper method.
    fn opposite(&self, pocket: Pocket) -> Pocket {
        (BOARD_SIZE - 2) - pocket
    }

    fn sub_move(&mut self, pocket: Pocket) -> Result<(), MancalaError> {
        let (own_pocket, enemy_pocket, start_index, end_index) =
        match self.turn {
            Turn::White => (WHITE_POCKET, BLACK_POCKET, (BLACK_POCKET + 1) % BOARD_SIZE, WHITE_POCKET),
            Turn::Black => (BLACK_POCKET, WHITE_POCKET, (WHITE_POCKET + 1) % BOARD_SIZE, BLACK_POCKET)
        };
        if (pocket < start_index) || (pocket >= end_index) {
            return Err(MancalaError::IndexError);
        }
        if self.board[pocket as usize] == 0 {
            return Err(MancalaError::EmptyError);
        }
        let mut cursor = pocket;
        let mut count = self.board[pocket as usize];
        while count > 0 {
            cursor = (cursor + 1) % BOARD_SIZE;
            if cursor != enemy_pocket {
                self.board[cursor as usize] += 1;
                count -= 1;
            }
        }
        // if we land in our own pocket, we get another turn, so "next-to-move" doesn't change
        // otherwise it does change
        if cursor == own_pocket { return Ok(()); }

        // capture rule - if we "land" on an empty zone that belongs to us,
        // and it isn't our scoring pocket, then we capture everything on the opposite pocket
        // (and also the stone we captured with)
        if self.board[cursor as usize] == 1 && start_index <= cursor && cursor < end_index {
            let opp = self.opposite(cursor);
            self.board[own_pocket as usize] += self.board[opp as usize] + 1;
            self.board[opp as usize] = 0;
            self.board[cursor as usize] = 0;
        }

        self.turn = self.turn.toggled();
        Ok(())
    }

    fn sub_children(&self) -> Vec<(Pocket, SubNode)> {
        let mut result = Vec::new();
        for pocket in 0..BOARD_SIZE-1 {
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
}

pub struct Node(SubNode);

impl Node {
    // get all node (NOT sub-node) children, given a starting sub-node
    fn children_from_sub_node(sub_node: &SubNode) -> Vec<(Vec<Pocket>, Node)> {
        let mut result = Vec::new();
        for (pocket, sub_child) in sub_node.sub_children() {
            if sub_child.turn != sub_node.turn {
                let full_move = vec![pocket];
                result.push((full_move, Node(sub_child)));
            }
            else {
                let mut full_move = vec![pocket];
                for (move_fragment, node) in Self::children_from_sub_node(&sub_child) {
                    let mut full_move = vec![pocket];
                    full_move.extend(move_fragment); // TODO: this is slow - speed this up
                    result.push((full_move, node))
                }
            }
        }
        result
    }

    pub fn children(&self) -> Vec<(Vec<Pocket>, Node)> {
        Self::children_from_sub_node(&self.0)
    }
}