use crate::session::{Move, Player};

pub struct HumanPlayer {
    pub name: String,
    pub make_move_callback: fn(player: &HumanPlayer) -> Move,
}

impl Player for HumanPlayer {
    fn make_move<const SIZE: usize>(&self, _: &crate::board::Board<SIZE>) -> Move {
        (self.make_move_callback)(&self)
    }
}
