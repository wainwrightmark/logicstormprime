use itertools::Itertools;
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::state::{GameMessage, GameState};

#[derive(Properties, PartialEq)]
pub struct AnswersProps {}

#[function_component(Answers)]
pub fn slots(_props: &AnswersProps) -> Html {
    let (state, dispatch) = use_store::<GameState>();

    if !state.is_real_word(){
        return Html::default();
    }

    let Some(answers) = state.all_solutions() else { return Html::default();};

    let rows : Html = answers.iter().map(|answer|
    html!(
        <tr>
        <td>
            {answer.iter().join("")}
        </td>
        </tr>

    ) ).collect();

    //let lines = state.possible_solutions_count()

    html!(
        <div class="answers">
        <h3>
            {"Possible Solutions"}
        </h3>
        <table class="answer-table">
        {rows}
        </table>
        </div>
    )
}
