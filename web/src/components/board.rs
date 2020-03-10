use mancala::{ayoayo::Ayoayo, board::Cup, GameState, MancalaError, Player};
use std::time::Duration;
use yew::services::{IntervalService, Task};
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

use super::log;

pub(crate) struct Board {
    board: Ayoayo,
    link: ComponentLink<Self>,
    play_click: Callback<usize>,
    // Keeping track of the interval
    job: Box<dyn Task>,
    // Thoughts: MancalalBoard might be able to "replay" moves...
    // Does that drastically change what MancalaBoard looks like internally?
    // Interval replays an iterator a step at a time, so store an iterator here
}

#[derive(Properties, Clone)]
pub(crate) struct Props {
    pub(crate) board: Ayoayo,
    pub(crate) play_click: Callback<usize>,
}

pub(crate) enum Msg {
    Collect(usize),
}

impl Component for Board {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Board {
            link,
            board: props.board,
            play_click: props.play_click,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Collect(pos) => {
                self.play_click.emit(pos);
                false
            }
        }
    }

    fn view(&self) -> Html {
        let cup_comp = |cup: &Cup| {
            let pos = cup.pos;
            let seeds = cup.seeds;
            let disabled = match self.board.state {
                GameState::InProgress(player) => cup.owner != player,
                _ => true,
            } || cup.seeds == 0;
            html! {
                <button class="cup" disabled=disabled onclick=self.link.callback(move |_| Msg::Collect(pos.clone())) >{seeds}</button>
            }
        };

        html! {
            <>
                <div class="bank1name">{"Player 1 Bank"}</div>
                <div class="bank1">{self.board.get_bank(Player::Player1)}</div>
                <div class="board">
                    <div>
                        {self.board.get_cups_for_player(Player::Player1).iter().map(cup_comp).collect::<Html>()}
                    </div>
                    <div>
                        {self.board.get_cups_for_player(Player::Player2).iter().map(cup_comp).collect::<Html>()}
                    </div>
                </div>
                <div class="bank2name">{"Player 2 Bank"}</div>
                <div class="bank2">{self.board.get_bank(Player::Player2)}</div>
            </>
        }
    }
    // fn mounted(&mut self) -> ShouldRender {
    //     false
    // }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        log(&format!(
            "Changes?\n{}\n{}\n{:?}",
            self.board,
            props.board,
            self.board != props.board
        )[..]);
        if self.board != props.board {
            self.board = props.board;
            true
        } else {
            false
        }
    }
}
