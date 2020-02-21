use std::{cell::RefCell, fmt};

mod board;

use board::{Cup, MancalaBoard, Player};

const BOARD_SIZE: usize = 12;
const STARTING_COUNT: usize = 4;

#[derive(Clone)]
pub struct Ayoayo {
    pub(crate) board: MancalaBoard,
}
impl Ayoayo {
    pub fn new() -> Ayoayo {
        let board: Vec<board::Cup> = [Player::Player1, Player::Player2]
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
        };
    }

    fn sow_filter(check_cup: &RefCell<Cup>, player: Player, start_cup: usize) -> bool {
        let cup = check_cup.borrow();
        !(cup.owner == player && cup.pos == start_cup)
    }

    pub fn play(&self, cup: usize, player: Player) -> Ayoayo {
        let (mut b, mut last) = self
            .board
            .pickup(
                Cup {
                    owner: player,
                    pos: cup,
                    seeds: 0,
                },
                player,
            )
            .sow(player, cup, Ayoayo::sow_filter);
        while last.seeds > 1 {
            println!("{}", b);
            let (tb, tlast) = b
                .pickup(last, player)
                .sow(player, last.pos, Ayoayo::sow_filter);
            b = tb;
            last = tlast;
        }
        if last.owner == player {
            Ayoayo {
                board: b
                    .pickup(
                        Cup {
                            owner: match player {
                                Player::Player1 => Player::Player2,
                                Player::Player2 => Player::Player1,
                            },
                            ..last
                        },
                        player,
                    )
                    .bank(player),
            }
        } else {
            Ayoayo { board: b }
        }
    }
}

impl fmt::Display for Ayoayo {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play() {
        let start_board = Ayoayo::new();
        let new_board = start_board.play(3, Player::Player1);
        assert_eq!("0 - ④|⑤|⑥|②|⑦|⑦\n①|⓪|④|④|④|④ - 0", format!("{}", new_board));
        let new_board = new_board.play(0, Player::Player2);
        assert_eq!("0 - ④|⓪|⑥|②|⑦|⑦\n⓪|①|④|④|④|④ - 5", format!("{}", new_board));
    }
}
