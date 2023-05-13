use importunate::Permutation;

use itertools::Itertools;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::iter::Step;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::sync::OnceLock;
use yewdux::store::Reducer;
use yewdux::store::Store;
pub type Perm = Permutation<u8, 5>;

use crate::letter::Letter;

#[derive(PartialEq, Eq, Clone, serde:: Serialize, serde::Deserialize, Store, Debug, Default)]
#[store(storage = "local", storage_tab_sync)]
pub struct GameState {
    pub perm: Perm,
    pub letters: [Option<Letter>; 5],
    pub selected_index: u8,
}

impl GameState {
    pub fn is_real_word(&self) -> bool {
        if self.letters.iter().all(|x| x.is_some()) {
            let arr = self.letters.map(|x| x.unwrap());
            return all_words()
                .get(&self.perm)
                .map(|x| x.contains(&arr))
                .unwrap_or_default();
        }
        return false;
    }

    pub fn total_solutions(&self)-> usize{
        return all_words()
        .get(&self.perm).map(|x|x.len()).unwrap_or_default();
    }

    pub fn possible_solutions(&self)->usize{

        let count = self.letters.iter().flatten().count();

        if count == 5 {
            let arr = self.letters.map(|x| x.unwrap());
            return all_words()
                .get(&self.perm)
                .map(|x| x.contains(&arr))
                .unwrap_or_default().then(||1).unwrap_or_default();
        }
        else if count == 0{
            return  self.total_solutions();
        }

        return all_words()
                .get(&self.perm)
                .map(|set| set.iter().filter(|x| {
                    self.letters.iter().zip(x.iter()).all(|(l,r)|l.is_none() || l.unwrap() == *r )
                }).count()).unwrap_or_default();

    }

    pub fn is_legal(&self) -> bool {
        if self.letters.iter().filter(|x| x.is_some()).count() <= 1 {
            return true;
        }
        let mut arr = self.letters;
        self.perm.invert().apply(&mut arr);
        arr.iter().flat_map(|x| x).is_sorted() && arr.iter().flat_map(|x| x).all_unique()
    }

    pub fn legal_letters_for_index(&self, index: u8) -> Option<RangeInclusive<Letter>> {
        let mut min = Letter::A;
        let mut max = Letter::Z;

        let old_index = self.perm.element_at_index(index, |x| x);
        let mut arr = self.letters;
        self.perm.invert().apply(&mut arr);
        for i in 0..old_index {
            match arr[i as usize] {
                Some(l) => {
                    if let Some(new_min) = Step::forward_checked(l, 1) {
                        min = new_min
                    } else {
                        return None;
                    }
                }
                None => {
                    if let Some(new_min) = Step::forward_checked(min, 1) {
                        min = new_min
                    } else {
                        return None;
                    }
                }
            }
        }

        for j in (((old_index + 1) as usize)..arr.len()).rev() {
            match arr[j] {
                Some(l) => {
                    if let Some(new_max) = Step::backward_checked(l, 1) {
                        max = new_max
                    } else {
                        return None;
                    }
                }
                None => {
                    if let Some(new_max) = Step::backward_checked(max, 1) {
                        max = new_max
                    } else {
                        return None;
                    }
                }
            }
        }

        Some(min..=max)
    }
}

fn all_words() -> &'static BTreeMap<Perm, BTreeSet<[Letter; 5]>> {
    static INSTANCE: OnceLock<BTreeMap<Perm, BTreeSet<[Letter; 5]>>> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        let mut map: BTreeMap<Perm, BTreeSet<[Letter; 5]>> = BTreeMap::new();
        let words = include_str!("words.txt").lines();

        'words: for word in words.filter(|x| x.is_ascii() && x.len() == 5) {
            let mut arr = [Letter::A; 5];
            let letters = word.chars().map(|c| {
                let mut my_buf: [u8; 4] = [0; 4];
                let my_str: &str = c.encode_utf8(&mut my_buf);

                Letter::from_str(my_str)
            });

            for (i, l) in letters.enumerate() {
                if let Ok(letter) = l {
                    arr[i] = letter;
                } else {
                    continue 'words;
                }
            }

            if arr.iter().all_unique() {
                let perm = Perm::calculate_incomplete(arr.as_slice());
                map.entry(perm)
                    .or_insert(Default::default())
                    .extend_one(arr);
            }
        }
        map
    })
}

#[derive(Debug, PartialEq, Eq, Default)]
pub enum GameMessage {
    #[default]
    None,
    NewGame(Option<Perm>),
    Clear,
    Backspace,
    Delete,
    ArrowLeft,
    ArrowRight,
    SelectIndex(u8),
    TypeLetter(Letter),
}

impl Reducer<GameState> for GameMessage {
    fn apply(self, state: std::rc::Rc<GameState>) -> std::rc::Rc<GameState> {
        let mut s = (*state).clone();
        match self {
            GameMessage::NewGame(perm) => {
                if let Some(perm) = perm{
                    s.perm = perm;
                }else{
                    let mut rng = ThreadRng::default();
                    s.letters = Default::default();
                    s.selected_index = 0;
                    let inner = rng.gen_range(0..=(Perm::get_last().inner()));
                    s.perm = Perm::from(inner);
                }

            }
            GameMessage::Clear => {
                s.letters = Default::default();
            }
            GameMessage::SelectIndex(i) => {
                s.selected_index = i % 5;
            }
            GameMessage::TypeLetter(l) => {
                s.letters[s.selected_index as usize] = Some(l);
                if s.selected_index < 4{
                    s.selected_index = s.selected_index + 1
                }
                if !s.is_legal() {
                    return state;
                }
            }
            GameMessage::None => return state,
            GameMessage::Delete => {
                s.letters[s.selected_index as usize] = None;
            },
            GameMessage::Backspace => {
                s.letters[s.selected_index as usize] = None;
                s.selected_index = s.selected_index.saturating_sub(1);
            }
            GameMessage::ArrowLeft => {
                s.selected_index = s.selected_index.saturating_sub(1);
            }
            GameMessage::ArrowRight => {
                if s.selected_index < 4{
                    s.selected_index = s.selected_index + 1
                }
            }
        }

        s.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::letter::Letter;

    use super::{GameState, Perm, all_words};
    #[test]
    pub fn test_letters() {
        let state = GameState {
            perm: Default::default(),
            letters: [None, Some(Letter::C), None, Some(Letter::I), None],
            selected_index: 0,
        };

        assert!(state.is_legal());

        assert_eq!(
            state.legal_letters_for_index(0),
            Some(Letter::A..=Letter::B)
        );
        assert_eq!(
            state.legal_letters_for_index(1),
            Some(Letter::B..=Letter::G)
        );
        assert_eq!(
            state.legal_letters_for_index(2),
            Some(Letter::D..=Letter::H)
        );
        assert_eq!(
            state.legal_letters_for_index(3),
            Some(Letter::E..=Letter::Y)
        );
        assert_eq!(
            state.legal_letters_for_index(4),
            Some(Letter::J..=Letter::Z)
        );
    }


    #[test]
    pub fn find_words(){
        let perm = Perm::calculate_incomplete("sword".as_bytes());
        for word in all_words().get(&perm).unwrap(){
            let word = itertools::Itertools::join(&mut word.iter(), "");

            println!("{word}")
        }
    }
}
