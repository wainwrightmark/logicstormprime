use std::ops::RangeInclusive;
use yew::prelude::*;
use yewdux::prelude::{use_store, Dispatch};

use crate::{letter::Letter, state::{GameState, GameMessage}};


#[derive(Properties, PartialEq)]
pub struct KeyboardProps {}

#[function_component(Keyboard)]
pub fn keyboard(_props: &KeyboardProps) -> Html {

    let (state, _) = use_store::<GameState>();
    let allowed_range = state.legal_letters_for_index(state.selected_index).unwrap_or_else(||Letter::Z..=Letter::A);
    let backspace_enabled = state.letters[state.selected_index as usize].is_some();
    html!(
        <div class="keyboard">
            <KeyRow range={Letter::A..=Letter::I} allowed_range={allowed_range.clone()}/>
            <KeyRow range={Letter::J..=Letter::R} allowed_range={allowed_range.clone()}/>
            <KeyRow range={Letter::S..=Letter::Z} allowed_range={allowed_range.clone()} backspace={backspace_enabled}/>
        </div>
    )
}

#[derive(Properties, PartialEq)]
pub struct KeyRowProps{
    pub range: RangeInclusive<Letter>,
    pub allowed_range: RangeInclusive<Letter>,
    pub backspace: Option<bool>
}

#[function_component(KeyRow)]
pub fn key_row(props: &KeyRowProps) -> Html{
    let keys = props.range.clone().map(|letter| html!(<Key letter={letter} disabled={!props.allowed_range.contains(&letter)} />)).collect::<Html>();

    let bs = if let Some(enabled) =  props.backspace{
        html!(<BackspaceKey  disabled={!enabled} />)
    }else{
        html!()
    };
    html!(
        <div class="keyboard-row">
            {keys}
            {bs}
        </div>
    )
}

#[derive(Properties, PartialEq)]
pub struct KeyProps {
    pub letter: Letter,
    pub disabled : bool
}

#[function_component(Key)]
pub fn key(props: &KeyProps) -> Html{

    let dispatch = Dispatch::<GameState>::new();
    let letter = props.letter;
    let onclick = dispatch.apply_callback(move |_| GameMessage::TypeLetter(letter));

    let disabled = props.disabled;

    html!(<button key={props.letter.to_string()} {disabled} {onclick} class={"key"}>{props.letter}</button> )
}

#[derive(Properties, PartialEq)]
pub struct BackspaceKeyProps {
    pub disabled : bool
}

#[function_component(BackspaceKey)]
pub fn key(props: &BackspaceKeyProps) -> Html{

    let dispatch = Dispatch::<GameState>::new();
    let onclick = dispatch.apply_callback(move |_| GameMessage::Backspace);

    let disabled = props.disabled;

    html!(<button key={"backspace"} {disabled} {onclick} class={"key"}>{"↚"}</button> )
}