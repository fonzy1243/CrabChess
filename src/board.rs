enum Piece {
    White,
    Black,
    WhitePawn,
    BlackPawn,
    WhiteKnight,
    BlackKnight,
    WhiteBishop,
    BlackBishop,
    WhiteRook,
    BlackRook,
    WhiteQueen,
    BlackQueen,
    WhiteKing,
    BlackKing,
    BlackSlave,
}

struct Board {
    pieces: [u64; 14],
    empty: u64,
    occupied: u64,
}

impl Board {
    fn get_piece_set(self, pt: usize) -> u64 {
        self.pieces[pt]
    }

    fn get_white_pawns(self) -> u64 {
        self.pieces[Piece::WhitePawn as usize]
    }

    fn get_black_pawns(self) -> u64 {
        self.pieces[Piece::BlackPawn as usize]
    }

    fn get_white_knights(self) -> u64 {
        self.pieces[Piece::WhiteKnight as usize]
    }

    fn get_black_knights(self) -> u64 {
        self.pieces[Piece::BlackKnight as usize]
    }

    fn get_white_bishops(self) -> u64 {
        self.pieces[Piece::WhiteBishop as usize]
    }

    fn get_black_bishops(self) -> u64 {
        self.pieces[Piece::BlackBishop as usize]
    }

    fn get_white_rooks(self) -> u64 {
        self.pieces[Piece::WhiteRook as usize]
    }

    fn get_black_rooks(self) -> u64 {
        self.pieces[Piece::BlackRook as usize]
    }

    fn get_white_queens(self) -> u64 {
        self.pieces[Piece::WhiteQueen as usize]
    }

    fn get_black_queens(self) -> u64 {
        self.pieces[Piece::BlackQueen as usize]
    }

    fn get_white_king(self) -> u64 {
        self.pieces[Piece::WhiteKing as usize]
    }

    fn get_black_king(self) -> u64 {
        self.pieces[Piece::BlackKing as usize]
    }

    fn get_pawns(self, ct: usize) -> u64 {
        self.pieces[Piece::WhitePawn as usize + ct]
    }

    fn get_knights(self, ct: usize) -> u64 {
        self.pieces[Piece::WhiteKnight as usize + ct]
    }

    fn get_bishops(self, ct: usize) -> u64 {
        self.pieces[Piece::WhiteBishop as usize + ct]
    }

    fn get_rooks(self, ct: usize) -> u64 {
        self.pieces[Piece::WhiteRook as usize + ct]
    }

    fn get_queens(self, ct: usize) -> u64 {
        self.pieces[Piece::WhiteQueen as usize + ct]
    }

    fn get_king(self, ct: usize) -> u64 {
        self.pieces[Piece::WhiteKing as usize + ct]
    }

    fn get_whites(self) -> u64 {
        self.pieces[Piece::White as usize]
    }

    fn get_blacks(self) -> u64 {
        self.pieces[Piece::Black as usize]
    }
}

// --------------------------------------------------
// Setwise Operations
// --------------------------------------------------

const NOT_A_FILE: u64 = 0xfefefefefefefefe;
const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;

fn sout_one(b: u64) -> u64 {
    b >> 8
}

fn nort_one(b: u64) -> u64 {
    b << 8
}

// Post-shift masks

fn east_one(b: u64) -> u64 {
    (b << 1) & NOT_A_FILE
}

fn no_ea_one(b: u64) -> u64 {
    (b << 9) & NOT_A_FILE
}

fn so_ea_one(b: u64) -> u64 {
    (b >> 7) & NOT_A_FILE
}

fn west_one(b: u64) -> u64 {
    (b >> 1) & NOT_H_FILE
}

fn so_we_one(b: u64) -> u64 {
    (b >> 9) & NOT_H_FILE
}

fn no_we_one(b: u64) -> u64 {
    (b << 7) & NOT_H_FILE
}

// Rotate

fn rotate_left(x: u64, s: i32) -> u64 {
    (x << s) | (x >> (64 - s))
}

fn rotate_right(x: u64, s: i32) -> u64 {
    (x >> s) | (x << (64 - s))
}

// Generalized Shift

/// Shifts left for positive amounts, but right for negative amounts.
fn gen_shift(x: u64, s: i32) -> u64 {
    if s > 0 {
        x << s
    } else {
        x >> -s
    }
}

// --------------------------------------------------
// Population Count
// --------------------------------------------------

// From Donald Knuth, The Art of Computer Programming
const K1: u64 = 0x5555555555555555; /* -1/3 */
const K2: u64 = 0x3333333333333333; /* -1/5 */
const K4: u64 = 0x0f0f0f0f0f0f0f0f; /* -1/17 */
const KF: u64 = 0x0101010101010101; /* -1/255 */

fn pop_count(mut x: u64) -> i32 {
    x = x - ((x >> 1) & K1); // put count of each 2 bits into those 2 bits
    x = (x & K2) + ((x >> 2) & K2); // put count of each 4 bits into those 4 bits
    x = (x + (x >> 4)) & K4; // put count of each 8 bits into those 8 bits
    x = (x * KF) >> 56; // returns 8 MSBs of x + (x << 8) + (x << 16) + ...
    x as i32
}

fn hamming_distance(a: u64, b: u64) -> i32 {
    pop_count(a ^ b)
}

// --------------------------------------------------
// Pawn Pushes
// --------------------------------------------------

fn w_single_push_targets(w_pawns: u64, empty: u64) -> u64 {
    nort_one(w_pawns) & empty
}

fn w_dbl_push_targets(w_pawns: u64, empty: u64) -> u64 {
    let rank4: u64 = 0x00000000FF000000;
    let single_pushes = w_single_push_targets(w_pawns, empty);
    nort_one(single_pushes) & empty & rank4
}

fn b_single_push_targets(b_pawns: u64, empty: u64) -> u64 {
    sout_one(b_pawns) & empty
}

fn b_dbl_push_targets(b_pawns: u64, empty: u64) -> u64 {
    let rank5: u64 = 0x00000000FF000000;
    let single_pushes = b_single_push_targets(b_pawns, empty);
    sout_one(single_pushes) & empty & rank5
}

fn w_pawns_able_to_push(w_pawns: u64, empty: u64) -> u64 {
    sout_one(empty) & w_pawns
}

fn w_pawns_able_to_dbl_push(w_pawns: u64, empty: u64) -> u64 {
    let rank4: u64 = 0x00000000FF000000;
    let empty_rank3 = sout_one(empty & rank4) & empty;
    w_pawns_able_to_push(w_pawns, empty_rank3)
}

fn single_push_targets(pawns: u64, empty: u64, color: i32) -> u64 {
    rotate_left(pawns, 8 - (color << 4)) & empty
}
