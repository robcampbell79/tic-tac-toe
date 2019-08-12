use std::io;

struct Player {
    marker: String,
    turn: u32,
}

impl Player {
    fn new(new_marker: &str, new_turn: u32) -> Player {
        Player {marker: new_marker.to_string(), turn: new_turn}
    }

    fn check_for_winner(arr: &[String; 3]) -> bool {
        let mut winner: bool = false;
        if arr[0] == "x" && arr[1] == "x" && arr[2] == "x" {
            winner = true;
        } 
        else if arr[0] == "o" && arr[1] == "o" && arr[2] == "o" {
            winner = true;
        }
        else {
            winner = false;
        }

        winner
    }
}

struct Board {
    a: [String;3],
    b: [String;3],
    c: [String;3],
}

impl Board {
    fn new() -> Board {
        Board {
            a: ["#".to_string(), "#".to_string(), "#".to_string()],
            b: ["#".to_string(), "#".to_string(), "#".to_string()],
            c: ["#".to_string(), "#".to_string(), "#".to_string()],
        }
    }

    fn show_board(&self) {
        println!("{:?}", self.a);
        println!("{:?}", self.b);
        println!("{:?}", self.c);
    }

}

fn player_move(board: &mut Board, make_move: String, player: &Player) -> bool {
    let mut row_name = make_move[0..1].trim();
    let mut row_num = make_move[1..2].trim();
    let mut marker = &player.marker;
    let mut turn = false;

    if row_name == "a" {
        match row_num {
            "1" => if board.a[0] == "x" || board.a[0] == "o" {
                        println!("Try again, not empty");
                        turn = false;
                    } else {
                        board.a[0] = marker.to_string();
                        turn = true;
                    },
            "2" => if board.a[1] == "x" || board.a[1] == "o" {
                        println!("Try again, not empty");
                        turn = false;
                    } else {
                        board.a[1] = marker.to_string();
                        turn = true;
                    },
            "3" => if board.a[2] == "x" || board.a[2] == "o" {
                        println!("Try again, not empty");
                        turn = false;
                    } else {
                        board.a[2] = marker.to_string();
                        turn = true;
                    },
            (_) => println!("Invalid number."),
        }
    }
    else if row_name == "b" {
        match row_num {
            "1" => if board.b[0] == "x" || board.b[0] == "o" {
                        println!("Try again, not empty");
                        turn = false;
                    } else {
                        board.b[0] = marker.to_string();
                        turn = true;
                    },
            "2" => if board.b[1] == "x" || board.b[1] == "o" {
                        println!("Try again, not empty");
                        turn = false;
                    } else {
                        board.b[1] = marker.to_string();
                        turn = true;
                    },
            "3" => if board.b[2] == "x" || board.b[2] == "o" {
                        println!("Try again, not empty");
                        turn = false;
                    } else {
                        board.b[2] = marker.to_string();
                        turn = true;
                    },
            (_) => println!("Invalid number."),
        }
    }
    else {
        match row_num {
            "1" => if board.c[0] == "x" || board.c[0] == "o" {
                        println!("Try again, not empty");
                        turn = false;
                    } else {
                        board.c[0] = marker.to_string();
                        turn = true;
                    },
            "2" => if board.c[1] == "x" || board.c[1] == "o" {
                        println!("Try again, not empty");
                        turn = false;
                    } else {
                        board.c[1] = marker.to_string();
                        turn = true;
                    },
            "3" => if board.c[2] == "x" || board.c[2] == "o" {
                        println!("Try again, not empty");
                        turn = false;
                    } else {
                        board.c[2] = marker.to_string();
                        turn = true;
                    },
            (_) => println!("Invalid number."),
        }
    }

    turn
}

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