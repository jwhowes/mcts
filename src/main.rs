use std::io::{Write, stdin, stdout};

use crate::{
    board::{Board, Player, connect_four::ConnectFourBoard},
    mcts::MCTS,
};

mod board;
mod mcts;

const SIMS_PER_MOVE: usize = 50_000;

fn main() {
    let mut tree: MCTS<ConnectFourBoard> = MCTS::new();

    while tree.board().winner().is_none() {
        let mut s = String::new();

        let action_idx = match tree.board().player() {
            Player::PlayerOne => {
                tree.board().display();

                print!("Enter your move: ");
                stdout().flush().unwrap();
                stdin().read_line(&mut s).unwrap();

                s.trim().parse().unwrap()
            }

            Player::PlayerTwo => {
                tree.run_simulation(SIMS_PER_MOVE);

                let best_action = tree.get_best_action();

                println!("Computer move: {}", best_action);

                best_action
            }
        };

        tree.make_action(action_idx);
    }

    let winner = tree.board().winner().unwrap();

    tree.board().display();

    match winner {
        Player::PlayerOne => {
            println!("Player wins!")
        }
        Player::PlayerTwo => {
            println!("Computer wins!")
        }
    }
}
