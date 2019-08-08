use std::io;

fn check_for_winner(arr: &[String; 3]) -> u32 {
    let mut winner: u32 = 0;
    if arr[0] == "x" && arr[1] == "x" && arr[2] == "x" {
        winner = 1;
    } 
    else if arr[0] == "o" && arr[1] == "o" && arr[2] == "o" {
        winner = 1;
    }
    else {
        winner = 0;
    }

    winner
}

fn main() {
    
    println!("Let's play tic-tac-toe!");

    let mut rowOne: [String;3] = ["#".to_string(), "#".to_string(), "#".to_string()];
    let mut rowTwo: [String;3] = ["#".to_string(), "#".to_string(), "#".to_string()];
    let mut rowThree: [String;3] = ["#".to_string(), "#".to_string(), "#".to_string()];

    let mut turn: u32 = 1;
    let mut marker = String::from("x");

    loop {

        if turn % 2 == 0 {
            marker = "o".to_string();
        } else {
            marker = "x".to_string();
        }

        if check_for_winner(&rowOne) == 1 || check_for_winner(&rowTwo) == 1 || check_for_winner(&rowThree) == 1 {
            println!("You win!");
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