use crate::board::{Board, Player};

const NUM_ROWS: usize = 6;
const NUM_COLS: usize = 7;

#[derive(Clone)]
pub struct ConnectFourBoard {
    top: [usize; NUM_COLS],
    grid: [[Option<Player>; NUM_ROWS]; NUM_COLS],
    player: Player,
    winner: Option<Player>,
}

impl Board for ConnectFourBoard {
    type Action = usize;

    fn new() -> Self {
        Self {
            top: [0; NUM_COLS],
            grid: [[None; NUM_ROWS]; NUM_COLS],
            player: Player::PlayerOne,
            winner: None,
        }
    }

    fn legal_actions(&self) -> Vec<Self::Action> {
        self.top
            .iter()
            .enumerate()
            .filter_map(|(i, t)| if *t < NUM_ROWS { Some(i) } else { None })
            .collect()
    }

    fn make_action(&mut self, action: &Self::Action) {
        let col = *action as i32;
        let row = self.top[*action] as i32;

        self.grid[*action][self.top[*action]] = Some(self.player);
        self.top[*action] += 1;

        let winning = 'winner: {
            // Horizontal check
            for i in 0..7 {
                let start = col + i - 3;

                if start >= 0 && start + 3 < NUM_COLS as i32 {
                    let mut found = true;
                    for j in 0..4 {
                        let x = (start + j) as usize;

                        if self.grid[x][row as usize] != Some(self.player) {
                            found = false;
                            break;
                        }
                    }

                    if found {
                        break 'winner true;
                    }
                }
            }

            // Vertical check
            for i in 0..7 {
                let start = row + i - 3;

                if start >= 0 && start + 3 < NUM_ROWS as i32 {
                    let mut found = true;
                    for j in 0..4 {
                        let y = (start + j) as usize;

                        if self.grid[col as usize][y] != Some(self.player) {
                            found = false;
                            break;
                        }
                    }

                    if found {
                        break 'winner true;
                    }
                }
            }

            // Diagonal up check
            for i in 0..7 {
                let start_x = col + i - 3;
                let start_y = row + i - 3;

                if start_x >= 0
                    && start_x + 3 < NUM_COLS as i32
                    && start_y >= 0
                    && start_y + 3 < NUM_ROWS as i32
                {
                    let mut found = true;
                    for j in 0..4 {
                        let x = (start_x + j) as usize;
                        let y = (start_y + j) as usize;

                        if self.grid[x][y] != Some(self.player) {
                            found = false;
                            break;
                        }
                    }

                    if found {
                        break 'winner true;
                    }
                }
            }

            // Diagonal down check
            for i in 0..7 {
                let start_x = col + i - 3;
                let start_y = row + 3 - i;

                if start_x >= 0
                    && start_x + 3 < NUM_COLS as i32
                    && start_y - 3 >= 0
                    && start_y < NUM_ROWS as i32
                {
                    let mut found = true;
                    for j in 0..4 {
                        let x = (start_x + j) as usize;
                        let y = (start_y - j) as usize;

                        if self.grid[x][y] != Some(self.player) {
                            found = false;
                            break;
                        }
                    }

                    if found {
                        break 'winner true;
                    }
                }
            }

            false
        };

        if winning {
            self.winner = Some(self.player);
        }

        self.player = match self.player {
            Player::PlayerOne => Player::PlayerTwo,
            Player::PlayerTwo => Player::PlayerOne,
        }
    }

    fn player(&self) -> Player {
        self.player
    }

    fn winner(&self) -> Option<Player> {
        if self.winner.is_none() && self.legal_actions().is_empty() {
            Some(Player::PlayerOne)
        } else {
            self.winner
        }
    }

    fn display(&self) {
        for y in 0..NUM_ROWS {
            for x in 0..NUM_COLS {
                let c = match self.grid[x][NUM_ROWS - y - 1] {
                    None => "-",
                    Some(Player::PlayerOne) => "o",
                    Some(Player::PlayerTwo) => "x",
                };

                print!("{} ", c);
            }

            println!();
        }

        for i in 0..NUM_COLS {
            print!("{} ", i);
        }

        println!();
    }
}
