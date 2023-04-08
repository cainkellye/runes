use runes_core::{
    board::{Board, Field, Position},
    human_player::HumanPlayer,
    session::{Move, Session},
};

fn main() {
    let b: Board<5> = Board::new()
        .change(Position(2, 2), Field::Birth)
        .change(Position(1, 3), Field::Gift)
        .change(Position(3, 1), Field::Gift)
        .change(Position(3, 2), Field::Knowledge)
        .change(Position(2, 3), Field::Wealth);
    println!("Hello, board!");
    println!("{:?}", b);

    println!("{:?}", b.fields_around(&Position(1, 2)));
    println!("{:?}", b.fields_around(&Position(4, 0)));
    println!("{:?}", b.fields_around(&Position(1, 4)));

    let player1 = HumanPlayer {
        name: "Player1".to_string(),
        make_move_callback: make_move,
    };
    let player2 = HumanPlayer {
        name: "Player2".to_string(),
        make_move_callback: make_move,
    };
    let my_session: Session<13, _, _> = Session::new(player1, player2);

    println!("{:?}", my_session.board);
}

fn make_move(player: &HumanPlayer) -> Move {
    println!("Your Turn: {}", player.name);
    Move::new(Position(0, 0), Field::Birth)
}
