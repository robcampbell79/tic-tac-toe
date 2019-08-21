pub struct Player {
    pub marker: &'static str,
    pub turn: u32,
}

impl Player {
    pub fn new<'a>(new_marker: &'static str, new_turn: u32) -> Player {
        Player {marker: new_marker.to_string(), turn: new_turn}
    }

    // pub fn check_for_winner(arr1: &[String; 3], arr2: &[String; 3], arr3: &[String; 3]) -> bool {
    //     let mut winner: bool = false;

    //     for i in arr1.iter() {

    //     }
    //     if arr1[0] == "x" && arr1[1] == "x" && arr1[2] == "x" || arr1[0] == "o" && arr1[1] == "o" && arr1[2] == "o" {
    //         winner = true;
    //     } 
    //     else if arr2[0] == "x" && arr2[1] == "x" && arr2[2] == "x" || arr2[0] == "o" && arr2[1] == "o" && arr2[2] == "o" {
    //         winner = true;
    //     }
    //     else if arr3[0] == "x" && arr3[1] == "x" && arr3[2] == "x" || arr3[0] == "o" && arr3[1] == "o" && arr3[2] == "o" {
    //         winner = true;
    //     }
    //     else if arr1[0] == "x" && arr2[0] == "x" && arr3[0] == "x" || arr1[0] == "o" && arr2[0] == "o" && arr3[0] == "o" {
    //         winner = true;
    //     }
    //     else if arr1[1] == "x" && arr2[1] == "x" && arr3[1] == "x" || arr1[1] == "o" && arr2[1] == "o" && arr3[1] == "o" {
    //         winner = true;
    //     }
    //     else if arr1[2] == "x" && arr2[2] == "x" && arr3[2] == "x" || arr1[2] == "o" && arr2[2] == "o" && arr3[2] == "o" {
    //         winner = true;
    //     }
    //     else if arr1[0] == "x" && arr2[1] == "x" && arr3[2] == "x" || arr1[0] == "o" && arr2[1] == "o" && arr3[2] == "o" {
    //         winner = true;
    //     }
    //     else if arr1[2] == "x" && arr2[1] == "x" && arr3[0] == "x" || arr1[2] == "o" && arr2[1] == "o" && arr3[0] == "o" {
    //         winner = true;
    //     }
    //     else {
    //         winner = false;
    //     }

    //     winner
    // }
}

pub struct Board {
    pub tiles: [Box<[String]>; 3],
}

impl Board {
    pub fn new() -> Board {
        let tile = "*";
        Board {
            tiles: [
                Box::new(["*".to_string(), "*".to_string(), "*".to_string()]),
                Box::new(["*".to_string(), "*".to_string(), "*".to_string()]),
                Box::new(["*".to_string(), "*".to_string(), "*".to_string()])
            ],
        }
    }

    pub fn show_board(&self) {
        println!("{:?}, {:?}, {:?}", self.tiles[0][0], self.tiles[0][1], self.tiles[0][2]);
        println!("{:?}, {:?}, {:?}", self.tiles[1][0], self.tiles[1][1], self.tiles[1][2]);
        println!("{:?}, {:?}, {:?}", self.tiles[2][0], self.tiles[2][1], self.tiles[2][2]);
    }

}

pub fn player_move(board: &mut Board, make_move: String, player: &Player) -> bool {
    let mut row_name = make_move[0..1].trim();
    let mut row_num = make_move[1..2].trim();
    let mut marker = &player.marker;
    let mut turn = false;

    if row_name == "a" {
        match row_num {
            "1" => if board.tiles[0][0] == "x" || board.tiles[0][0] == "o" {
                        println!("Try again, not empty");
                        turn = false;
                    } else {
                        board.tiles[0][0] = marker.to_string();
                        turn = true;
                    },
            "2" => if board.tiles[0][1] == "x" || board.tiles[0][1] == "o" {
                        println!("Try again, not empty");
                        turn = false;
                    } else {
                        board.tiles[0][1] = marker.to_string();
                        turn = true;
                    },
            "3" => if board.tiles[0][2] == "x" || board.tiles[0][2] == "o" {
                        println!("Try again, not empty");
                        turn = false;
                    } else {
                        board.tiles[0][2] = marker.to_string();
                        turn = true;
                    },
            (_) => println!("Invalid number."),
        }
    }
    else if row_name == "b" {
        match row_num {
            "1" => if board.tiles[1][0] == "x" || board.tiles[1][0] == "o" {
                        println!("Try again, not empty");
                        turn = false;
                    } else {
                        board.tiles[1][0] = marker.to_string();
                        turn = true;
                    },
            "2" => if board.tiles[1][1] == "x" || board.tiles[1][1] == "o" {
                        println!("Try again, not empty");
                        turn = false;
                    } else {
                        board.tiles[1][1] = marker.to_string();
                        turn = true;
                    },
            "3" => if board.tiles[1][2] == "x" || board.tiles[1][2] == "o" {
                        println!("Try again, not empty");
                        turn = false;
                    } else {
                        board.tiles[1][2] = marker.to_string();
                        turn = true;
                    },
            (_) => println!("Invalid number."),
        }
    }
    else {
        match row_num {
            "1" => if board.tiles[2][0] == "x" || board.tiles[2][0] == "o" {
                        println!("Try again, not empty");
                        turn = false;
                    } else {
                        board.tiles[2][0] = marker.to_string();
                        turn = true;
                    },
            "2" => if board.tiles[2][1] == "x" || board.tiles[2][1] == "o" {
                        println!("Try again, not empty");
                        turn = false;
                    } else {
                        board.tiles[2][1] = marker.to_string();
                        turn = true;
                    },
            "3" => if board.tiles[2][2] == "x" || board.tiles[2][2] == "o" {
                        println!("Try again, not empty");
                        turn = false;
                    } else {
                        board.tiles[2][2] = marker.to_string();
                        turn = true;
                    },
            (_) => println!("Invalid number."),
        }
    }

    turn
}

pub fn check_for_winner(arr: &[[String; 3]; 3]) -> bool {
        let mut winner: bool = false;

        if arr[0][0] == "x" && arr[0][1] == "x" && arr[0][2] == "x" || arr[0][0] == "o" && arr[0][1] == "o" && arr[0][2] == "o" {
            winner = true;
        } 
        else if arr[1][0] == "x" && arr[1][1] == "x" && arr[1][2] == "x" || arr[1][0] == "o" && arr[1][1] == "o" && arr[1][2] == "o" {
            winner = true;
        }
        else if arr[2][0] == "x" && arr[2][1] == "x" && arr[2][2] == "x" || arr[2][0] == "o" && arr[2][1] == "o" && arr[2][2] == "o" {
            winner = true;
        }
        // else if arr1[0] == "x" && arr2[0] == "x" && arr3[0] == "x" || arr1[0] == "o" && arr2[0] == "o" && arr3[0] == "o" {
        //     winner = true;
        // }
        // else if arr1[1] == "x" && arr2[1] == "x" && arr3[1] == "x" || arr1[1] == "o" && arr2[1] == "o" && arr3[1] == "o" {
        //     winner = true;
        // }
        // else if arr1[2] == "x" && arr2[2] == "x" && arr3[2] == "x" || arr1[2] == "o" && arr2[2] == "o" && arr3[2] == "o" {
        //     winner = true;
        // }
        // else if arr1[0] == "x" && arr2[1] == "x" && arr3[2] == "x" || arr1[0] == "o" && arr2[1] == "o" && arr3[2] == "o" {
        //     winner = true;
        // }
        // else if arr1[2] == "x" && arr2[1] == "x" && arr3[0] == "x" || arr1[2] == "o" && arr2[1] == "o" && arr3[0] == "o" {
        //     winner = true;
        // }
        else {
            winner = false;
        }

        winner
    }