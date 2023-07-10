use super::mancala;

pub fn minimax(node: &mancala::Node, depth: usize) -> (Option<mancala::Move>, mancala::Score) {
    if depth == 0 {
        return (None, node.eval());
    }
    let children = node.children();
    let wrapped_extremum_tuple =
    match node.get_turn() {
        mancala::Player::White => children.iter().max_by_key(|(_, child)| minimax(child, depth-1).1),
        mancala::Player::Black => children.iter().min_by_key(|(_, child)| minimax(child, depth-1).1)
    };
    // if wrapped_extremum_tuple is None then that means there are no children (i.e. the game is over)
    match wrapped_extremum_tuple {
        Some(extremum_tuple) => (Some(extremum_tuple.0.clone()), extremum_tuple.1.eval()),
        None => (None, node.eval())
    }
}