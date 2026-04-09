use std::{
    fmt::Display,
    io::{Write, stdin, stdout},
    str::FromStr,
    sync::mpsc,
    thread,
};

use crate::{
    board::{Board, Player, connect_four::ConnectFourBoard},
    mcts::mcts_thread,
};

mod board;
mod mcts;

fn play_vs_cpu<B: Board>() -> anyhow::Result<()>
where
    B::Action: 'static + Display + Send + Sync + FromStr,
    <B::Action as FromStr>::Err: Send + Sync + std::error::Error,
{
    let mut board = B::new();

    let (p2c_tx, p2c_rx) = mpsc::channel::<B::Action>();
    let (c2p_tx, c2p_rx) = mpsc::channel::<B::Action>();

    let cpu_thread = thread::spawn(move || mcts_thread::<B>(c2p_tx, p2c_rx));

    while board.winner().is_none() {
        board.display();

        let mut s = String::new();

        print!("Enter your move: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut s).unwrap();

        let player_action: B::Action = s.trim().parse()?;

        board.make_action(&player_action);

        p2c_tx.send(player_action)?;

        let cpu_action = c2p_rx.recv()?;

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

fn main() -> anyhow::Result<()> {
    play_vs_cpu::<ConnectFourBoard>()
}
