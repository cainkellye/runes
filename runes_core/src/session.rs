use crate::board::{Board, Field, Position};

pub struct Session<const SIZE: usize, P1: Player, P2: Player> {
    pub board: Board<SIZE>,
    pub player1: P1,
    pub player2: P2,
    pub game_over: bool,
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
    fn make_move<const SIZE: usize>(&self, board: &Board<SIZE>) -> Move;
}

impl<const SIZE: usize, P1: Player, P2: Player> Session<SIZE, P1, P2> {
    pub fn new(player1: P1, player2: P2) -> Self {
        Self {
            board: Board::<SIZE>::new(),
            player1,
            player2,
            game_over: false,
        }
    }

    pub fn apply_move(&mut self, move_to_apply: Move) -> Result<(),String> {
        if !self.is_valid_move(&move_to_apply) { 
            return Err("Invalid move".to_string());
        }
        self.board = self
            .board
            .change(move_to_apply.position, move_to_apply.symbol);
        return Ok(())
    }

    pub fn valid_symbols_at(&self, position: &Position) -> Vec<Field> {
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
        if birth_count == 1 && gift_count == 1 && empty_count == 5 && knowledge_count == 1
            || wealth_count == 1
        {
            valid.push(Field::Joy);
        }
        return valid;
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
