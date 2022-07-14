mod board;
mod setup;
mod moves;

use std::io;
use moves::*;

use crate::setup::{Board, setup_board};
use crate::board::{at, print_board};

fn test_move(board: &Board, m: &Move) {
    println!("{}: {}", m, validate_move(&board, m).0);
}

fn main() {
    let mut board: Board = [[None; 8]; 8];

    setup_board(&mut board);

    // println!();
    //
    // board[2][5] = Some('p');
    //
    // print_board(&board);
    //
    // let from = square_to_coords("C1");
    // let to = square_to_coords("D2");
    // let m = &Move {
    //     from,
    //     to,
    //     piece: at(&board,from).unwrap(),
    // };
    // test_move(&board, m);
    //
    // let _ = apply_move(&mut board, m);

    let mut is_white = true;

    loop {
        print_board(&board);

        println!("\n{} please enter your move (EG A1:A2): ", if is_white { "White" } else { "Black" });

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => continue
        };

        input = input.trim().to_string();
        let split = input.split(':').collect::<Vec<&str>>();
        let from = square_to_coords(split[0]);
        let to = square_to_coords(split[1]);
        let piece_at = at(&board, from);
        if piece_at.is_none() || piece_at.unwrap_or(' ').is_ascii_uppercase() != is_white {
            println!("Cannot move piece at {}", split[0]);
            continue;
        }

        let m = &Move {
            from,
            to,
            piece: piece_at.unwrap(),
        };

        if !validate_move(&board, m).0 {
            println!("Invalid move");
            continue;
        }

        let capture = apply_move(&mut board, m);
        if capture.is_some() {
            println!("Captured {}", capture.unwrap());
        }
        is_white = !is_white;
    }
}
