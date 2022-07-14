pub type Square = Option<Piece>;

pub type Board = [[Square; 8]; 8];

pub type Piece = char;

pub fn setup_board(board : &mut Board) {
    board[7][0] = Some('r');
    board[7][1] = Some('n');
    board[7][2] = Some('b');
    board[7][3] = Some('q');
    board[7][4] = Some('k');
    board[7][5] = Some('b');
    board[7][6] = Some('n');
    board[7][7] = Some('r');

    board[0][0] = Some('R');
    board[0][1] = Some('N');
    board[0][2] = Some('B');
    board[0][3] = Some('Q');
    board[0][4] = Some('K');
    board[0][5] = Some('B');
    board[0][6] = Some('N');
    board[0][7] = Some('R');

    for i in 0..8 {
        board[1][i] = Some('P');
        board[6][i] = Some('p');
    }
}