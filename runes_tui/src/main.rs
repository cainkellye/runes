use std::io::{stdin, stdout, Write};

use runes_core::{
    //ai_player::{AiPlayer, AiPlayerMonte, Level},
    ai_player::{AiPlayerMonte, Level},
    board::Position,
    game::{Game, Move, Session},
    human_player::HumanPlayer,
};

fn main() {
    let player1 = Box::new(AiPlayerMonte::new(Level::Easy));
    //let player1 = Box::new(AiPlayer::new(Level::Medium));
    //let player2 = Box::new(AiPlayerMonte::new(Level::Easy));
    let player2 = Box::new(HumanPlayer::new("Apa".to_string(), make_move));
    let mut my_session: Session = Session::new(player1, player2, 13);
    let printout = |s: &Session| {
        println!("{}", s.game.board);
    };
    my_session.start_loop(printout);
    if let Some(s) = my_session.winner() {
        println!("Game over. Winner is {s}");
    } else {
        println!("Game over. Board is full.");
    }
    printout(&my_session);
}

// fn try_out() {
//     let mut player1 = HumanPlayer::new("Player1".to_string(), make_move);
//     let mut player2 = HumanPlayer::new("Player2".to_string(), make_move);

//     let mut my_game: Game = Game::new(&mut player1, &mut player2, 13);

//     my_game
//         .apply_move(Move::new(Position(7, 6), Field::Gift))
//         .unwrap();
//     my_game
//         .apply_move(Move::new(Position(6, 5), Field::Knowledge))
//         .unwrap();
//     println!("{:?}", my_game.board);

//     let best = my_game.best_symbol_at(&Position(2, 3));
//     println!("Best1 (2,3): {:?}", best);

//     my_game.next_player = my_game.player2;
//     let best = my_game.best_symbol_at(&Position(7, 6));
//     println!("Best2 (7,6): {:?}", best);

//     my_game.next_player = my_game.player1;
//     let best = my_game.best_symbol_at(&Position(7, 5));
//     println!("Best1 (7,5): {:?}", best);

//     my_game.next_player = my_game.player2;
//     let best = my_game.best_symbol_at(&Position(7, 5));
//     println!("Best2 (7,5): {:?}", best);
// }

fn make_move(player: &HumanPlayer, game: Game) -> Move {
    println!("Your Turn: {} ({})", player.name, player.symbol);
    loop {
        print!("Move (row, col): ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        if let Some(p) = input
            .split_once(',')
            .and_then(|s| Some(Position(s.0.trim().parse().ok()?, s.1.trim().parse().ok()?)))
        {
            let p = Position(p.0 - 1, p.1 - 1);
            let m = Move::new(p, game.best_symbol_at(&p));
            if game.is_valid_move(&m) {
                return m;
            } else {
                println!("Invalid move. Try again!");
            }
        } else {
            println!("Wrong input. Write row number and column number separated by a comma.");
        }
    }
}
