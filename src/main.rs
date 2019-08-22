use std::io;

use tic_tac_toe::Player;
use tic_tac_toe::Board;
use tic_tac_toe::player_move;
use tic_tac_toe::check_for_winner;


fn main() {

    let mut marker1 = String::new();
    let mut marker2 = String::new();
    let mut board = Board::new();
    
    println!("Let's play tic-tac-toe!");

    println!("Please enter a character that the first player will use to mark their place on the board.");

    io::stdin().read_line(&mut marker1).expect("Invalid character");
    let player1 = Player::new(marker1.trim(), 1);

    println!("Please enter a character that the second player will use to mark their place on the board.");

    io::stdin().read_line(&mut marker2).expect("Invalid character");
    let player2 = Player::new(marker2.trim(), 2);

    let mut turn: u32 = 1;
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

        if check_for_winner(&board.tiles, &player1, &player2) == true {
            if turn % 2 == 0 {
                println!("{} wins!", first);
            } else {
                println!("{} wins!", second);
            }

            board.show_board();

            println!("Do you want to play again? y/n");

            let mut play_again = String::new();

            io::stdin().read_line(&mut play_again).expect("Invalid move");

            if play_again.trim() == "y" {
                board = Board::new();
                turn = 1;
            } else {
                break;
            }
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