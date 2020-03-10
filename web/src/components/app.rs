use mancala::{ayoayo::Ayoayo, GameState, MancalaError};

use yew::{html, Callback, ClickEvent, Component, ComponentLink, Html, ShouldRender};

use super::board::Board;
use super::log;

pub(crate) struct App {
    game: Ayoayo,
    onclick: Callback<usize>,
    restart: Callback<ClickEvent>,
    errors: Option<MancalaError>,
}

pub(crate) enum Msg {
    Play(usize),
    Restart,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            game: Ayoayo::new(),
            onclick: link.callback(Msg::Play),
            restart: link.callback(|_| Msg::Restart),
            errors: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Play(pos) => {
                let result = self.game.play(pos);
                self.errors = result.err();

                true
            }
            Msg::Restart => {
                self.game = Ayoayo::new();
                self.errors = None;

                true
            }
        }
    }

    fn view(&self) -> Html {
        log(&format!("{:?}", self.game)[..]);
        let info = |info: &str| html! {<div class="info">{info}</div>};

        let error = match self.errors {
            None => html! {<></>},
            Some(MancalaError::MustFeedError) => info("You need to Sow to your oppoent"),
            Some(MancalaError::NoSeedsToSow) => info("No Seeds in that Cup"),
            Some(MancalaError::NoSuchCup) => info("Cup doesn't exist, how did you click on it?"),
        };
        let game_state = match self.game.state {
            GameState::Won(player) => info(&format!("{} Won!", player)),
            GameState::Draw => info("Nobody Won!"),
            GameState::InProgress(player) => info(&format!("{}'s Turn.", player)),
        };

        html! {
            <div class="root">
                {error}
                {game_state}
                <Board board=&self.game.clone() play_click=&self.onclick />
                <button class="restart" onclick=&self.restart>{"restart"}</button>
            </div>
        }
    }
}
