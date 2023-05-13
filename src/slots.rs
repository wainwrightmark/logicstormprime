use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::state::{GameMessage, GameState};

#[derive(Properties, PartialEq)]
pub struct SlotsProps {}

#[function_component(Slots)]
pub fn slots(_props: &SlotsProps) -> Html {
    html!(
        <div class="slots">
            <Slot index={0} />
            <Slot index={1} />
            <Slot index={2} />
            <Slot index={3} />
            <Slot index={4} />
        </div>
    )
}

#[derive(Properties, PartialEq)]
pub struct SlotProps {
    pub index: u8,
}

#[function_component(Slot)]
pub fn slot(props: &SlotProps) -> Html {
    let (state, dispatch) = use_store::<GameState>();
    let index = props.index;
    let onclick = dispatch.apply_callback(move |_| GameMessage::SelectIndex(index));

    let number = state.perm.element_at_index(index, |x| x);

    let letter = state.solution.letters[index as usize]
        .map(|x| x.to_string())
        .unwrap_or_default();

    let class = if index == state.solution.selected_index {
        classes!("slot", "slot-focused")
    } else {
        classes!("slot")
    };

    html!(
        <div class="slot-pair">
            <button {onclick} {class}>{letter}</button>
            <label class="slot-number">{number + 1} </label>

        </div>
    )
}
