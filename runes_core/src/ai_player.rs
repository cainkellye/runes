use crate::{game::{Move, Player, GameRules}, board::{Field, Position}};

pub enum Level {
    Easy,
    Medium,
    Hard,
    VeryHard,
}

pub struct AiPlayer {
    pub symbol: Field,
    pub level: Level,
}

impl AiPlayer {
    pub fn new(level: Level) -> Self {
        AiPlayer { symbol: Field::Empty, level }
    }
}

impl Player for AiPlayer {
    fn make_move(&self, game: &impl GameRules) -> Move {
        let pos = Position(0,0);
        let best_move = game.best_symbol_at(&pos);
        return Move::new(pos, best_move);
    }

    fn set_symbol(&mut self, symbol: Field) {
        self.symbol = symbol;
    }
}
