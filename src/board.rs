use std::ops::BitOr;

enum Color {
    White = 8,
    Black = 16,
}

pub const WHITE: Color = Color::White;
pub const BLACK: Color = Color::Black;

impl From<Color> for i32 {
    fn from(value: Color) -> Self {
        value as Self
    }
}

enum Piece {
    Empty = 0,
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,
    Queen = 5,
    King = 6,
}

pub const EMPTY: Piece = Piece::Empty;
pub const PAWN: Piece = Piece::Pawn;
pub const KNIGHT: Piece = Piece::Knight;
pub const BISHOP: Piece = Piece::Bishop;
pub const ROOK: Piece = Piece::Rook;
pub const QUEEN: Piece = Piece::Queen;
pub const KING: Piece = Piece::King;

impl BitOr<Piece> for Color {
    type Output = i32;

    fn bitor(self, rhs: Piece) -> Self::Output {
        self as i32 | rhs as i32
    }
}

impl From<Piece> for i32 {
    fn from(value: Piece) -> Self {
        value as Self
    }
}

#[derive(Debug)]
pub(crate) struct Board {
    square: [i32; 64],
    is_white_turn: bool,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            square: [
                WHITE | ROOK,
                WHITE | KNIGHT,
                WHITE | BISHOP,
                WHITE | QUEEN,
                WHITE | KING,
                WHITE | BISHOP,
                WHITE | KNIGHT,
                WHITE | ROOK,
                WHITE | PAWN,
                WHITE | PAWN,
                WHITE | PAWN,
                WHITE | PAWN,
                WHITE | PAWN,
                WHITE | PAWN,
                WHITE | PAWN,
                WHITE | PAWN,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                EMPTY as i32,
                BLACK | PAWN,
                BLACK | PAWN,
                BLACK | PAWN,
                BLACK | PAWN,
                BLACK | PAWN,
                BLACK | PAWN,
                BLACK | PAWN,
                BLACK | PAWN,
                BLACK | ROOK,
                BLACK | KNIGHT,
                BLACK | BISHOP,
                BLACK | QUEEN,
                BLACK | KING,
                BLACK | BISHOP,
                BLACK | KNIGHT,
                BLACK | ROOK,
            ],
            is_white_turn: true,
        }
    }
}

impl Board {}
