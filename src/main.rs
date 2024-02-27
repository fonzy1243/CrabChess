pub mod board;
pub mod pieces;

use crate::board::*;
use iced::executor;
use iced::{Application, Command, Element, Settings, Theme};

pub fn main() -> iced::Result {
    CrabChess::run(Settings::default())
}

// State - application state (TODO)
struct CrabChess {
    board: Board,
}
// Messages - user interactions or important events (TODO)
pub enum Message {}

impl Application for CrabChess {
    type Executor = executor::Default;

    type Message = ();

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: ()) -> (CrabChess, Command<Self::Message>) {
        (CrabChess, Command::none())
    }

    fn title(&self) -> String {
        String::from("CrabChess")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        "Google en passant".into()
    }

    fn theme(&self) -> Theme {
        Theme::CatppuccinLatte
    }
}
