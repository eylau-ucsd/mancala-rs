use std::io;
use std::io::Write;

mod mancala;
mod minimax;

const DEPTH: usize = 10;

fn cls() {
    print!("{esc}c", esc = 27 as char);
}

fn main() -> io::Result<()> {
    let mut node = mancala::Node::default();
    cls();
    print!("Hello! I am the Mancala Rust AI. Would you like to play as White or Black? (w/b) ");
    io::stdout().flush()?;
    let user_player = loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        break match buffer.trim() as &str {
            "w" | "W" => mancala::Player::White,
            "b" | "Black" => mancala::Player::Black,
            _ => {
                print!("Invalid option. Enter 'w' (White) or 'b' (Black): ");
                io::stdout().flush()?;
                continue;
            }
        };
    };

    cls();
    println!("{}", node);

    loop {
        if node.children().is_empty() {
            println!("Game over!");
            let final_score = node.final_score();
            if final_score > 0 {
                println!("White wins by {}", final_score);
            }
            else if final_score < 0 {
                println!("Black wins by {}", -1 * final_score);
            }
            else {
                println!("Draw.");
            }
            break;
        }
        if *node.get_turn() == user_player {
            loop {
                print!("Enter move: ");
                io::stdout().flush()?;
                let mut buffer = String::new();
                io::stdin().read_line(&mut buffer)?;
                match buffer.trim().parse() {
                    Ok(v) => {
                        match node.sub_move(v) {
                            Ok(_) => {
                                cls();
                                println!("{}", node);
                                break;
                            }
                            Err(_) => {
                                println!("Invalid input, please try again.");
                                continue;
                            }
                        }
                    }
                    Err(_) => {
                        println!("Invalid input, please try again.");
                        continue;
                    }
                }
            };
        }
        else {
            cls();
            println!("{}", node);
            println!("AI is thinking...");
            let mut alpha = mancala::Score::MIN;
            let mut beta = mancala::Score::MAX;
            let (wrapped_best_move, score) = minimax::minimax(&node, DEPTH, &mut alpha, &mut beta);
            let best_move = wrapped_best_move.unwrap();
            match node.full_move(&best_move) {
                Ok(_) => {
                    cls();
                    println!("{}", node);
                    println!("AI moved: {:?}", best_move);
                },
                Err(_) => { println!("Error occurred when playing move"); }
            }
        }
    }
    Ok(())
}
