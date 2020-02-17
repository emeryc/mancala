use itertools::Itertools;
use std::fmt;

const BOARD_SIZE: usize = 12;
const STARTING_COUNT: usize = 4;

const VALUES: [char; 21] = [
    '\u{24EA}', '\u{2460}', '\u{2461}', '\u{2462}', '\u{2463}', '\u{2464}', '\u{2465}', '\u{2466}',
    '\u{2467}', '\u{2468}', '\u{2469}', '\u{2470}', '\u{2471}', '\u{2472}', '\u{2473}', '\u{2474}',
    '\u{2475}', '\u{2476}', '\u{2477}', '\u{2478}', '\u{2479}',
];

pub enum Player {
    Player1,
    Player2,
}

#[derive(Debug, PartialEq)]
pub struct Ayoayo {
    board: [usize; BOARD_SIZE],
    p1_bank: usize,
    p2_bank: usize,
}
impl Ayoayo {
    pub fn new() -> Ayoayo {
        return Ayoayo {
            board: [STARTING_COUNT; BOARD_SIZE],
            p1_bank: 0,
            p2_bank: 0,
        };
    }

    pub fn sow(&self, start_cup: usize, seeds: usize) -> (Ayoayo, usize) {
        let mut new_board = self.board.clone();
        let mut seeds_left = seeds;
        let mut idx = 0;
        let mut cup = 0;
        while seeds_left > 0 {
            println!("cup - {}: idx - {}: seeds_left - {}", cup, idx, seeds_left);
            cup = (idx + start_cup) % BOARD_SIZE;
            idx += 1;
            if cup == start_cup {
                continue;
            }
            new_board[cup] += 1;
            seeds_left -= 1;
        }
        println!("cup - {}: idx - {}: seeds_left - {}", cup, idx, seeds_left);
        (
            Ayoayo {
                board: new_board,
                ..*self
            },
            cup,
        )
    }

    pub fn remove(&self, cup: usize) -> (Ayoayo, usize) {
        let mut new_board = self.board.clone();
        new_board[cup] = 0;
        (
            Ayoayo {
                board: new_board,
                ..*self
            },
            self.board[cup],
        )
    }

    pub fn collect(&self, cup: usize, player: Player) -> Ayoayo {
        let (board, collected_seeds) = self.remove(cup);

        match player {
            Player::Player1 => Ayoayo {
                p1_bank: self.p1_bank + collected_seeds,
                ..board
            },
            Player::Player2 => Ayoayo {
                p2_bank: self.p2_bank + collected_seeds,
                ..board
            },
        }
    }

    pub fn play(&self, cup: usize, player: Player) -> Ayoayo {
        let (b, seeds) = self.remove(cup);
        let (mut b, mut last) = b.sow(cup, seeds);
        while b.board[last] != 1 {
            let (tmp, seeds) = b.remove(last);
            let (tb, tlast) = tmp.sow(last, seeds);
            b = tb;
            last = tlast;
        }
        b.collect(last, player)
    }
}

impl fmt::Display for Ayoayo {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (top, bottom) = self.board.split_at(6);

        let top: String = top.iter().map(|x| VALUES[*x]).join("|");
        let bottom: String = bottom.iter().map(|x| VALUES[*x]).join("|");
        write!(
            fmt,
            "{} - {}\n{} - {}",
            self.p1_bank, top, self.p2_bank, bottom
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starting_board() {
        assert_eq!(Ayoayo::new().board[0], 4);
    }

    #[test]
    fn print_board() {
        println!("{}", Ayoayo::new());

        assert_eq!(
            format!("{}", Ayoayo::new()),
            "0 - ④|④|④|④|④|④\n0 - ④|④|④|④|④|④"
        );
    }

    #[test]
    fn sow_a() {
        let start_board = Ayoayo::new();
        let new_board = start_board.sow(3, 13);
        assert_eq!(
            Ayoayo {
                board: [5, 5, 5, 4, 6, 6, 5, 5, 5, 5, 5, 5],
                p1_bank: 0,
                p2_bank: 0
            },
            new_board.0
        );
        assert_eq!(!new_board.1, 5)
    }

    #[test]
    fn sow_b() {
        let start_board = Ayoayo {
            board: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            p1_bank: 0,
            p2_bank: 0,
        };
        let new_board = start_board.sow(3, 3);
        assert_eq!(
            Ayoayo {
                board: [0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0],
                p1_bank: 0,
                p2_bank: 0
            },
            new_board.0
        );
        assert_eq!(new_board.1, 6)
    }

    #[test]
    fn remove() {
        let start_board = Ayoayo::new();
        let (new_board, seeds) = start_board.remove(3);
        assert_eq!(
            Ayoayo {
                board: [4, 4, 4, 0, 4, 4, 4, 4, 4, 4, 4, 4],
                p1_bank: 0,
                p2_bank: 0
            },
            new_board
        );
        assert_eq!(4, seeds);
    }

    #[test]
    fn collect() {
        let start_board = Ayoayo::new();
        let new_board = start_board.collect(3, Player::Player1);
        assert_eq!(
            Ayoayo {
                board: [4, 4, 4, 0, 4, 4, 4, 4, 4, 4, 4, 4],
                p1_bank: 4,
                p2_bank: 0
            },
            new_board
        );
    }
}
