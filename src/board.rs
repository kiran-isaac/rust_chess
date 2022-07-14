use crate::setup::{Board};

pub fn at(board: &Board, pos: (u8, u8)) -> Option<char> {
    board[pos.1 as usize][pos.0 as usize]
}

pub fn print_board(board : &Board) {
    println!("   A  B  C  D  E  F  G  H");
    for i in 0..8 {
        print!("{} ", 8 - i);
        for j in 0..8 {
            let bg_color = if (i + j) % 2 == 0 {
                crossterm::style::Color::DarkGrey
            } else {
                crossterm::style::Color::Grey
            };

            let fg_color = crossterm::style::Color::Yellow;
            match &board[7 - i][j] {
                Some(piece) => {match crossterm::execute!(
                    std::io::stdout(),
                    crossterm::style::SetBackgroundColor(bg_color),
                    crossterm::style::SetForegroundColor(fg_color),
                    crossterm::style::Print(" "),
                    crossterm::style::Print(piece),
                    crossterm::style::Print(" "),
                    crossterm::style::ResetColor
                ) {
                    Ok(_) => {},
                    Err(e) => {
                        println!("{}", e);
                    }
                }; ()},
                None => {match crossterm::execute!(
                    std::io::stdout(),
                    crossterm::style::SetBackgroundColor(bg_color),
                    crossterm::style::SetForegroundColor(fg_color),
                    crossterm::style::Print("   "),
                    crossterm::style::ResetColor
                ) {
                    Ok(_) => {},
                    Err(e) => {
                        println!("{}", e);
                    }
                }; ()},
            }
        }
        println!();
    }
}