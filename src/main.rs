use std::io;

use tic_tac_toe::Player;
use tic_tac_toe::Board;
use tic_tac_toe::player_move;


fn main() {

    let player1 = Player::new("x", 1);
    let player2 = Player::new("o", 2);
    let mut board = Board::new();
    
    println!("Let's play tic-tac-toe!");

    let mut turn: u32 = 1;
    let mut marker: String;
    let mut index_num: usize;

    let first: String;
    let second: String;

    if player1.turn % 2 == 0 {
        first = String::from("Player2");
        second = String::from("Player1");
    } else {
        first = String::from("Player1");
        second = String::from("Player2");
     }

    loop {

        if Player::check_for_winner(&board.a) == true || Player::check_for_winner(&board.b) == true || Player::check_for_winner(&board.c) == true {
            if turn % 2 == 0 {
                println!("{} wins!", first);
            } else {
                println!("{} wins!", second);
            }

            board.show_board();

            break;
        }

        println!("Where do you want to move?");

        board.show_board();

        let mut make_move = String::new();

        io::stdin().read_line(&mut make_move).expect("Invalid move");

        if turn % 2 == 0 {
            if player_move(&mut board, make_move, &player2) == true {
                turn += 1;
            } 

        } else {
            if player_move(&mut board, make_move, &player1) == true {
                turn += 1;
            } 
        }
    }
}