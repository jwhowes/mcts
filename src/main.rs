use std::{
    io::{Write, stdin, stdout},
    sync::mpsc,
    thread,
};

use crate::{
    board::{Board, Player, connect_four::ConnectFourBoard},
    mcts::mcts_thread,
};

mod board;
mod mcts;

fn main() -> anyhow::Result<()> {
    let mut board = ConnectFourBoard::new();

    let (p2c_tx, p2c_rx) = mpsc::channel::<usize>();
    let (c2p_tx, c2p_rx) = mpsc::channel::<usize>();

    let cpu_thread = thread::spawn(move || mcts_thread::<ConnectFourBoard>(c2p_tx, p2c_rx));

    while board.winner().is_none() {
        board.display();

        let mut s = String::new();

        print!("Enter your move: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut s).unwrap();

        let player_action: usize = s.trim().parse()?;

        board.make_action(&player_action);

        p2c_tx.send(player_action)?;

        let cpu_action_idx = c2p_rx.recv()?;
        let cpu_action = board.legal_actions()[cpu_action_idx];

        println!("Computer move: {}", cpu_action);

        board.make_action(&cpu_action);
    }

    let winner = board.winner().unwrap();

    board.display();

    match winner {
        Player::PlayerOne => {
            println!("Player wins!");
        }
        Player::PlayerTwo => {
            println!("Computer wins!");
        }
    }

    let _ = cpu_thread.join();

    Ok(())
}
