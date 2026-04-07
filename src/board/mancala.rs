use crate::board::{Board, Player};

const NUM_PITS: usize = 6;
const INIT_COUNTERS: usize = 4;

#[derive(Clone)]
pub struct MancalaBoard {
	player: Player,
	pits: [[usize; NUM_PITS]; 2],
	scores: [usize; 2]
}

impl MancalaBoard {
	fn player_idx(&self) -> usize {
		match &self.player {
			Player::PlayerOne => 0,
			Player::PlayerTwo => 1
		}
	}
}

fn direction(player: &Player) -> i32 {
	match player {
		Player::PlayerOne =>  1,
		Player::PlayerTwo => -1
	}
}

impl Board for MancalaBoard {
	type Action = usize;

	fn new() -> Self {
		Self {
			player: Player::PlayerOne,

			pits: [[INIT_COUNTERS; NUM_PITS]; 2],
			scores: [0; 2]
		}
	}

	fn legal_actions(&self) -> Vec<Self::Action> {
		self.pits[self.player_idx()].iter().enumerate()
			.filter_map(|(idx, p)| if *p == 0 {
				None
			} else {
				Some(idx)
			}).collect()
	}

	fn player(&self) -> Player {
		self.player
	}

	fn winner(&self) -> Option<Player> {
		let num_remaining_counters: [usize; 2] = [
			self.pits[0].iter().sum(),
			self.pits[1].iter().sum()
		];

		if num_remaining_counters[0] == 0 || num_remaining_counters[1] == 0 {
			let scores = [
				self.scores[0] + num_remaining_counters[0],
				self.scores[1] + num_remaining_counters[1]
			];

			// TODO: Board should handle ties

			if scores[0] > scores[1] {
				Some(Player::PlayerOne)
			} else {
				Some(Player::PlayerTwo)
			}
		} else {
			None
		}
	}

	fn make_action(&mut self, action: &Self::Action) {
		let mut player = self.player;

		let mut pit_idx = *action;

		let mut num_counters = self.pits[self.player_idx()][*action];

		self.pits[self.player_idx()][*action] = 0;

		while num_counters > 0 {
			let dir = direction(&player);

			if pit_idx == NUM_PITS - 1 && dir > 0 {
				if self.player == Player::PlayerOne {
					self.scores[0] += 1;
					num_counters -= 1;

					if num_counters == 0 {
						return;
					}
				}

				player = Player::PlayerTwo;
			} else if pit_idx == 0 && dir < 0 {
				if self.player == Player::PlayerTwo {
					self.scores[1] += 1;
					num_counters -= 1;

					if num_counters == 0 {
						return;
					}
				}

				player = Player::PlayerOne;
			} else {
				pit_idx = (pit_idx as i32 + dir) as usize;
			}

			self.pits[match player {
				Player::PlayerOne => 0,
				Player::PlayerTwo => 1
			}][pit_idx] += 1;
			num_counters -= 1;
		}

		if player == self.player && self.pits[self.player_idx()][pit_idx] == 1 && self.pits[1 - self.player_idx()][pit_idx] > 0 {
			self.scores[self.player_idx()] += self.pits[1 - self.player_idx()][pit_idx] + 1;

			self.pits[1 - self.player_idx()][pit_idx] = 0;
			self.pits[self.player_idx()][pit_idx] = 0;
		}

		self.player = match self.player {
			Player::PlayerOne => Player::PlayerTwo,
			Player::PlayerTwo => Player::PlayerOne
		}
	}

	fn display(&self) {
		println!("Scores: {} {}", self.scores[0], self.scores[1]);

		println!("{:?}", self.pits[1]);
		println!("{:?}", self.pits[0]);
	}
}