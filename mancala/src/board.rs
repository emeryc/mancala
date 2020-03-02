use crate::{MancalaError, Player, Result};
use itertools::Itertools;
use std::{collections::HashMap, fmt};

#[derive(Debug, PartialEq, Clone)]
pub struct Cup {
    pub owner: Player,
    pub seeds: usize,
    pub pos: usize,
}

#[derive(Copy, PartialEq, Debug, Clone)]
pub struct CupPos {
    pub owner: Player,
    pub pos: usize,
}

impl PartialEq<Cup> for CupPos {
    fn eq(&self, rhs: &Cup) -> bool {
        self.owner == rhs.owner && self.pos == rhs.pos
    }
}
impl PartialEq<CupPos> for Cup {
    fn eq(&self, rhs: &CupPos) -> bool {
        self.owner == rhs.owner && self.pos == rhs.pos
    }
}
impl From<&Cup> for CupPos {
    fn from(cup: &Cup) -> Self {
        CupPos {
            owner: cup.owner,
            pos: cup.pos,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct MancalaBoard {
    pub(crate) cups: Vec<Cup>,
    pub(crate) bank: HashMap<Player, usize>,
    pub(crate) in_hand: HashMap<Player, usize>,
}

// struct InfIterMut<'a>(&'a Vec<Cup>, usize);
// impl<'a> Iterator for InfIterMut<'a> {
//     type Item = &'a mut Cup;

//     fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
//         self.0.get_mut(self.1)
//     }
// }

impl MancalaBoard {
    // Does the board need the concept of the bank and the hand?
    pub(crate) fn new(cups: Vec<Cup>, players: &[Player]) -> MancalaBoard {
        MancalaBoard {
            cups: cups.into_iter().map(|c| c).collect(),
            bank: players.iter().map(|player| (*player, 0)).collect(),
            in_hand: players.iter().map(|player| (*player, 0)).collect(),
        }
    }

    pub(crate) fn get_cup(&self, cup: CupPos) -> Option<&Cup> {
        for c in self.cups.iter() {
            if *c == cup {
                return Some(c);
            }
        }
        return None;
    }

    fn get_mut_cup(&mut self, cup: CupPos) -> Option<&mut Cup> {
        for c in self.cups.iter_mut() {
            if *c == cup {
                return Some(c);
            }
        }
        return None;
    }

    pub(crate) fn starving(&self, player: Player) -> bool {
        self.cups
            .iter()
            .filter(|cup| cup.owner == player)
            .all(|cup| cup.seeds == 0)
    }

    pub(crate) fn pickup(&mut self, cup: CupPos, player: Player) -> Option<()> {
        let mut cup = self.get_mut_cup(cup);
        // let mut in_hand = self.in_hand.get_mut(&player);
        cup.take()
            .map(|cup| {
                let seeds = cup.seeds;
                cup.seeds = 0;
                seeds
            })
            .map(|seeds| {
                self.in_hand.insert(player, seeds);
            })
    }

    // Move this into board, take a filter argument to validate that this is a cell you should be able to sow into
    pub(crate) fn sow<F>(&mut self, player: Player, cup: CupPos, filter: F) -> Result<Cup>
    where
        F: Fn(&CupPos, Player, usize) -> bool,
    {
        let seeds = *self
            .in_hand
            .get_mut(&player)
            .expect("Player doesn't exist!");
        let final_cup = self
            .cups
            .clone()
            .iter()
            .map(|c| CupPos::from(c))
            .cycle()
            .skip_while(|c| *c != cup)
            .filter(|c| filter(c, cup.owner, cup.pos))
            .take(seeds)
            .map(|cup_pos| {
                let mut cup = self.get_mut_cup(cup_pos).expect("");
                cup.seeds += 1;
                cup.clone()
            })
            .last();
        self.in_hand.insert(player, 0);

        return final_cup.ok_or(MancalaError::NoSeedsToSow);
    }

    pub(crate) fn bank(&mut self, player: Player) {
        let value = self.in_hand.insert(player, 0).unwrap_or_else(|| 0);
        self.bank
            .entry(player)
            .and_modify(|cur_bank| *cur_bank += value);
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

        let top: String = top.iter().map(|x| VALUES[x.seeds]).join("|");
        let bottom: String = bottom.iter().map(|x| VALUES[x.seeds]).join("|");
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
        return MancalaBoard::new(board, &[Player::Player1, Player::Player2]);
    }

    #[test]
    fn print_board() {
        println!("{}", build_board(2, 2));

        assert_eq!(format!("{}", build_board(2, 2)), "0 - ②\n② - 0");
    }

    #[test]
    fn remove() {
        let mut board = build_board(12, 4);
        board.pickup(
            CupPos {
                owner: Player::Player2,
                pos: 3,
            },
            Player::Player1,
        );
        assert_eq!("0 - ④|④|④|④|④|④\n④|④|④|⓪|④|④ - 0", format!("{}", board));
        assert_eq!(4, *board.in_hand.get(&Player::Player1).expect("Yikes!"));
    }

    #[test]
    fn collect() {
        let mut board = build_board(12, 4);
        board.pickup(
            CupPos {
                owner: Player::Player1,
                pos: 2,
            },
            Player::Player2,
        );
        board.bank(Player::Player2);
        assert_eq!("0 - ④|④|⓪|④|④|④\n④|④|④|④|④|④ - 4", format!("{}", board));
    }

    #[test]
    fn sow_1() {
        let mut board = build_board(4, 2);
        board.pickup(
            CupPos {
                pos: 0,
                owner: Player::Player1,
            },
            Player::Player1,
        );
        let cup = board.sow(
            Player::Player1,
            CupPos {
                pos: 0,
                owner: Player::Player1,
            },
            |cup, p, _| cup.owner != p,
        );
        assert_eq!(0, *board.in_hand.get(&Player::Player1).expect("Debugging"));
        assert_eq!("0 - ⓪|②\n③|③ - 0", format!("{}", board));
        assert_eq!(
            "Cup { owner: Player2, seeds: 3, pos: 1 }",
            format!("{:?}", cup.unwrap())
        )
    }

    #[test]
    fn sow_2() {
        let mut board = MancalaBoard::new(
            vec![
                Cup {
                    seeds: 1,
                    owner: Player::Player1,
                    pos: 0,
                },
                Cup {
                    seeds: 0,
                    owner: Player::Player1,
                    pos: 1,
                },
                Cup {
                    seeds: 1,
                    owner: Player::Player1,
                    pos: 2,
                },
                Cup {
                    seeds: 0,
                    owner: Player::Player2,
                    pos: 0,
                },
                Cup {
                    seeds: 0,
                    owner: Player::Player2,
                    pos: 1,
                },
                Cup {
                    seeds: 0,
                    owner: Player::Player2,
                    pos: 2,
                },
            ],
            &[Player::Player1, Player::Player2],
        );
        board.pickup(
            CupPos {
                pos: 0,
                owner: Player::Player1,
            },
            Player::Player1,
        );
        let cup = board.sow(
            Player::Player1,
            CupPos {
                pos: 0,
                owner: Player::Player1,
            },
            |cup, p, pos| !(cup.owner == p && cup.pos == pos),
        );
        assert_eq!(0, *board.in_hand.get(&Player::Player1).expect("Debugging"));
        assert_eq!("0 - ⓪|①|①\n⓪|⓪|⓪ - 0", format!("{}", board));
        assert_eq!(
            "Cup { owner: Player1, seeds: 1, pos: 1 }",
            format!("{:?}", cup.unwrap())
        )
    }
}
