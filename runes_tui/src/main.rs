use runes_core::{
    board::{Field, Position, Board},
    human_player::HumanPlayer,
    game::{Move, Game, GameRules},
};

fn main() {
    let player1 = HumanPlayer::new("Player1".to_string(), make_move);
    let player2 = HumanPlayer::new("Player2".to_string(), make_move);

    let mut my_game: Game<_, _> = Game::new(player1, player2, 13);
    my_game.apply_move(Move::new(Position(7, 6), Field::Gift)).unwrap();
    my_game.apply_move(Move::new(Position(6, 5), Field::Knowledge)).unwrap();
    println!("{:?}", my_game.board);

    let best = my_game.best_symbol_at(&Position(2, 3));
    println!("Best1 (2,3): {:?}", best);
    
    my_game.next_player = 2;
    let best = my_game.best_symbol_at(&Position(7, 6));
    println!("Best2 (7,6): {:?}", best);
    
    my_game.next_player = 1;
    let best = my_game.best_symbol_at(&Position(7, 5));
    println!("Best1 (7,5): {:?}", best);

    my_game.next_player = 2;
    let best = my_game.best_symbol_at(&Position(7, 5));
    println!("Best2 (7,5): {:?}", best);

    println!("Board clone");
    let b1 = Board::new(5);
    let b2 = b1.change(Position(2, 3), Field::Joy);
    println!("{:?}", b1);
    println!("{:?}", b2);
}

fn make_move(player: &HumanPlayer) -> Move {
    println!("Your Turn: {} ({:?})", player.name, player.symbol);
    Move::new(Position(0, 0), Field::Birth)
}
