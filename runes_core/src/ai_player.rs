use minimax::{Negamax, Strategy};
use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use crate::{
    board::{Field, Position},
    game::{Game, Move, Player},
};

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Level {
    Easy = 1,
    Medium = 2,
    Hard = 3,
    VeryHard = 4,
}

pub struct AiPlayer<'a> {
    pub symbol: Field,
    pub level: Level,
    strategy: Rc<RefCell<Negamax<Eval<'a>>>>,
}

impl<'a> AiPlayer<'a> {
    pub fn new(level: Level) -> Self {
        AiPlayer {
            symbol: Field::Empty,
            level,
            strategy: Rc::new(RefCell::new(Negamax::new(Eval::default(), level as u8))),
        }
    }
}

impl<'a> Player<'a> for AiPlayer<'a> {
    fn set_symbol(&mut self, symbol: Field) {
        self.symbol = symbol;
    }

    fn make_move(&self, game: &Game<'a>) -> Move {
        let mut strategy = self.strategy.borrow_mut();
        let ai_move = strategy.choose_move(game);
        return ai_move.unwrap();
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
#[derive(Default)]
struct Eval<'a> {
    ty: PhantomData<&'a ()>,
}
impl<'a> minimax::Evaluator for Eval<'a> {
    type G = Game<'a>;

    fn evaluate(&self, s: &<Self::G as minimax::Game>::S) -> minimax::Evaluation {
        let positions =
            (0..s.board.size).flat_map(|i| (0..s.board.size).map(move |j| Position(i, j)));
        let mut score = 10;
        let next_player_symbol = s.next_player_symbol();
        for pos in positions {
            let around = s.board.fields_around(&pos);
            let mut empty_count = 0;
            let mut birth_count = 0;
            let mut gift_count = 0;
            let mut wealth_count = 0;
            let mut knowledge_count = 0;

            for field in &around {
                match field {
                    Field::Empty => empty_count += 1,
                    Field::Birth => birth_count += 1,
                    Field::Gift => gift_count += 1,
                    Field::Wealth => wealth_count += 1,
                    Field::Knowledge => knowledge_count += 1,
                    Field::Joy => (),
                }
            }

            if birth_count == 1 && gift_count == 1 && empty_count == 5 {
                if wealth_count == 1 {
                    score *= 10;
                } else if knowledge_count == 1 {
                    score /= 10;
                }
            }
        }
        (if next_player_symbol == Field::Wealth {
            score
        } else {
            -score
        }) as minimax::Evaluation
    }
}
