use crate::board::{Cup, CupPos, MancalaBoard};
use crate::{GameState, MancalaError, Player, Result};
use compare::{natural, Compare};
use std::cmp::Ordering;
use std::fmt;

const BOARD_SIZE: usize = 12;
const STARTING_COUNT: usize = 4;

#[derive(Clone, PartialEq)]
pub struct Ayoayo {
    pub(crate) board: MancalaBoard,
    pub state: GameState,
}

impl Default for Ayoayo {
    fn default() -> Self {
        Self::new()
    }
}

impl Ayoayo {
    pub fn new() -> Ayoayo {
        let board: Vec<Cup> = [Player::Player1, Player::Player2]
            .iter()
            .flat_map(|player| {
                (0..(BOARD_SIZE / 2)).map(move |i| Cup {
                    owner: *player,
                    seeds: STARTING_COUNT,
                    pos: i,
                })
            })
            .collect();

        Ayoayo {
            board: MancalaBoard::new(board, &[Player::Player1, Player::Player2]),
            state: GameState::InProgress(Player::Player1),
        }
    }

    fn sow_filter(check_cup: &CupPos, player: Player, start_cup: usize) -> bool {
        let cup = check_cup;
        !(cup.owner == player && cup.pos == start_cup)
    }

    fn sow(&mut self, player: Player, cup: usize) -> Result<()> {
        self.board.pickup(
            CupPos {
                owner: player,
                pos: cup,
            },
            player,
        );
        let mut last = self.board.sow(
            player,
            CupPos {
                owner: player,
                pos: cup,
            },
            Ayoayo::sow_filter,
        )?;
        while last.seeds > 1 {
            let cup_pos = CupPos::from(&last);
            self.board.pickup(cup_pos, player);
            last = self.board.sow(player, cup_pos, Ayoayo::sow_filter)?;
        }
        if last.owner == player {
            self.board.pickup(
                CupPos {
                    owner: player.next_player(),
                    pos: last.pos,
                },
                player,
            );
            self.board.bank(player);
        };
        Ok(())
    }

    fn win_state(&mut self, player: Player) {
        for cup in self.board.cups.clone().iter() {
            let cup_pos = CupPos::from(cup);
            self.board.pickup(cup_pos, player);
            self.board.bank(player);
        }
        self.state = match natural().compare(
            self.board.bank.get(&player).expect("Player should exist"),
            self.board
                .bank
                .get(&player.next_player())
                .expect("Player should exist"),
        ) {
            Ordering::Less => GameState::Won(player.next_player()),
            Ordering::Greater => GameState::Won(player),
            Ordering::Equal => GameState::Draw,
        }
    }

    // Game over Check (No valid moves)
    // Feeding check (Must give other player seeds if other player has no seeds _at start of play_)
    pub fn play(&mut self, cup: usize) -> Result<()> {
        let player = match self.state {
            GameState::InProgress(p) => p,
            _ => return Ok(()),
        };

        if cup >= (BOARD_SIZE / 2) {
            return Err(MancalaError::NoSuchCup);
        }

        match self.board.get_cup(CupPos {
            owner: player,
            pos: cup,
        }) {
            Some(Cup { seeds: 0, .. }) => return Err(MancalaError::NoSeedsToSow),
            None => return Err(MancalaError::NoSuchCup),
            _ => (),
        };

        let must_feed = self.board.starving(player.next_player());
        let mut test_board = self.clone();
        test_board.sow(player, cup)?;
        if must_feed && test_board.board.starving(player.next_player()) {
            //If we must feed and didn't, we need to make sure that we couldn't have
            let any_not_starving = (0..(BOARD_SIZE / 2))
                .filter(|i| *i != cup)
                .map(|cup| {
                    let mut b = self.clone();
                    if b.sow(player, cup).is_err() {
                        return self.clone().board;
                    }
                    b.board
                })
                .map(|board| board.starving(player.next_player()))
                .any(|s| !s);
            if any_not_starving {
                return Err(MancalaError::MustFeedError);
            }
        };

        self.board = test_board.board.clone();
        if test_board.board.starving(player.next_player()) {
            self.win_state(player);
        } else {
            self.state = GameState::InProgress(player.next_player())
        };

        Ok(())
    }
}

impl fmt::Display for Ayoayo {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.board)
    }
}

impl fmt::Debug for Ayoayo {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play() -> Result<()> {
        let mut board = Ayoayo::new();
        board.play(3)?;
        assert_eq!("0 - ①|⑥|⑥|②|⑦|①\n⑥|①|⑥|⑥|⑥|⓪ - 0", format!("{}", board));
        assert_eq!(board.state, GameState::InProgress(Player::Player2));
        println!("-- play2 --");
        board.play(0)?;
        assert_eq!("0 - ②|⑨|②|⑤|⑩|①\n②|④|⓪|①|⑨|③ - 0", format!("{}", board));
        println!("-- play3 --");
        board.play(0)?;
        assert_eq!("3 - ①|⑩|⓪|⑥|⑰|⓪\n⓪|⓪|①|②|⑩|④ - 0", format!("{}", board));
        println!("-- play4 --");
        board.play(4)?;
        assert_eq!("3 - ②|⑰|①|⑦|⓪|①\n①|①|⓪|③|①|⑤ - 12", format!("{}", board));
        println!("-- play5 --");
        board.play(2)?;
        assert_eq!("3 - ⑥|①|⓪|①|⑤|③\n①|⑤|②|⑦|②|⓪ - 12", format!("{}", board));
        println!("-- play6 --");
        board.play(1)?;
        assert_eq!("3 - ⓪|⓪|①|②|⑥|④\n②|①|③|⑧|③|① - 14", format!("{}", board));
        println!("-- play7 --");
        board.play(3)?;
        assert_eq!("8 - ⓪|②|①|①|⓪|①\n④|③|⓪|⑩|①|③ - 14", format!("{}", board));
        println!("-- play8 --");
        board.play(3)?;
        assert_eq!("8 - ①|⑤|④|⓪|①|④\n①|⓪|③|⓪|⑤|② - 14", format!("{}", board));
        println!("-- play9 --");
        board.play(1)?;
        assert_eq!("8 - ⓪|①|⓪|②|③|⑥\n①|②|①|①|⑥|③ - 14", format!("{}", board));
        println!("-- play10 --");
        board.play(1)?;
        assert_eq!("8 - ①|②|①|⓪|④|⓪\n⓪|①|⓪|①|⑧|① - 21", format!("{}", board));
        println!("-- play11 --");
        board.play(0)?;
        assert_eq!("10 - ①|①|②|①|⓪|①\n①|⓪|①|⓪|⑨|⓪ - 21", format!("{}", board));
        println!("-- play12 --");
        board.play(4)?;
        assert_eq!("10 - ②|⓪|③|②|①|②\n②|①|①|⓪|⓪|① - 23", format!("{}", board));
        println!("-- play13 --");
        board.play(2)?;
        assert_eq!("10 - ②|⓪|⓪|③|②|⓪\n③|②|⓪|①|①|① - 23", format!("{}", board));
        println!("-- play14 --");
        board.play(3)?;
        assert_eq!("10 - ⓪|①|①|⓪|⓪|①\n④|⓪|①|①|①|② - 26", format!("{}", board));
        println!("-- play15 --");
        board.play(5)?;
        assert_eq!("12 - ①|②|⓪|①|①|⓪\n⓪|①|②|②|⓪|⓪ - 26", format!("{}", board));
        println!("-- play16 --");
        board.play(2)?;
        assert_eq!("12 - ①|②|⓪|①|⓪|⓪\n⓪|①|⓪|③|①|⓪ - 27", format!("{}", board));
        println!("-- play17 --");
        board.play(3)?;
        assert_eq!("13 - ①|②|⓪|⓪|①|⓪\n⓪|①|⓪|③|⓪|⓪ - 27", format!("{}", board));
        println!("-- play18 --");
        board.play(3)?;
        assert_eq!("13 - ⓪|③|①|⓪|①|⓪\n⓪|①|⓪|⓪|①|① - 27", format!("{}", board));
        println!("-- play19 --");
        board.play(4)?;
        assert_eq!("14 - ⓪|③|①|⓪|⓪|①\n⓪|①|⓪|⓪|①|⓪ - 27", format!("{}", board));
        println!("-- play20 --");
        board.play(1)?;
        assert_eq!("14 - ⓪|③|⓪|⓪|⓪|①\n⓪|⓪|①|⓪|①|⓪ - 28", format!("{}", board));
        println!("-- play21 --");
        board.play(5)?;
        assert_eq!("14 - ⓪|③|⓪|⓪|⓪|⓪\n①|⓪|①|⓪|①|⓪ - 28", format!("{}", board));
        println!("-- play22 --");
        board.play(0)?;
        assert_eq!("14 - ⓪|⓪|⓪|⓪|⓪|⓪\n⓪|⓪|⓪|⓪|⓪|⓪ - 34", format!("{}", board));
        assert_eq!(board.state, GameState::Won(Player::Player2));
        Ok(())
    }

    #[test]
    fn must_feed_test() {
        let mut game = Ayoayo {
            board: MancalaBoard::new(
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
            ),
            state: GameState::InProgress(Player::Player1),
        };
        assert_eq!(Err(MancalaError::MustFeedError), game.play(0));
    }

    #[test]
    fn no_seeds_test() {
        let mut game = Ayoayo {
            board: MancalaBoard::new(
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
            ),
            state: GameState::InProgress(Player::Player1),
        };
        assert_eq!(Err(MancalaError::NoSeedsToSow), game.play(1));
    }
}
