use crate::board::{Board, Field, Position};

const PLAYER1_SYMBOL: Field = Field::Wealth;
const PLAYER2_SYMBOL: Field = Field::Knowledge;

pub struct Session<const SIZE: usize, P1: Player, P2: Player> {
    pub board: Board<SIZE>,
    pub player1: P1,
    pub player2: P2,
    pub game_over: bool,
    pub next_player: u8,
}

pub struct Move {
    position: Position,
    symbol: Field,
}

impl Move {
    pub fn new(position: Position, symbol: Field) -> Self {
        Self { position, symbol }
    }
}

pub trait Player {
    fn set_symbol(&mut self, symbol: Field);
    fn make_move<const SIZE: usize>(&self, board: &Board<SIZE>) -> Move;
}

impl<const SIZE: usize, P1: Player, P2: Player> Session<SIZE, P1, P2> {
    pub fn new(mut player1: P1, mut player2: P2) -> Self {
        player1.set_symbol(PLAYER1_SYMBOL);
        player2.set_symbol(PLAYER2_SYMBOL);
        Self {
            board: Board::<SIZE>::new()
                .change(Position(SIZE / 2, SIZE / 2), Field::Birth),
            player1,
            player2,
            game_over: false,
            next_player: 1,
        }
    }

    pub fn apply_move(&mut self, move_to_apply: Move) -> Result<(), String> {
        if !self.is_valid_move(&move_to_apply) {
            return Err("Invalid move".to_string());
        }
        self.board = self
            .board
            .change(move_to_apply.position, move_to_apply.symbol);
        return Ok(());
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
        if birth_count == 1 && gift_count == 1 && empty_count == 5
            && (knowledge_count == 1 && player_symbol == Field::Knowledge 
                || wealth_count == 1 && player_symbol == Field::Wealth) {
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

    fn next_player_symbol(&self) -> Field {
        if self.next_player == 1 { PLAYER1_SYMBOL } else { PLAYER2_SYMBOL }
    }

    fn is_valid_move(&self, move_to_check: &Move) -> bool {
        let pos = &move_to_check.position;
        //boundary check
        pos.0 < SIZE && pos.1 < SIZE
        //target is empty
            && self.board.is_empty(pos)
        //symbol is valid
            && self.valid_symbols_at(pos).contains(&move_to_check.symbol)
    }
}
