use std::ops::{DerefMut, Deref};

use crate::board::{Board, Field, Position};

pub const PLAYER_SYMBOLS: [Field; 2] = [Field::Wealth, Field::Knowledge];


#[derive(Clone)]
pub struct Game {
    pub board: Board,
    pub game_over: bool,
    pub next_player: u8,
    last_move: Option<Move>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub position: Position,
    pub symbol: Field,
}

impl Move {
    pub fn new(position: Position, symbol: Field) -> Self {
        Self { position, symbol }
    }
}


impl Deref for Move {
    type Target = Move;

    fn deref(&self) -> &Self::Target {
        self
    }
}
impl DerefMut for Move {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}

pub trait Player {
    fn set_symbol(&mut self, symbol: Field);
    fn make_move(&self, board: Game) -> Move;
    fn name(&self) -> String;
}

// impl PartialEq for &'a dyn Player {
//     fn eq(&self, other: &Self) -> bool {
//         *self as *const _ == *other as *const _
//     }
// }

impl Game {
    pub fn new(board_size: usize) -> Self {
        let mut board = Board::new(board_size);
        board.change(Position(board_size / 2, board_size / 2), Field::Birth);
        Self {
            board,
            game_over: false,
            next_player: 0,
            last_move: None,
        }
    }

    pub fn reset(&mut self) {
        self.board.reset();
        self.game_over = false;
        self.next_player = 0;
    }

    pub fn winner(&self) -> Option<u8> {
        if !self.game_over {
            None
        } else if let Some(Move {
            position: pos,
            symbol: Field::Joy,
        }) = self.last_move
        {
            if self.board.fields_around(&pos).contains(&Field::Wealth) {
                Some(0)
            } else {
                Some(1)
            }
        } else {
            None
        }
    }

    pub fn apply_best_move_at(&mut self, position: &Position)  -> Result<Field, String> {
        self.apply_move(Move::new(*position, self.best_symbol_at(position)))
    }

    pub fn apply_move(&mut self, move_to_apply: Move) -> Result<Field, String> {
        if !self.is_valid_move(&move_to_apply) {
            return Err("Invalid move".to_string());
        }
        self.board
            .change(move_to_apply.position, move_to_apply.symbol);
        self.last_move = Some(move_to_apply);
        if move_to_apply.symbol == Field::Joy || self.board.is_full() {
            self.game_over = true;
        }
        self.next_player = 1 - self.next_player;
        Ok(move_to_apply.symbol)
    }

    pub fn next_player_symbol(&self) -> Field {
        PLAYER_SYMBOLS[self.next_player as usize]
    }

    pub fn is_valid_move(&self, move_to_check: &Move) -> bool {
        let pos = &move_to_check.position;
        //boundary check
        pos.0 < self.board.size && pos.1 < self.board.size
        //target is empty
            && self.board.is_empty(pos)
        //symbol is valid
            && self.valid_symbols_at(pos).contains(&move_to_check.symbol)
    }

    pub fn valid_symbols_at(&self, position: &Position) -> Vec<Field> {
        if !self.board.is_empty(position) {
            return Vec::new();
        }
        let (empty_count, birth_count, gift_count, wealth_count, knowledge_count) =
            self.board.count_around(position);

        let mut valid = Vec::new();
        if birth_count == 0 && gift_count == 0 && wealth_count == 0 && knowledge_count == 0 {
            valid.push(Field::Birth);
        } else {
            valid.push(Field::Gift);
        }
        let player_symbol = self.next_player_symbol();
        if birth_count > 0 && gift_count > 0 {
            valid.push(player_symbol);
        }
        if birth_count == 1
            && gift_count == 1
            && empty_count == 5
            && (knowledge_count == 1 && player_symbol == Field::Knowledge
                || wealth_count == 1 && player_symbol == Field::Wealth)
        {
            valid.push(Field::Joy);
        }
        valid
    }

    pub fn best_symbol_at(&self, position: &Position) -> Field {
        match self.valid_symbols_at(position).iter().max() {
            Some(&f) => f,
            None => Field::Empty,
        }
    }

    pub fn generate_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        for i in 0..self.board.size {
            for j in 0..self.board.size {
                let pos = Position(i, j);
                if self.board.is_empty(&pos) {
                    moves.push(Move::new(pos, self.best_symbol_at(&pos)));
                }
            }
        }
        let player_symbol = self.next_player_symbol();
        let mut my_winning = Vec::new();
        let mut opp_winning = Vec::new();
        for &m in &moves {
            let (empty_count, birth_count, gift_count, wealth_count, _) =
                self.board.count_around(&m.position);
            if birth_count == 1 && gift_count == 1 && empty_count == 5 {
                if wealth_count == 1 {
                    if player_symbol == Field::Wealth {
                        my_winning.push(m);
                    } else {
                        opp_winning.push(m);
                    }
                } else if player_symbol == Field::Knowledge {
                    my_winning.push(m);
                } else {
                    opp_winning.push(m);
                }
            }
        }

        if !my_winning.is_empty() {
            return my_winning;
        } else if !opp_winning.is_empty() {
            moves.retain(|m| opp_winning.iter().any(|o| o.position.near(&m.position)));
        }
        moves
    }
}
