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
}
