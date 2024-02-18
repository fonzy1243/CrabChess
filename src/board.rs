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
