use runes_core::board::{Board, Field, Position};

fn main() {
    let mut b: Board<5> = Board::new();
    b = b.change(Position(2,2), Field::Birth);
    b = b.change(Position(1,3), Field::Gift);
    println!("Hello, board!");
    println!("{:?}", b);
}
