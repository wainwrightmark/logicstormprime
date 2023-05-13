use std::str::FromStr;

use yew::prelude::*;

use yew_hooks::{use_effect_once, use_search_param};
use yewdux::prelude::Dispatch;

use crate::{
    header::Header,
    keyboard::Keyboard,
    letter::Letter,
    slots::Slots,
    state::{GameMessage, GameState, Perm},
    answers::Answers, controls::Controls
};

#[function_component(App)]
pub fn app() -> Html {
    let key = use_search_param("key".to_string()).unwrap_or_default();

    let dispatch = Dispatch::<GameState>::new();
    {
        let dispatch = dispatch.clone();
        use_effect_once(move || {
            if key.len() == 5 {
                let perm = Perm::calculate_incomplete(key.as_bytes());
                dispatch.apply(GameMessage::NewGame(Some(perm)));
            }

            || {}
        });
    }

    let onkeydown = dispatch.apply_callback(|e: KeyboardEvent| {
        let key = e.key();
        if let Ok(letter) = Letter::from_str(&key) {
            GameMessage::TypeLetter(letter)
        } else if key == "Backspace" {
            GameMessage::Backspace
        }else if key == "Delete" {
            GameMessage::Delete
        }

        else if key == "ArrowLeft" {
            GameMessage::ArrowLeft
        } else if key == "ArrowRight" {
            GameMessage::ArrowRight
        } else {
            log::info!("{key}");
            GameMessage::None
        }
    });
    html! {
        <div {onkeydown} class="container">
            <div class="sm-4">
                <div class="centered">
                    <h1>{"Logic Storm Prime"}</h1 >
                </div>
                <div class="centered">
                    <Header />
                </div>

                <br/>
                <br/>
                <div class="centered">
                    <Slots/>
                </div>
                <br/>
                <br/>
                <div class="centered">
                    <Keyboard/>
                </div>
                <br/>
                <br/>
                <div class="centered">
                    <Controls/>
                </div>
                <br/>
                <br/>
                <div class="centered">
                    <Answers/>
                </div>

            </div>

        </div>

    }
}
