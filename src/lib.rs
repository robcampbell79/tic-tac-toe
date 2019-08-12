pub struct Player {
    pub marker: String,
    pub turn: u32,
}

impl Player {
    pub fn new(new_marker: &str, new_turn: u32) -> Player {
        Player {marker: new_marker.to_string(), turn: new_turn}
    }

    pub fn check_for_winner(arr: &[String; 3]) -> bool {
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

pub struct Board {
    pub a: [String;3],
    pub b: [String;3],
    pub c: [String;3],
}

impl Board {
    pub fn new() -> Board {
        Board {
            a: ["#".to_string(), "#".to_string(), "#".to_string()],
            b: ["#".to_string(), "#".to_string(), "#".to_string()],
            c: ["#".to_string(), "#".to_string(), "#".to_string()],
        }
    }

    pub fn show_board(&self) {
        println!("{:?}", self.a);
        println!("{:?}", self.b);
        println!("{:?}", self.c);
    }

}

pub fn player_move(board: &mut Board, make_move: String, player: &Player) -> bool {
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