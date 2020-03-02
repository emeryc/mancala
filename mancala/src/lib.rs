use std::error;
use std::fmt;
pub mod ayoayo;
pub mod board;

#[derive(Copy, Debug, PartialEq, Clone, Hash, Eq)]
pub enum Player {
    Player1,
    Player2,
}

impl Player {
    pub(crate) fn next_player(&self) -> Player {
        match self {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::Player1 => write!(f, "Player 1"),
            Player::Player2 => write!(f, "Player 2"),
        }
    }
}

type Result<T> = std::result::Result<T, MancalaError>;

#[derive(PartialEq, Debug, Clone)]
pub enum MancalaError {
    MustFeedError,
    NoSeedsToSow,
    NoSuchCup,
}

impl fmt::Display for MancalaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MancalaError::MustFeedError => {
                write!(f, "Your play must result in seeds for your opponent")
            }
            MancalaError::NoSeedsToSow => write!(f, "You must choose a cup with seeds"),
            MancalaError::NoSuchCup => write!(f, "That cup doesn't exist"),
        }
    }
}

impl error::Error for MancalaError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress(Player),
    Won(Player),
    Draw,
}
