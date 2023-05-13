use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::state::{GameMessage, GameState};

#[derive(Properties, PartialEq)]
pub struct HeaderProps {}

#[function_component(Header)]
pub fn slots(_props: &HeaderProps) -> Html {
    let (state, dispatch) = use_store::<GameState>();



    let win = if state.is_real_word() {
        html!(<h2>{"Congratulations"} </h2>)
    } else {
        let solutions = state.possible_solutions_count();
        html!(<h3>{ format!("{solutions:3<} Possible Solutions")} </h3>)
    };

    let on_new_game_click = dispatch.apply_callback(|_| GameMessage::NewGame(None));

    html!(
        <div class="header">
        {win}
        <button class="new-game-button" onclick={on_new_game_click}>{"New Game"}</button>
        </div>
    )
}
