use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsoleColour {
    Black,
    Blue,
    Brown,
    Cyan,
    DarkerGreen,
    Green,
    Grey,
    LightGrey,
    Orange,
    Pink,
    Purple,
    Red,
    White,
    Yellow,
    Caret
}

impl Display for ConsoleColour {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ConsoleColour::Black => "black",
            ConsoleColour::Blue => "blue",
            ConsoleColour::Brown => "brown",
            ConsoleColour::Cyan => "cyan",
            ConsoleColour::DarkerGreen => "darkergreen",
            ConsoleColour::Green => "green",
            ConsoleColour::Grey => "grey",
            ConsoleColour::LightGrey => "lightgrey",
            ConsoleColour::Orange => "orange",
            ConsoleColour::Pink => "pink",
            ConsoleColour::Purple => "purple",
            ConsoleColour::Red => "red",
            ConsoleColour::White => "white",
            ConsoleColour::Yellow => "yellow",
            ConsoleColour::Caret => "caret"
        })
    }
}