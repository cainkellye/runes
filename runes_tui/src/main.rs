use runes_core::{
    ai_player::{AiPlayer, Level},
    board::{Field, Position},
    game::{Game, Move},
    human_player::HumanPlayer,
};

fn main() {
    let mut player1 = AiPlayer::new(Level::Hard);
    let mut player2 = AiPlayer::new(Level::Hard);
    let mut my_game: Game = Game::new(&mut player1, &mut player2, 9);
    let printout = |s: &Game| {
        println!("{:?}", s.board);
    };
    my_game.start_loop(printout);
    if let Some(s) = my_game.winner() {
        println!("Game over. Winner is {}", s);
    } else {
        println!("Game over. Board is full.");
    }
    printout(&my_game);
}

fn try_out() {
    let mut player1 = HumanPlayer::new("Player1".to_string(), make_move);
    let mut player2 = HumanPlayer::new("Player2".to_string(), make_move);

    let mut my_game: Game = Game::new(&mut player1, &mut player2, 13);

    my_game
        .apply_move(Move::new(Position(7, 6), Field::Gift))
        .unwrap();
    my_game
        .apply_move(Move::new(Position(6, 5), Field::Knowledge))
        .unwrap();
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
