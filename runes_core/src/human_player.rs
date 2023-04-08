use crate::{session::{Move, Player}, board::Field};

pub struct HumanPlayer {
    pub name: String,
    pub make_move_callback: fn(player: &HumanPlayer) -> Move,
    pub symbol: Field,
}

impl HumanPlayer {
    pub fn new(name: String, make_move_callback: fn(player: &HumanPlayer) -> Move) -> Self {
        HumanPlayer { name, make_move_callback, symbol: Field::Empty }
    }
}

impl Player for HumanPlayer {
    fn make_move<const SIZE: usize>(&self, _: &crate::board::Board<SIZE>) -> Move {
        (self.make_move_callback)(&self)
    }

    fn set_symbol(&mut self, symbol: Field) {
        self.symbol = symbol;
    }
}
