use runes_core::board::{Board, Field, Position};

fn main() {
    let mut b: Board<5> = Board::new();
    b = b
        .change(Position(2, 2), Field::Birth)
        .change(Position(1, 3), Field::Gift)
        .change(Position(3, 1), Field::Gift)
        .change(Position(3, 2), Field::Knowledge)
        .change(Position(2, 3), Field::Wealth);
    println!("Hello, board!");
    println!("{:?}", b);

    println!("{:?}", b.fields_around(Position(1, 2)));
    println!("{:?}", b.fields_around(Position(4, 0)));
    println!("{:?}", b.fields_around(Position(1, 4)));
}
