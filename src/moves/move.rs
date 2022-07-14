use std::fmt;
use crate::setup::{Board};

pub struct Move {
    pub from: (u8, u8),
    pub to: (u8, u8),
    pub piece: char,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} from {} to {})", if self.piece.is_ascii_uppercase() { "White" } else { "Black" }, char_to_name(self.piece), coords_to_square(self.from), coords_to_square(self.to))
    }
}

fn char_to_name(piece: char) -> String {
    match piece.to_ascii_lowercase() {
        'p' => String::from("Pawn"),
        'r' => String::from("Rook"),
        'n' => String::from("Knight"),
        'b' => String::from("Bishop"),
        'q' => String::from("Queen"),
        'k' => String::from("King"),
        _ => String::from(""),
    }
}

pub fn coords_to_square(coords: (u8, u8)) -> String {
    let mut s = String::new();
    s.push((coords.0 + 'A' as u8) as char);
    s.push((coords.1 + '1' as u8) as char);
    s
}

pub fn square_to_coords(square: &str) -> (u8, u8) {
    let mut s = square.chars();
    let x = s.next().unwrap().to_ascii_uppercase() as u8 - 'A' as u8;
    let y = s.next().unwrap() as u8 - '1' as u8;
    (x, y)
}

pub fn apply_move(board: &mut Board, move_: &Move) -> Option<char> {
    let (from_x, from_y) = move_.from;
    let (to_x, to_y) = move_.to;

    let piece = move_.piece;
    let capture = board[to_y as usize][to_x as usize];
    board[to_y as usize][to_x as usize] = Some(piece);
    board[from_y as usize][from_x as usize] = None;

    capture
}