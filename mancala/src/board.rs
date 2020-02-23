use crate::Player;
use itertools::{zip, Itertools};
use std::{cell::RefCell, collections::HashMap, fmt};

#[derive(Copy, Debug, PartialEq, Clone)]
pub struct Cup {
    pub owner: Player,
    pub seeds: usize,
    pub pos: usize,
}

pub struct CupItterator<'a>(&'a Vec<RefCell<Cup>>, usize);

impl<'a> Iterator for CupItterator<'a> {
    type Item = &'a RefCell<Cup>;

    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        let next_pos = (self.1 + 1) % self.0.len();
        self.1 = next_pos;

        self.0.get(next_pos)
    }
}

#[derive(Clone, PartialEq)]
pub struct MancalaBoard {
    pub(crate) cups: Vec<RefCell<Cup>>,
    pub(crate) bank: HashMap<Player, usize>,
    pub(crate) in_hand: HashMap<Player, usize>,
}

impl MancalaBoard {
    // Does the board need the concept of the bank and the hand?
    pub(crate) fn new(cups: &[Cup], players: &[Player]) -> MancalaBoard {
        MancalaBoard {
            cups: cups.iter().map(|c| RefCell::new(*c)).collect(),
            bank: players.iter().map(|player| (*player, 0)).collect(),
            in_hand: players.iter().map(|player| (*player, 0)).collect(),
        }
    }

    pub(crate) fn get_cup(&self, player: Player, cup: usize) -> Cup {
        self.iter_at_cup(player, cup)
            .next()
            .expect("Why would we ever not have a cup?")
            .borrow()
            .clone()
    }

    pub(crate) fn starving(&self, player: Player) -> bool {
        self.cups
            .iter()
            .filter(|cup| cup.borrow().owner == player)
            .all(|cup| cup.borrow().seeds == 0)
    }

    pub fn pickup(&self, cup: Cup, player: Player) -> MancalaBoard {
        let new_board = self.clone();
        let seeds: Option<usize> = zip(new_board.iter_at_cup(cup.owner, cup.pos), 0..1)
            .map(|(cup, _)| cup.replace_with(|cup| Cup { seeds: 0, ..*cup }).seeds)
            .last();
        let mut new_board = new_board.clone();
        match seeds {
            Some(seeds) => {
                new_board.in_hand.insert(player, seeds);
                ()
            }
            None => (),
        };
        new_board
    }

    // Move this into board, take a filter argument to validate that this is a cell you should be able to sow into
    pub fn sow<F>(&self, player: Player, cup: Cup, filter: F) -> (MancalaBoard, Cup)
    where
        F: Fn(&RefCell<Cup>, Player, usize) -> bool,
    {
        let new_board = self.clone();
        let final_cup: Option<&RefCell<Cup>> = zip(
            new_board
                .iter_at_cup(cup.owner, cup.pos)
                .filter(|cup_ref| filter(cup_ref, cup.owner, cup.pos)),
            0..*self.in_hand.get(&player).expect("Yikes"),
        )
        .map(|(c, _)| c)
        .map(|cup_ref| {
            cup_ref.replace_with(|old| Cup {
                seeds: old.seeds + 1,
                ..*old
            });
            cup_ref
        })
        .last();

        let mut new_board = new_board.clone();
        let bad_cup = new_board.cups[0].borrow().clone();
        new_board.in_hand.insert(player, 0);

        return (
            new_board,
            match final_cup {
                Some(cup) => cup.borrow().clone(),
                None => bad_cup, //Is this right?
            },
        );
    }

    pub fn bank(&self, player: Player) -> MancalaBoard {
        let mut new_board = self.clone();
        new_board.bank.entry(player).and_modify(|cur_bank| {
            *cur_bank += match self.in_hand.get(&player) {
                Some(e) => e,
                None => &0,
            }
        });
        new_board
    }

    fn iter(&self) -> CupItterator {
        CupItterator(&self.cups, 0)
    }

    fn iter_at_cup(&self, player: Player, c_pos: usize) -> impl Iterator<Item = &RefCell<Cup>> {
        let mut iter = self.iter().peekable();
        loop {
            let cup = iter.peek().expect("Infinite Itterator Failed?").borrow();
            if cup.owner == player && cup.pos == c_pos {
                return iter;
            }
            iter.next();
        }
    }
}

const VALUES: [char; 21] = [
    '\u{24EA}', '\u{2460}', '\u{2461}', '\u{2462}', '\u{2463}', '\u{2464}', '\u{2465}', '\u{2466}',
    '\u{2467}', '\u{2468}', '\u{2469}', '\u{2470}', '\u{2471}', '\u{2472}', '\u{2473}', '\u{2474}',
    '\u{2475}', '\u{2476}', '\u{2477}', '\u{2478}', '\u{2479}',
];

impl fmt::Display for MancalaBoard {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (top, bottom) = self.cups.split_at(self.cups.len() / 2);

        let top: String = top.iter().map(|x| VALUES[x.borrow().seeds]).join("|");
        let bottom: String = bottom.iter().map(|x| VALUES[x.borrow().seeds]).join("|");
        write!(
            fmt,
            "{} - {}\n{} - {}",
            self.bank.get(&Player::Player1).expect("Ugh"),
            top,
            bottom,
            self.bank.get(&Player::Player2).expect("Ugh")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_board(size: usize, count: usize) -> MancalaBoard {
        let mut board = Vec::new();
        for owner in [Player::Player1, Player::Player2].iter() {
            for i in 0..(size / 2) {
                board.push(Cup {
                    owner: owner.clone(),
                    seeds: count,
                    pos: i,
                })
            }
        }
        return MancalaBoard::new(&board[..], &[Player::Player1, Player::Player2]);
    }

    #[test]
    fn print_board() {
        println!("{}", build_board(2, 2));

        assert_eq!(format!("{}", build_board(2, 2)), "0 - ②\n② - 0");
    }

    #[test]
    fn remove() {
        let start_board = build_board(12, 4);
        let new_board = start_board.pickup(
            Cup {
                owner: Player::Player2,
                pos: 3,
                seeds: 0,
            },
            Player::Player1,
        );
        assert_eq!("0 - ④|④|④|④|④|④\n④|④|④|⓪|④|④ - 0", format!("{}", new_board));
        assert_eq!(4, *new_board.in_hand.get(&Player::Player1).expect("Yikes!"));
    }

    #[test]
    fn collect() {
        let start_board = build_board(12, 4);
        let new_board = start_board
            .pickup(
                Cup {
                    owner: Player::Player1,
                    pos: 2,
                    seeds: 0,
                },
                Player::Player2,
            )
            .bank(Player::Player2);
        assert_eq!("0 - ④|④|⓪|④|④|④\n④|④|④|④|④|④ - 4", format!("{}", new_board));
    }

    #[test]
    fn sow_1() {
        let start_board = build_board(4, 2);
        let (new_board, cup) = start_board
            .pickup(
                Cup {
                    pos: 0,
                    owner: Player::Player1,
                    seeds: 0,
                },
                Player::Player1,
            )
            .sow(
                Player::Player1,
                Cup {
                    pos: 0,
                    owner: Player::Player1,
                    seeds: 0,
                },
                |c, p, _| {
                    let cup = c.borrow();
                    cup.owner != p
                },
            );
        assert_eq!(
            0,
            *new_board.in_hand.get(&Player::Player1).expect("Debugging")
        );
        assert_eq!("0 - ⓪|②\n③|③ - 0", format!("{}", new_board));
        assert_eq!(
            "Cup { owner: Player2, seeds: 3, pos: 1 }",
            format!("{:?}", cup)
        )
    }
}
