use std::ops::BitOr;

enum Color {
    White = 8,
    Black = 16,
}

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
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.square.len() {
            if i % 8 == 0 {
                writeln!(f)?;
            }

            match self.square[i] {
                x if x == Piece::Empty.into() => write!(f, " "),
                x if x == Color::White | Piece::Pawn => write!(f, "♙"),
                x if x == Color::Black | Piece::Pawn => write!(f, "♟︎"),
                x if x == Color::White | Piece::Knight => write!(f, "♘"),
                x if x == Color::Black | Piece::Knight => write!(f, "♞"),
                x if x == Color::White | Piece::Bishop => write!(f, "♗"),
                x if x == Color::Black | Piece::Bishop => write!(f, "♝"),
                x if x == Color::White | Piece::Rook => write!(f, "♖"),
                x if x == Color::Black | Piece::Rook => write!(f, "♜"),
                x if x == Color::White | Piece::Queen => write!(f, "♕"),
                x if x == Color::Black | Piece::Queen => write!(f, "♛"),
                x if x == Color::White | Piece::King => write!(f, "♔"),
                x if x == Color::Black | Piece::King => write!(f, "♚"),
                _ => panic!("Board has invalid piece."),
            }?;
        }
        Ok(())
    }
}
