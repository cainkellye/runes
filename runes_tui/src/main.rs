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

    let player1 = HumanPlayer::new("Player1".to_string(), make_move);
    let player2 = HumanPlayer::new("Player2".to_string(), make_move);

    let mut my_session: Session<13, _, _> = Session::new(player1, player2);
    my_session.apply_move(Move::new(Position(7, 6), Field::Gift)).unwrap();
    my_session.apply_move(Move::new(Position(6, 5), Field::Knowledge)).unwrap();
    println!("{:?}", my_session.board);

    let best = my_session.best_symbol_at(&Position(2, 3));
    println!("Best1 (2,3): {:?}", best);
    
    my_session.next_player = 2;
    let best = my_session.best_symbol_at(&Position(7, 6));
    println!("Best2 (7,6): {:?}", best);
    
    my_session.next_player = 1;
    let best = my_session.best_symbol_at(&Position(7, 5));
    println!("Best1 (7,5): {:?}", best);

    my_session.next_player = 2;
    let best = my_session.best_symbol_at(&Position(7, 5));
    println!("Best2 (7,5): {:?}", best);
}

fn make_move(player: &HumanPlayer) -> Move {
    println!("Your Turn: {} ({:?})", player.name, player.symbol);
    Move::new(Position(0, 0), Field::Birth)
}
