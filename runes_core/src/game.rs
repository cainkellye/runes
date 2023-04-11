use crate::board::{Board, Field, Position};

const PLAYER1_SYMBOL: Field = Field::Wealth;
const PLAYER2_SYMBOL: Field = Field::Knowledge;

pub struct Game< P1: Player, P2: Player> {
    pub board: Board,
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

pub trait GameRules {
    fn valid_symbols_at(&self, position: &Position) -> Vec<Field>;
    fn best_symbol_at(&self, position: &Position) -> Field;
    fn get_board(&self) -> &Board;
}

pub trait Player {
    fn set_symbol(&mut self, symbol: Field);
    fn make_move(&self, board: &impl GameRules) -> Move;
}

impl< P1: Player, P2: Player> Game<P1, P2> {
    pub fn new(mut player1: P1, mut player2: P2, board_size: usize) -> Self {
        player1.set_symbol(PLAYER1_SYMBOL);
        player2.set_symbol(PLAYER2_SYMBOL);
        Self {
            board: Board::new(board_size)
                .change(Position(board_size / 2, board_size / 2), Field::Birth),
            player1,
            player2,
            game_over: false,
            next_player: 1,
        }
    }

    pub fn start_loop(&mut self) {
        while !self.game_over {
            let player_move = if self.next_player == 1 {
                self.player1.make_move(self as &Self)
            } else {
                self.player2.make_move(self as &Self)
            };
            match self.apply_move(player_move) {
                Ok(symbol) => {
                    if symbol == Field::Joy {
                        self.game_over = true;
                        break;
                    }
                    self.next_player = 3 - self.next_player;
                },
                Err(s) => println!("{}", s),
            }
        }
    }

    pub fn reset(&mut self) {
        self.board.reset();
        self.game_over = false;
        self.next_player = 1;
    }

    pub fn apply_move(&mut self, move_to_apply: Move) -> Result<Field, String> {
        if !self.is_valid_move(&move_to_apply) {
            return Err("Invalid move".to_string());
        }
        self.board = self
            .board
            .change(move_to_apply.position, move_to_apply.symbol);
        return Ok(move_to_apply.symbol);
    }

    fn next_player_symbol(&self) -> Field {
        if self.next_player == 1 {
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
}

impl< P1: Player, P2: Player> GameRules for Game<P1, P2> {
    fn valid_symbols_at(&self, position: &Position) -> Vec<Field> {
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

    fn best_symbol_at(&self, position: &Position) -> Field {
        match self.valid_symbols_at(position).iter().max() {
            Some(Field::Knowledge) | Some(Field::Wealth) => self.next_player_symbol(),
            Some(&f) => f,
            None => Field::Empty,
        }
    }

    fn get_board(&self) -> &Board {
        return &self.board;
    }
}