use super::mancala;
use std::cmp;

// taken from pseudocode found on Wikipedia
pub fn minimax(node: &mancala::Node, depth: usize, alpha: &mut mancala::Score, beta: &mut mancala::Score) -> (Option<mancala::Move>, mancala::Score) {
    let children = node.children();
    if children.is_empty() {
        return (None, node.final_score());
    }
    if depth == 0 {
        return (None, node.eval());
    }
    match node.get_turn() {
        mancala::Player::White => {
            let mut max_score = mancala::Score::MIN;
            let mut max_move = vec![];
            for (mv, child) in children {
                let score = minimax(&child, depth - 1, alpha, beta).1;
                if score > max_score {
                    max_score = score;
                    max_move = mv;
                }
                if max_score > *beta { break; }
                *alpha = cmp::max(*alpha, max_score);
            }
            (Some(max_move), max_score)
        }
        mancala::Player::Black => {
            let mut min_score = mancala::Score::MAX;
            let mut min_move = vec![];
            for (mv, child) in children {
                let score = minimax(&child, depth - 1, alpha, beta).1;
                if score < min_score {
                    min_score = score;
                    min_move = mv;
                }
                if min_score < *alpha { break; }
                *beta = cmp::min(*beta, min_score);
            }
            (Some(min_move), min_score)
        }
    }
}