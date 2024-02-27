pub mod board;
pub mod r#move;
pub mod pieces;

use crate::board::*;

fn main() {
    println!("{}", Board::default());
}
