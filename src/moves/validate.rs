use std::cmp::max;
use crate::{apply_move, Board, coords_to_square, Move};
use crate::board::at;

fn validate_pawn_move(board: &Board, m: &Move) -> (bool, bool) {
    let (from_x, from_y) = m.from;
    let (to_x, to_y) = m.to;

    let piece_at = at(board, m.to);

    let is_white = at(board, m.from).unwrap_or(' ') == 'P';
    let direction = if is_white { 1 } else { -1 };

    let max_dist = (if is_white && from_y == 1 || !is_white && from_y == 6 { 2 } else { 1 });

    let delta_y = to_y as i32 - from_y as i32;
    let delta_x = to_x as i32 - from_x as i32;

    if delta_y.abs() > max_dist || delta_y.abs() == 0 || delta_y / delta_y.abs() != direction {
        return (false, false);
    }

    let mut cap = false;

    if delta_x.abs() == 1 && delta_y.abs() == 1 {
        if piece_at.is_none() {
            return (false, false);
        }

        if piece_at.unwrap().is_ascii_uppercase() == is_white {
            return (false, false);
        }
        cap = true;
    } else {
        if piece_at.is_some() {
            return (false, false);
        }
    }

    (true, cap)
}

fn validate_in_direction(board: &Board, initial: (u8, u8), magnitude: i32, direction: (i32, i32)) -> bool {
    for i in 1..magnitude {
        let x = initial.0 as i32 + i * direction.0;
        let y = initial.1 as i32 + i * direction.1;

        // println!("{}", coords_to_square((x as u8, y as u8)));

        if at(board, (x as u8, y as u8)).is_some() {
            return false;
        }
    }

    true
}

fn validate_rook_move(board: &Board, m: &Move) -> bool {
    let (from_x, from_y) = m.from;
    let (to_x, to_y) = m.to;


    let is_white = at(board, m.from).unwrap_or(' ').is_ascii_uppercase();

    let piece_at = at(board, m.to);

    if piece_at.is_some() && piece_at.unwrap().is_ascii_uppercase() == is_white {
        return false;
    }

    let delta_y = to_y as i32 - from_y as i32;
    let delta_x = to_x as i32 - from_x as i32;

    let direction: (i32, i32) = (
        if delta_x != 0 { delta_x / delta_x.abs()} else {0},
        if delta_y != 0 { delta_y / delta_y.abs()} else {0}
    );

    if !((direction.0 == 0) ^ (direction.1 == 0)) {
        return false;
    }

    let magnitude = max(delta_x.abs(), delta_y.abs());

    return validate_in_direction(board,m.from, magnitude, direction);
}

fn validate_bishop_move(board: &Board, m: &Move) -> bool {
    let (from_x, from_y) = m.from;
    let (to_x, to_y) = m.to;

    let is_white = at(board, m.from).unwrap_or(' ').is_ascii_uppercase();

    let piece_at = at(board, m.to);

    if piece_at.is_some() && piece_at.unwrap().is_ascii_uppercase() == is_white {
        return false;
    }

    let delta_y = to_y as i32 - from_y as i32;
    let delta_x = to_x as i32 - from_x as i32;

    let direction: (i32, i32) = (
        if delta_x != 0 { delta_x / delta_x.abs()} else {0},
        if delta_y != 0 { delta_y / delta_y.abs()} else {0}
    );

    if direction.0 == 0 || direction.1 == 0 {
        return false;
    }

    if delta_x.abs() != delta_y.abs() {
        return false;
    }

    let magnitude = max(delta_x.abs(), delta_y.abs());

    return validate_in_direction(board,m.from, magnitude, direction);
}

fn validate_queen_move(board: &Board, m: &Move) -> bool {
    return validate_rook_move(board, m) || validate_bishop_move(board, m);
}

fn validate_king_move(board: &Board, m: &Move) -> bool {
    let (from_x, from_y) = m.from;
    let (to_x, to_y) = m.to;

    let is_white = at(board, m.from).unwrap_or(' ') == 'P';

    let piece_at = at(board, m.to);

    if piece_at.is_some() && piece_at.unwrap().is_ascii_uppercase() == is_white {
        return false;
    }

    let delta_y = to_y as i32 - from_y as i32;
    let delta_x = to_x as i32 - from_x as i32;

    if delta_x.abs() > 1 || delta_y.abs() > 1 {
        return false;
    }

    if delta_x.abs() == 1 && delta_y.abs() == 1 {
        if piece_at.is_some() && piece_at.unwrap().is_ascii_uppercase() == is_white {
            return false;
        }
    }

    true
}

fn validate_knight_move(board: &Board, m: &Move) -> bool {
    let (from_x, from_y) = m.from;
    let (to_x, to_y) = m.to;

    let is_white = at(board, m.from).unwrap_or(' ').is_ascii_uppercase();

    let piece_at = at(board, m.to);

    if piece_at.is_some() && piece_at.unwrap().is_ascii_uppercase() == is_white {
        return false;
    }

    let delta_y = to_y as i32 - from_y as i32;
    let delta_x = to_x as i32 - from_x as i32;

    return delta_x.abs() == 2 && delta_y.abs() == 1 || delta_x.abs() == 1 && delta_y.abs() == 2;
}

fn find_king(board: &Board, is_white: bool) -> (u8, u8) {
    for (i, row) in board.iter().enumerate() {
        for (j, square) in row.iter().enumerate() {
            if square.is_none() {
                continue;
            }
            if square.unwrap().is_ascii_uppercase() == is_white && square.unwrap().to_ascii_lowercase() == 'k' {
                return (j as u8, i as u8);
            }
        }
    }

    (0, 0)
}

fn check_check(board: &Board, is_white: bool) -> bool {
    let king_square = find_king(board, is_white);

    for i in 0..8 {
        for j in 0..8 {
            let square = (i as u8, j as u8);
            let piece = at(board, square);

            if piece.is_none() {
                continue;
            }

            if piece.unwrap().is_ascii_uppercase() == is_white {
                continue;
            }

            let valid = validate_move(board, &Move {from: square, to: king_square, piece: piece.unwrap()});

            if valid.0 && valid.1 {
                return true;
            }
        }
    }

    false
}

pub fn validate_move(board: &Board, m: &Move) -> (bool, bool) {
    if m.from.0 > 7 || m.from.1 > 7 || m.to.0 > 7 || m.to.1 > 7
        || Some(m.piece) != board[m.from.1 as usize][m.from.0 as usize]
        || board[m.from.1 as usize][m.from.0 as usize] == None {
        return (false, false);
    }

    let result = match m.piece.to_ascii_lowercase() {
        'p' => validate_pawn_move(board, m),
        'r' => (validate_rook_move(board, m), true),
        'b' => (validate_bishop_move(board, m), true),
        'q' => (validate_queen_move(board, m), true),
        'k' => (validate_king_move(board, m), true),
        'n' => (validate_knight_move(board, m), true),
        _ => unimplemented!(),
    };

    if !(result.0) {
        return (false, false);
    }

    let mut ghost = board.clone();

    let _ = apply_move(&mut ghost, &m);

    let after = check_check(&ghost, m.piece.is_ascii_uppercase());

    if after {
        return (false, false);
    }

    (true, result.1)
}