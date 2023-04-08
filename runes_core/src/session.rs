use crate::board::{Board, Field, Position};

pub struct Session<P1: Player, P2: Player, const SIZE: usize> {
    board: Board<SIZE>,
    player1: P1,
    player2: P2,
}

pub struct Move {
    position: Position,
    symbol: u8,
}

pub trait Player {
    fn make_move<const SIZE: usize>(&self, board: &Board<SIZE>) -> Move;
}
