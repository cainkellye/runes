use crate::{
    board::Field,
    game::{Game, Move, Player},
};

pub struct HumanPlayer {
    pub name: String,
    pub make_move_callback: fn(player: &HumanPlayer) -> Move,
    pub symbol: Field,
}

impl HumanPlayer {
    pub fn new(name: String, make_move_callback: fn(player: &HumanPlayer) -> Move) -> Self {
        HumanPlayer {
            name,
            make_move_callback,
            symbol: Field::Empty,
        }
    }
}

impl Player for HumanPlayer {
    fn set_symbol(&mut self, symbol: Field) {
        self.symbol = symbol;
    }

    fn make_move(&self, _: Game) -> Move {
        (self.make_move_callback)(self)
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
