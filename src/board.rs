pub enum Piece {
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

pub struct Board {
    pieces: [u64; 14],
    empty: u64,
    occupied: u64,
}

impl Board {
    pub(crate) fn get_piece_set(self, pt: usize) -> u64 {
        self.pieces[pt]
    }

    pub(crate) fn get_white_pawns(self) -> u64 {
        self.pieces[Piece::WhitePawn as usize]
    }

    pub(crate) fn get_black_pawns(self) -> u64 {
        self.pieces[Piece::BlackPawn as usize]
    }

    pub(crate) fn get_white_knights(self) -> u64 {
        self.pieces[Piece::WhiteKnight as usize]
    }

    pub(crate) fn get_black_knights(self) -> u64 {
        self.pieces[Piece::BlackKnight as usize]
    }

    pub(crate) fn get_white_bishops(self) -> u64 {
        self.pieces[Piece::WhiteBishop as usize]
    }

    pub(crate) fn get_black_bishops(self) -> u64 {
        self.pieces[Piece::BlackBishop as usize]
    }

    pub(crate) fn get_white_rooks(self) -> u64 {
        self.pieces[Piece::WhiteRook as usize]
    }

    pub(crate) fn get_black_rooks(self) -> u64 {
        self.pieces[Piece::BlackRook as usize]
    }

    pub(crate) fn get_white_queens(self) -> u64 {
        self.pieces[Piece::WhiteQueen as usize]
    }

    pub(crate) fn get_black_queens(self) -> u64 {
        self.pieces[Piece::BlackQueen as usize]
    }

    pub(crate) fn get_white_king(self) -> u64 {
        self.pieces[Piece::WhiteKing as usize]
    }

    pub(crate) fn get_black_king(self) -> u64 {
        self.pieces[Piece::BlackKing as usize]
    }

    pub(crate) fn get_pawns(self, ct: usize) -> u64 {
        self.pieces[Piece::WhitePawn as usize + ct]
    }

    pub(crate) fn get_knights(self, ct: usize) -> u64 {
        self.pieces[Piece::WhiteKnight as usize + ct]
    }

    pub(crate) fn get_bishops(self, ct: usize) -> u64 {
        self.pieces[Piece::WhiteBishop as usize + ct]
    }

    pub(crate) fn get_rooks(self, ct: usize) -> u64 {
        self.pieces[Piece::WhiteRook as usize + ct]
    }

    pub(crate) fn get_queens(self, ct: usize) -> u64 {
        self.pieces[Piece::WhiteQueen as usize + ct]
    }

    pub(crate) fn get_king(self, ct: usize) -> u64 {
        self.pieces[Piece::WhiteKing as usize + ct]
    }

    pub(crate) fn get_whites(self) -> u64 {
        self.pieces[Piece::White as usize]
    }

    pub(crate) fn get_blacks(self) -> u64 {
        self.pieces[Piece::Black as usize]
    }
}

// --------------------------------------------------
// Setwise Operations
// --------------------------------------------------

pub const NOT_A_FILE: u64 = 0xfefefefefefefefe;
pub const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;

pub(crate) fn sout_one(b: u64) -> u64 {
    b >> 8
}

pub(crate) fn nort_one(b: u64) -> u64 {
    b << 8
}

// Post-shift masks

pub(crate) fn east_one(b: u64) -> u64 {
    (b << 1) & NOT_A_FILE
}

pub(crate) fn no_ea_one(b: u64) -> u64 {
    (b << 9) & NOT_A_FILE
}

pub(crate) fn so_ea_one(b: u64) -> u64 {
    (b >> 7) & NOT_A_FILE
}

pub(crate) fn west_one(b: u64) -> u64 {
    (b >> 1) & NOT_H_FILE
}

pub(crate) fn so_we_one(b: u64) -> u64 {
    (b >> 9) & NOT_H_FILE
}

pub(crate) fn no_we_one(b: u64) -> u64 {
    (b << 7) & NOT_H_FILE
}

// Rotate

pub(crate) fn rotate_left(x: u64, s: i32) -> u64 {
    (x << s) | (x >> (64 - s))
}

pub(crate) fn rotate_right(x: u64, s: i32) -> u64 {
    (x >> s) | (x << (64 - s))
}

// Generalized Shift

/// Shifts left for positive amounts, but right for negative amounts.
pub(crate) fn gen_shift(x: u64, s: i32) -> u64 {
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

pub(crate) fn pop_count(mut x: u64) -> i32 {
    x = x - ((x >> 1) & K1); // put count of each 2 bits into those 2 bits
    x = (x & K2) + ((x >> 2) & K2); // put count of each 4 bits into those 4 bits
    x = (x + (x >> 4)) & K4; // put count of each 8 bits into those 8 bits
    x = (x * KF) >> 56; // returns 8 MSBs of x + (x << 8) + (x << 16) + ...
    x as i32
}

pub(crate) fn hamming_distance(a: u64, b: u64) -> i32 {
    pop_count(a ^ b)
}
