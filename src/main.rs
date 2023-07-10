mod mancala;
mod minimax;

fn main() {
    let node = mancala::Node::default();
    let (wrapped_best_move, eval) = minimax::minimax(&node, 6);
    println!("Best move: {:?}", wrapped_best_move.unwrap());
    println!("Evaluation: {}", eval);
}
