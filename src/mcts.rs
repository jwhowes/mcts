use rand::{rng, seq::IndexedRandom};
use std::{
    f32::consts::SQRT_2,
    mem,
    sync::mpsc::{self, SendError},
};

use crate::board::{Board, Player};

const C_UCT: f32 = SQRT_2;

struct MCTSNode {
    num_wins: Vec<usize>,
    num_visits: Vec<usize>,

    children: Vec<Option<MCTSNode>>,
}

impl MCTSNode {
    fn value(&self) -> Vec<f32> {
        let ln_parent_visits: f32 = (self.num_visits.iter().sum::<usize>() as f32).ln();

        (0..self.children.len())
            .map(|i| {
                (self.num_wins[i] as f32 / self.num_visits[i] as f32)
                    + C_UCT * (ln_parent_visits / self.num_visits[i] as f32).sqrt()
            })
            .collect()
    }

    fn from_board(board: &impl Board) -> Self {
        let num_actions = board.legal_actions().len();

        Self {
            num_wins: (0..num_actions).into_iter().map(|_| 0).collect(),
            num_visits: (0..num_actions).into_iter().map(|_| 0).collect(),

            children: (0..num_actions).into_iter().map(|_| None).collect(),
        }
    }

    fn run_simulation(&mut self, mut board: impl Board) -> Player {
        if let Some(winner) = board.winner() {
            return winner;
        }

        let actions = board.legal_actions();

        let unvisited_children = self
            .children
            .iter()
            .enumerate()
            .filter_map(|(c_idx, c)| if c.is_none() { Some(c_idx) } else { None })
            .collect::<Vec<_>>();

        let action_idx = if unvisited_children.is_empty() {
            self.value()
                .iter()
                .enumerate()
                .max_by(|(_, v1), (_, v2)| {
                    if v1 < v2 {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                })
                .unwrap()
                .0
        } else {
            *unvisited_children.choose(&mut rng()).unwrap()
        };

        let player = board.player();

        board.make_action(&actions[action_idx]);

        if self.children[action_idx].is_none() {
            self.children[action_idx] = Some(Self::from_board(&board));
        }

        let winner = self.children[action_idx]
            .as_mut()
            .unwrap()
            .run_simulation(board);

        self.num_visits[action_idx] += 1;

        if winner == player {
            self.num_wins[action_idx] += 1;
        }

        winner
    }
}

pub struct MCTS<B: Board> {
    board: B,

    root: MCTSNode,
}

impl<B: Board> MCTS<B> {
    pub fn new() -> Self {
        let board = B::new();

        Self {
            root: MCTSNode::from_board(&board),
            board,
        }
    }

    pub fn run_simulation(&mut self, num_steps: usize) {
        for _ in 0..num_steps {
            self.root.run_simulation(self.board.clone());
        }
    }

    pub fn make_action(&mut self, action_idx: usize) {
        let action = &self.board.legal_actions()[action_idx];

        self.board.make_action(action);

        let new_root = self
            .root
            .children
            .remove(action_idx)
            .unwrap_or_else(|| MCTSNode::from_board(&self.board));

        let _ = mem::replace(&mut self.root, new_root);
    }

    pub fn get_best_action(&self) -> usize {
        self.root
            .num_visits
            .iter()
            .enumerate()
            .max_by_key(|(_, v)| **v)
            .unwrap()
            .0
    }

    pub fn board(&self) -> &B {
        &self.board
    }
}

const STEPS_PER_POLL: usize = 1000;
const STEPS_BEFORE_RESPONSE: usize = 10_000;

pub fn mcts_thread<B: Board>(
    tx: mpsc::Sender<usize>,
    rx: mpsc::Receiver<usize>,
) -> Result<(), SendError<usize>> {
    let mut tree = MCTS::<B>::new();

    while tree.board().winner().is_none() {
        match rx.try_recv() {
            Err(_) => {
                tree.run_simulation(STEPS_PER_POLL);
            }

            Ok(player_action) => {
                tree.make_action(player_action);

                tree.run_simulation(STEPS_BEFORE_RESPONSE);

                let action = tree.get_best_action();
                tree.make_action(action);

                tx.send(action)?;
            }
        }
    }

    Ok(())
}
