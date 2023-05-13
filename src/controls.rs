use std::ops::RangeInclusive;
use yew::prelude::*;
use yewdux::prelude::{use_store, Dispatch};

use crate::{letter::Letter, state::{GameState, GameMessage}};


#[derive(Properties, PartialEq)]
pub struct ControlProps {}

#[function_component(Controls)]
pub fn controls(_props: &ControlProps) -> Html {
    html!(
        <div class="controls">
            <UndoButton/>
            <ClearButton/>

        </div>
    )
}

#[function_component(UndoButton)]
pub fn undo_button()-> Html{
    let (state, dispatch) = use_store::<GameState>();
    let disabled = state.history.is_empty();

    let onclick = dispatch.apply_callback(|_|GameMessage::Undo);

    html!(
        <button {disabled} {onclick} class="undo-button">
            {"Undo"}
        </button>
    )
}

#[function_component(ClearButton)]
pub fn clear_button()-> Html{
    let (state, dispatch) = use_store::<GameState>();
    let disabled = state.solution.letters.iter().all(|x|x.is_none());

    let onclick = dispatch.apply_callback(|_|GameMessage::Clear);

    html!(
        <button {disabled} {onclick} class="clear-button">
            {"Clear"}
        </button>
    )
}
