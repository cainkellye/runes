use crate::board::{Board, Field, Position};

const PLAYER1_SYMBOL: Field = Field::Wealth;
const PLAYER2_SYMBOL: Field = Field::Knowledge;

#[derive(Clone)]
pub struct Game<'a> {
    pub board: Board,
    pub player1: &'a dyn Player<'a>,
    pub player2: &'a dyn Player<'a>,
    pub game_over: bool,
    pub next_player: &'a dyn Player<'a>,
    last_move: Option<Move>,
}

#[derive(Clone, Copy)]
pub struct Move {
    pub position: Position,
    pub symbol: Field,
}

impl Move {
    pub fn new(position: Position, symbol: Field) -> Self {
        Self { position, symbol }
    }
}

pub trait Player<'a> {
    fn set_symbol(&mut self, symbol: Field);
    fn make_move(&self, board: &Game<'a>) -> Move;
}

impl<'a> PartialEq for &'a dyn Player<'a> {
    fn eq(&self, other: &Self) -> bool {
        *self as *const _ == *other as *const _
    }
}

impl<'a> Game<'a> {
    pub fn new(
        player1: &'a mut dyn Player<'a>,
        player2: &'a mut dyn Player<'a>,
        board_size: usize,
    ) -> Self {
        player1.set_symbol(PLAYER1_SYMBOL);
        player2.set_symbol(PLAYER2_SYMBOL);
        let mut board = Board::new(board_size);
        board.change(Position(board_size / 2, board_size / 2), Field::Birth);
        Self {
            board,
            player1,
            player2,
            game_over: false,
            next_player: player1,
            last_move: None,
        }
    }

    pub fn start_loop(&mut self, callback: fn(&Self)) {
        while !self.game_over {
            callback(&self);
            let player_move = self.next_player.make_move(&self);
            match self.apply_move(player_move) {
                Ok(_) => (),
                Err(s) => println!("{}", s),
            }
        }
    }

    pub fn reset(&mut self) {
        self.board.reset();
        self.game_over = false;
        self.next_player = self.player1;
    }

    pub fn apply_move(&mut self, move_to_apply: Move) -> Result<Field, String> {
        // if !self.is_valid_move(&move_to_apply) {
        //     return Err("Invalid move".to_string());
        // }
        self.board
            .change(move_to_apply.position, move_to_apply.symbol);
        self.last_move = Some(move_to_apply);
        if move_to_apply.symbol == Field::Joy || self.board.is_full() {
            self.game_over = true;
        }
        self.next_player = if self.next_player == self.player1 {
            self.player2
        } else {
            self.player1
        };
        return Ok(move_to_apply.symbol);
    }

    pub fn next_player_symbol(&self) -> Field {
        if self.next_player == self.player1 {
            PLAYER1_SYMBOL
        } else {
            PLAYER2_SYMBOL
        }
    }

    pub fn winner(&self) -> Option<String> {
        if !self.game_over {
            return None;
        }
        if let Some(Move {
            position: pos,
            symbol: Field::Joy,
        }) = self.last_move
        {
            if self.board.fields_around(&pos).contains(&Field::Wealth) {
                return Some("Player1".to_string());
            } else {
                return Some("Player2".to_string());
            }
        } else {
            return None;
        }
    }

    fn is_valid_move(&self, move_to_check: &Move) -> bool {
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
            return vec![];
        }
        let around = self.board.fields_around(position);

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

        let mut valid = Vec::new();
        if empty_count == around.len() {
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
        return valid;
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
        return moves;
    }
}
