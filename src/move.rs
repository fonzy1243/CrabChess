use crate::board::*;

pub struct Move {
    start: i32,
    target: i32,
}

pub fn gen_moves(board: Board) -> std::vec::Vec<Move> {
    let mut moves = vec![];

    for (start, piece) in board.squares.iter().enumerate() {
        if board.is_color(*piece) {
            if is_sliding(*piece) {
                gen_sliding_moves(&board, &mut moves, start, *piece);
            }
        }
    }

    moves
}

fn gen_sliding_moves(board: &Board, moves: &mut Vec<Move>, start: usize, piece: i32) {
    todo!()
}
