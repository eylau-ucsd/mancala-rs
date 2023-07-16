mod mancala;
mod minimax;

fn main() {
    let mut node = mancala::Node::default();
    let mut alpha = mancala::Score::MIN;
    let mut beta = mancala::Score::MAX;
    let (wrapped_best_move, score) = minimax::minimax(&node, 10, &mut alpha, &mut beta);
    let best_move = wrapped_best_move.unwrap();
    println!("Best move: {:?}", best_move);
    println!("Score: {}", score);
    match node.full_move(&best_move) {
        Ok(_) => {println!("Position after move:\n{}", node)},
        Err(_) => {println!("Error occurred when playing move")}
    }
}
