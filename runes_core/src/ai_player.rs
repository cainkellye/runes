use minimax::{MCTSOptions, MonteCarloTreeSearch, Negamax, Random, Strategy};
use std::cell::RefCell;

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

pub struct AiPlayerRandom {
    pub symbol: Field,
    pub level: Level,
    strategy: RefCell<Random<Game>>,
}

impl AiPlayerRandom {
    pub fn new(level: Level) -> Self {
        AiPlayerRandom {
            symbol: Field::Empty,
            level,
            strategy: RefCell::new(Random::new()),
        }
    }
}

impl Player for AiPlayerRandom {
    fn set_symbol(&mut self, symbol: Field) {
        self.symbol = symbol;
    }

    fn make_move(&self, game: Game) -> Move {
        let ai_move = self.strategy.borrow_mut().choose_move(&game);
        ai_move.unwrap()
    }

    fn name(&self) -> String {
        format!("AI {}", self.symbol)
    }
}

pub struct AiPlayerMonte {
    pub symbol: Field,
    pub level: Level,
    strategy: RefCell<MonteCarloTreeSearch<Game>>,
}

impl AiPlayerMonte {
    pub fn new(level: Level) -> Self {
        AiPlayerMonte {
            symbol: Field::Empty,
            level,
            strategy: RefCell::new(MonteCarloTreeSearch::new(
                MCTSOptions::default()
                    .with_num_threads(3),
            )),
        }
    }
}

impl Player for AiPlayerMonte {
    fn set_symbol(&mut self, symbol: Field) {
        self.symbol = symbol;
    }

    fn make_move(&self, game: Game) -> Move {
        let ai_move = self.strategy.borrow_mut().choose_move(&game);
        ai_move.unwrap()
    }

    fn name(&self) -> String {
        format!("AI {}", self.symbol)
    }
}

pub struct AiPlayer {
    pub symbol: Field,
    pub level: Level,
    strategy: RefCell<Negamax<Eval>>,
}

impl AiPlayer {
    pub fn new(level: Level) -> Self {
        AiPlayer {
            symbol: Field::Empty,
            level,
            strategy: RefCell::new(Negamax::new(Eval::default(), level as u8)),
        }
    }
}

impl Player for AiPlayer {
    fn set_symbol(&mut self, symbol: Field) {
        self.symbol = symbol;
    }

    fn make_move(&self, game: Game) -> Move {
        let mut strategy = self.strategy.borrow_mut();
        let ai_move = strategy.choose_move(&game);
        ai_move.unwrap()
    }

    fn name(&self) -> String {
        format!("AI {}", self.symbol)
    }
}

impl minimax::Game for Game {
    type S = Game;
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
#[derive(Default, Clone)]
struct Eval {}
impl minimax::Evaluator for Eval {
    type G = Game;

    fn evaluate(&self, s: &<Self::G as minimax::Game>::S) -> minimax::Evaluation {
        let positions =
            (0..s.board.size).flat_map(|i| (0..s.board.size).map(move |j| Position(i, j)));
        let mut w_opp = 0;
        let mut k_opp = 0;
        let next_player_symbol = s.next_player_symbol();
        for pos in positions {
            let (empty_count, birth_count, gift_count, wealth_count, knowledge_count) =
                s.board.count_around(&pos);

            if birth_count == 1 && gift_count == 1 && empty_count == 5 {
                if wealth_count == 1 {
                    w_opp += 1;
                } else if knowledge_count == 1 {
                    k_opp += 1;
                }
            }
        }
        (if next_player_symbol == Field::Wealth {
            w_opp * 3 - k_opp
        } else {
            k_opp * 3 - w_opp
        }) as minimax::Evaluation
    }
}
