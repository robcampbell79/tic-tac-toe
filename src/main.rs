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

fn main() {

    let player1 = Player::new("x", 1);
    let player2 = Player::new("o", 2);
    
    println!("Let's play tic-tac-toe!");

    let mut rowOne: [String;3] = ["#".to_string(), "#".to_string(), "#".to_string()];
    let mut rowTwo: [String;3] = ["#".to_string(), "#".to_string(), "#".to_string()];
    let mut rowThree: [String;3] = ["#".to_string(), "#".to_string(), "#".to_string()];

    let mut turn: u32 = 1;
    let mut marker: String;

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

        if turn % 2 == 0 {
            marker = player2.marker.to_string();
        } else {
            marker = player1.marker.to_string();
        }

        if Player::check_for_winner(&rowOne) == true || Player::check_for_winner(&rowTwo) == true || Player::check_for_winner(&rowThree) == true {
            if turn % 2 == 0 {
                println!("{} wins!", first);
            } else {
                println!("{} wins!", second);
            }
            println!("{:?}", rowOne);
            println!("{:?}", rowTwo);
            println!("{:?}", rowThree);
            break;
        }

        println!("Where do you want to move?");

        println!("{:?}", rowOne);
        println!("{:?}", rowTwo);
        println!("{:?}", rowThree);

        let mut make_move = String::new();

        io::stdin().read_line(&mut make_move).expect("Invalid move");

        if make_move.trim() == "a1" {
            if rowOne[0] == "x" || rowOne[0] == "o" {
                println!("Try again, not empty");
                turn -= 1;
            } else {
                rowOne[0] = marker;
            }
        }
        else if make_move.trim() == "a2" {
            if rowOne[1] == "x" || rowOne[1] == "o" {
                println!("Try again, not empty");
                turn -= 1;
            } else {
                rowOne[1] = marker;
            }
        }
        else if make_move.trim() == "a3" {
            if rowOne[2] == "x" || rowOne[2] == "o" {
                println!("Try again, not empty");
                turn -= 1;
            } else {
                rowOne[2] = marker;
            }
        }
        else if make_move.trim() == "b1" {
            if rowTwo[0] == "x" || rowTwo[0] == "o" {
                println!("Try again, not empty");
                turn -= 1;
            } else {
                rowTwo[0] = marker;
            }
        }
        else if make_move.trim() == "b2" {
            if rowTwo[1] == "x" || rowTwo[1] == "o" {
                println!("Try again, not empty");
                turn -= 1;
            } else {
                rowTwo[1] = marker;
            }
        }
        else if make_move.trim() == "b3" {
            if rowTwo[2] == "x" || rowTwo[2] == "o" {
                println!("Try again, not empty");
                turn -= 1;
            } else {
                rowTwo[2] = marker;
            }
        }
        else if make_move.trim() == "c1" {
            if rowThree[0] == "x" || rowThree[0] == "o" {
                println!("Try again, not empty");
                turn -= 1;
            } else {
                rowThree[0] = marker;
            }
        }
        else if make_move.trim() == "c2" {
            if rowThree[1] == "x" || rowThree[1] == "o" {
                println!("Try again, not empty");
                turn -= 1;
            } else {
                rowThree[1] = marker;
            }
        }
        else if make_move.trim() == "c3" {
            if rowThree[2] == "x" || rowThree[2] == "o" {
                println!("Try again, not empty");
                turn -= 1;
            } else {
                rowThree[2] = marker;
            }
        }
        else {
            println!("Under Construction");
            break;
        }

        turn += 1;
    }
}