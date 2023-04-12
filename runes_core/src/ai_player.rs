use std::marker::PhantomData;
use minimax::Strategy;

use crate::{game::{Move, Player, Game}, board::{Field, Position}};

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
    fn set_symbol(&mut self, symbol: Field) {
        self.symbol = symbol;
    }

    fn make_move<'a>(&self, game: &Game<'a>) -> Move {
        let mut strat = minimax::strategies::negamax::Negamax::new(Eval{ty: PhantomData::default()}, 10);
        strat.choose_move(game);
        let pos = Position(0,0);
        let best_move = game.best_symbol_at(&pos);
        return Move::new(pos, best_move);
    }
}

impl<'a> minimax::Game for Game<'a> {
    type S = Game<'a>;
    type M = Move;

    fn generate_moves(state: &Self::S, moves: &mut Vec<Self::M>) {
        moves.extend(state.generate_moves());
    }

    fn apply(state: &mut Self::S, m: Self::M) -> Option<Self::S> {
        let mut new_state = state.clone();
        match new_state.apply_move(m) {
            Ok(_) => Some(new_state),
            Err(_) => None,
        }
    }

    fn get_winner(state: &Self::S) -> Option<minimax::Winner> {
        if !state.game_over {
            None
        } else {
            Some(minimax::Winner::PlayerJustMoved)
        }
    }
}
struct Eval<'a> {
    ty: PhantomData<&'a ()>,
}
impl<'a> minimax::Evaluator for Eval<'a> {
    type G = Game<'a>;

    fn evaluate(&self, s: &<Self::G as minimax::Game>::S) -> minimax::Evaluation {
        let positions = (0..s.board.size).flat_map(|i| (0..s.board.size).map(move |j| Position(i,j)));
        positions.filter(|p| s.best_symbol_at(p) == Field::Joy)
        .count() as minimax::Evaluation
    }
}