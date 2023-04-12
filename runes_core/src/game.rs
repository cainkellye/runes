use crate::board::{Board, Field, Position};

const PLAYER1_SYMBOL: Field = Field::Wealth;
const PLAYER2_SYMBOL: Field = Field::Knowledge;

#[derive(Clone)]
pub struct Game<'a> {
    pub board: Board,
    pub player1: &'a dyn Player,
    pub player2: &'a dyn Player,
    pub game_over: bool,
    pub next_player: &'a dyn Player,
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

pub trait Player {
    fn set_symbol(&mut self, symbol: Field);
    fn make_move(&self, board: &Game) -> Move;
}

impl<'a> PartialEq for &'a dyn Player {
    fn eq(&self, other: &Self) -> bool {
        self as *const _ == other as *const _
    }
}

impl<'a> Game<'a> {
    pub fn new(player1: &'a mut dyn Player, player2: &'a mut dyn Player, board_size: usize) -> Self {
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
        }
    }

    pub fn start_loop(&mut self) {
        while !self.game_over {
            let player_move = self.next_player.make_move(&self);
            match self.apply_move(player_move) {
                Ok(symbol) => {
                    if symbol == Field::Joy {
                        self.game_over = true;
                        break;
                    }
                    self.next_player = if self.next_player == self.player1 {self.player2} else {self.player1};
                },
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
        if !self.is_valid_move(&move_to_apply) {
            return Err("Invalid move".to_string());
        }
        self.board.change(move_to_apply.position, move_to_apply.symbol);
        return Ok(move_to_apply.symbol);
    }

    fn next_player_symbol(&self) -> Field {
        if self.next_player == self.player1 {
            PLAYER1_SYMBOL
        } else {
            PLAYER2_SYMBOL
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
        if birth_count > 0 && gift_count > 0 {
            valid.push(Field::Wealth);
            valid.push(Field::Knowledge);
        }
        let player_symbol = self.next_player_symbol();
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
            Some(Field::Knowledge) | Some(Field::Wealth) => self.next_player_symbol(),
            Some(&f) => f,
            None => Field::Empty,
        }
    }

    pub fn generate_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        for i in 0..self.board.size {
            for j in 0..self.board.size {
                let pos = Position(i,j);
                if self.board.is_empty(&pos) {
                    moves.push(Move::new(pos, self.best_symbol_at(&pos)));
                }
            }
        }
        return moves;
    }
}