use runes_core::{
    board::{Field, Position},
    human_player::HumanPlayer,
    game::{Move, Game}, ai_player::AiPlayer,
};

fn main() {
    let mut player1 = AiPlayer::new(runes_core::ai_player::Level::Medium);
    let mut player2 = AiPlayer::new(runes_core::ai_player::Level::Medium);
    let mut my_game: Game = Game::new(&mut player1, &mut player2, 13);
    let printout = |s: &Game| {
        println!("{:?}", s.board);
    };
    my_game.start_loop(printout);
    println!("Game over. Winner is {}", my_game.winner().unwrap());
    printout(&my_game);
}

fn try_out() {
    let mut player1 = HumanPlayer::new("Player1".to_string(), make_move);
    let mut player2 = HumanPlayer::new("Player2".to_string(), make_move);

    let mut my_game: Game = Game::new(&mut player1, &mut player2, 13);

    my_game.apply_move(Move::new(Position(7, 6), Field::Gift)).unwrap();
    my_game.apply_move(Move::new(Position(6, 5), Field::Knowledge)).unwrap();
    println!("{:?}", my_game.board);

    let best = my_game.best_symbol_at(&Position(2, 3));
    println!("Best1 (2,3): {:?}", best);
    
    my_game.next_player = my_game.player2;
    let best = my_game.best_symbol_at(&Position(7, 6));
    println!("Best2 (7,6): {:?}", best);
    
    my_game.next_player = my_game.player1;
    let best = my_game.best_symbol_at(&Position(7, 5));
    println!("Best1 (7,5): {:?}", best);

    my_game.next_player = my_game.player2;
    let best = my_game.best_symbol_at(&Position(7, 5));
    println!("Best2 (7,5): {:?}", best);
}

fn make_move(player: &HumanPlayer) -> Move {
    println!("Your Turn: {} ({:?})", player.name, player.symbol);
    Move::new(Position(0, 0), Field::Birth)
}
