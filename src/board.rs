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

impl Default for Board {
    fn default() -> Self {
        Self {
            square: [
                Color::White | Piece::Rook,
                Color::White | Piece::Knight,
                Color::White | Piece::Bishop,
                Color::White | Piece::Queen,
                Color::White | Piece::King,
                Color::White | Piece::Bishop,
                Color::White | Piece::Knight,
                Color::White | Piece::Rook,
                Color::White | Piece::Pawn,
                Color::White | Piece::Pawn,
                Color::White | Piece::Pawn,
                Color::White | Piece::Pawn,
                Color::White | Piece::Pawn,
                Color::White | Piece::Pawn,
                Color::White | Piece::Pawn,
                Color::White | Piece::Pawn,
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Piece::Empty.into(),
                Color::Black | Piece::Pawn,
                Color::Black | Piece::Pawn,
                Color::Black | Piece::Pawn,
                Color::Black | Piece::Pawn,
                Color::Black | Piece::Pawn,
                Color::Black | Piece::Pawn,
                Color::Black | Piece::Pawn,
                Color::Black | Piece::Pawn,
                Color::Black | Piece::Rook,
                Color::Black | Piece::Knight,
                Color::Black | Piece::Bishop,
                Color::Black | Piece::Queen,
                Color::Black | Piece::King,
                Color::Black | Piece::Bishop,
                Color::Black | Piece::Knight,
                Color::Black | Piece::Rook,
            ],
        }
    }
}
