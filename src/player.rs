use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;

#[derive(PartialEq, Clone)]
enum Player {
    X,
    O,
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Self::X => write!(f, "X"),
            Self::O => write!(f, "O"),
        }
    }
}
