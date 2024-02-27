use crate::r#move::*;
use core::{fmt, ops::BitOr, panic};

pub enum Color {
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

#[derive(PartialEq, Eq)]
pub enum Piece {
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

pub fn is_sliding(piece: i32) -> bool {
    matches!(piece & 7, x if x == BISHOP as i32 || x == ROOK as i32 || x == QUEEN as i32)
}

pub fn is_type(piece: i32, piece_type: Piece) -> bool {
    let piece_type_num = piece_type as i32;
    piece & 7 == piece_type_num
}

fn piece_to_char(piece: i32) -> char {
    match piece {
        x if x == EMPTY as i32 => ' ',
        x if x == WHITE | PAWN => '♙',
        x if x == WHITE | KNIGHT => '♘',
        x if x == WHITE | BISHOP => '♗',
        x if x == WHITE | ROOK => '♖',
        x if x == WHITE | QUEEN => '♕',
        x if x == WHITE | KING => '♔',
        x if x == BLACK | PAWN => '♟',
        x if x == BLACK | KNIGHT => '♞',
        x if x == BLACK | BISHOP => '♝',
        x if x == BLACK | ROOK => '♜',
        x if x == BLACK | QUEEN => '♛',
        x if x == BLACK | KING => '♚',
        _ => panic!(),
    }
}

#[derive(Debug)]
pub struct Board {
    pub squares: [i32; 64],
    pub color_to_move: i32,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "    a   b   c   d   e   f   g   h")?;
        writeln!(f, "  ╔═══════════════════════════════╗")?;

        for rank in (0..8).rev() {
            write!(f, "{} ║", rank + 1)?;

            for file in 0..8 {
                let index = rank * 8 + file;
                let piece = self.squares[index];
                write!(f, " {} ", piece_to_char(piece))?;
                if file < 7 {
                    write!(f, "│")?;
                }
            }

            writeln!(f, "║ {}", rank + 1)?;

            if rank > 0 {
                writeln!(f, "  ║───┼───┼───┼───┼───┼───┼───┼───║")?;
            }
        }

        writeln!(f, "  ╚═══════════════════════════════╝")?;
        writeln!(f, "    a   b   c   d   e   f   g   h")?;
        Ok(())
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            squares: [
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
            color_to_move: WHITE as i32,
        }
    }
}

impl Board {
    pub fn is_color(&self, piece: i32) -> bool {
        if self.color_to_move == WHITE as i32 {
            piece & 8 == 8
        } else {
            piece & 16 == 16
        }
    }
}
