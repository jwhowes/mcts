pub mod connect_four;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Player {
	PlayerOne,
	PlayerTwo
}

pub trait Board: Clone {
	type Action;

	fn new() -> Self;
	fn legal_actions(&self) -> Vec<Self::Action>;
	fn make_action(&mut self, action: &Self::Action);
	fn player(&self) -> Player;
	fn winner(&self) -> Option<Player>;
	fn display(&self);
}