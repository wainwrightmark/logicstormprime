use std::iter::Step;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, EnumProperty, EnumString, IntoStaticStr, FromRepr};


#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Debug,
    Serialize,
    Deserialize,
    FromRepr,
    EnumProperty,
    EnumString,
    EnumIter,
    EnumCount,
    IntoStaticStr,
    Display,
    Hash,
)]
#[strum(ascii_case_insensitive)]
pub enum Letter{
    A = 1,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z
}

impl Step for Letter{
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        (*end as usize).checked_sub(*start as usize)
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        Self::from_repr((start as usize) + count)
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        (start as usize).checked_sub(count).and_then(|x| Self::from_repr(x))
    }
}