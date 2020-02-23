use crate::board::{Cup, MancalaBoard};
use crate::{GameState, MancalaError, Player, Result};
use compare::{natural, Compare};
use std::cmp::Ordering;
use std::{cell::RefCell, fmt};

const BOARD_SIZE: usize = 12;
const STARTING_COUNT: usize = 4;

#[derive(Clone, PartialEq)]
pub struct Ayoayo {
    pub(crate) board: MancalaBoard,
    pub(crate) state: GameState,
}
impl Ayoayo {
    pub fn new() -> Ayoayo {
        let board: Vec<Cup> = [Player::Player1, Player::Player2]
            .iter()
            .flat_map(|player| {
                (0..(BOARD_SIZE / 2)).map(move |i| Cup {
                    owner: player.clone(),
                    seeds: STARTING_COUNT,
                    pos: i,
                })
            })
            .collect();

        return Ayoayo {
            board: MancalaBoard::new(&board[..], &[Player::Player1, Player::Player2]),
            state: GameState::InProgress(Player::Player1),
        };
    }

    fn sow_filter(check_cup: &RefCell<Cup>, player: Player, start_cup: usize) -> bool {
        let cup = check_cup.borrow();
        !(cup.owner == player && cup.pos == start_cup)
    }

    fn sow(board: MancalaBoard, player: Player, cup: usize) -> MancalaBoard {
        let (mut b, mut last) = board
            .pickup(
                Cup {
                    owner: player,
                    pos: cup,
                    seeds: 0,
                },
                player,
            )
            .sow(
                player,
                Cup {
                    owner: player,
                    pos: cup,
                    seeds: 0,
                },
                Ayoayo::sow_filter,
            );
        while last.seeds > 1 {
            println!("{}", b);
            let (tb, tlast) = b.pickup(last, player).sow(player, last, Ayoayo::sow_filter);
            b = tb;
            last = tlast;
        }
        println!("{}", b);
        if last.owner == player {
            b.pickup(
                Cup {
                    owner: player.next_player(),
                    ..last
                },
                player,
            )
            .bank(player)
        } else {
            b
        }
    }

    fn win_state(board: MancalaBoard, player: Player) -> GameState {
        let mut board = board.clone();
        for cup in board.clone().cups {
            board = board.pickup(*cup.borrow(), player).bank(player);
        }
        match natural().compare(
            board.bank.get(&player).expect("Player should exist"),
            board
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
    pub fn play(&self, cup: usize) -> Result<Ayoayo> {
        let player = match self.state {
            GameState::InProgress(p) => p,
            _ => return Ok(self.clone()),
        };

        println!("Cup - {:?}", self.board.get_cup(player, cup));
        if self.board.get_cup(player, cup).seeds == 0 {
            return Err(MancalaError::NoSeedsToSow);
        }

        let must_feed = self.board.starving(player.next_player());

        let board_result = Ayoayo::sow(self.board.clone(), player, cup);
        if must_feed && board_result.starving(player.next_player()) {
            //If we must feed and didn't, we need to make sure that we couldn't have
            let any_not_starving = (0..(BOARD_SIZE / 2))
                .filter(|i| *i != cup)
                .map(|cup| Ayoayo::sow(self.board.clone(), player, cup))
                .map(|board| board.starving(player.next_player()))
                .any(|s| !s);
            if any_not_starving {
                return Err(MancalaError::MustFeedError);
            }
        };
        Ok(Ayoayo {
            board: board_result.clone(),
            state: if board_result.starving(player.next_player()) {
                Ayoayo::win_state(board_result, player)
            } else {
                GameState::InProgress(player.next_player())
            },
        })
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
        let start_board = Ayoayo::new();
        let new_board = start_board.play(3)?;
        assert_eq!("0 - ①|⑥|⑥|②|⑦|①\n⑥|①|⑥|⑥|⑥|⓪ - 0", format!("{}", new_board));
        assert_eq!(new_board.state, GameState::InProgress(Player::Player2));
        println!("-- play2 --");
        let new_board = new_board.play(0)?;
        assert_eq!("0 - ②|⑨|②|⑤|⑩|①\n②|④|⓪|①|⑨|③ - 0", format!("{}", new_board));
        println!("-- play3 --");
        let new_board = new_board.play(0)?;
        assert_eq!("3 - ①|⑩|⓪|⑥|⑰|⓪\n⓪|⓪|①|②|⑩|④ - 0", format!("{}", new_board));
        println!("-- play4 --");
        let new_board = new_board.play(4)?;
        assert_eq!(
            "3 - ②|⑰|①|⑦|⓪|①\n①|①|⓪|③|①|⑤ - 12",
            format!("{}", new_board)
        );
        println!("-- play5 --");
        let new_board = new_board.play(2)?;
        assert_eq!(
            "3 - ⑥|①|⓪|①|⑤|③\n①|⑤|②|⑦|②|⓪ - 12",
            format!("{}", new_board)
        );
        println!("-- play6 --");
        let new_board = new_board.play(1)?;
        assert_eq!(
            "3 - ⓪|⓪|①|②|⑥|④\n②|①|③|⑧|③|① - 14",
            format!("{}", new_board)
        );
        println!("-- play7 --");
        let new_board = new_board.play(3)?;
        assert_eq!(
            "8 - ⓪|②|①|①|⓪|①\n④|③|⓪|⑩|①|③ - 14",
            format!("{}", new_board)
        );
        println!("-- play8 --");
        let new_board = new_board.play(3)?;
        assert_eq!(
            "8 - ①|⑤|④|⓪|①|④\n①|⓪|③|⓪|⑤|② - 14",
            format!("{}", new_board)
        );
        println!("-- play9 --");
        let new_board = new_board.play(1)?;
        assert_eq!(
            "8 - ⓪|①|⓪|②|③|⑥\n①|②|①|①|⑥|③ - 14",
            format!("{}", new_board)
        );
        println!("-- play10 --");
        let new_board = new_board.play(1)?;
        assert_eq!(
            "8 - ①|②|①|⓪|④|⓪\n⓪|①|⓪|①|⑧|① - 21",
            format!("{}", new_board)
        );
        println!("-- play11 --");
        let new_board = new_board.play(0)?;
        assert_eq!(
            "10 - ①|①|②|①|⓪|①\n①|⓪|①|⓪|⑨|⓪ - 21",
            format!("{}", new_board)
        );
        println!("-- play12 --");
        let new_board = new_board.play(4)?;
        assert_eq!(
            "10 - ②|⓪|③|②|①|②\n②|①|①|⓪|⓪|① - 23",
            format!("{}", new_board)
        );
        println!("-- play13 --");
        let new_board = new_board.play(2)?;
        assert_eq!(
            "10 - ②|⓪|⓪|③|②|⓪\n③|②|⓪|①|①|① - 23",
            format!("{}", new_board)
        );
        println!("-- play14 --");
        let new_board = new_board.play(3)?;
        assert_eq!(
            "10 - ⓪|①|①|⓪|⓪|①\n④|⓪|①|①|①|② - 26",
            format!("{}", new_board)
        );
        println!("-- play15 --");
        let new_board = new_board.play(5)?;
        assert_eq!(
            "12 - ①|②|⓪|①|①|⓪\n⓪|①|②|②|⓪|⓪ - 26",
            format!("{}", new_board)
        );
        println!("-- play16 --");
        let new_board = new_board.play(2)?;
        assert_eq!(
            "12 - ①|②|⓪|①|⓪|⓪\n⓪|①|⓪|③|①|⓪ - 27",
            format!("{}", new_board)
        );
        println!("-- play17 --");
        let new_board = new_board.play(3)?;
        assert_eq!(
            "13 - ①|②|⓪|⓪|①|⓪\n⓪|①|⓪|③|⓪|⓪ - 27",
            format!("{}", new_board)
        );
        println!("-- play18 --");
        let new_board = new_board.play(3)?;
        assert_eq!(
            "13 - ⓪|③|①|⓪|①|⓪\n⓪|①|⓪|⓪|①|① - 27",
            format!("{}", new_board)
        );
        println!("-- play19 --");
        let new_board = new_board.play(4)?;
        assert_eq!(
            "14 - ⓪|③|①|⓪|⓪|①\n⓪|①|⓪|⓪|①|⓪ - 27",
            format!("{}", new_board)
        );
        println!("-- play20 --");
        let new_board = new_board.play(1)?;
        assert_eq!(
            "14 - ⓪|③|⓪|⓪|⓪|①\n⓪|⓪|①|⓪|①|⓪ - 28",
            format!("{}", new_board)
        );
        println!("-- play21 --");
        let new_board = new_board.play(5)?;
        assert_eq!(
            "14 - ⓪|③|⓪|⓪|⓪|⓪\n①|⓪|①|⓪|①|⓪ - 28",
            format!("{}", new_board)
        );
        println!("-- play22 --");
        let new_board = new_board.play(0)?;
        assert_eq!(
            "14 - ⓪|⓪|⓪|⓪|⓪|⓪\n⓪|①|①|⓪|①|⓪ - 31",
            format!("{}", new_board)
        );
        assert_eq!(new_board.state, GameState::Won(Player::Player2));
        Ok(())
    }

    #[test]
    fn must_feed_test() {
        let game = Ayoayo {
            board: MancalaBoard::new(
                &[
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
        let game = Ayoayo {
            board: MancalaBoard::new(
                &[
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
