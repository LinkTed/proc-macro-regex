mod nfa_to_dfa;
mod to_tokens;

use crate::character::Character;
use crate::dfa::nfa_to_dfa::NfaToDfaIter;
pub(super) use crate::dfa::to_tokens::DfaToTokens;
use crate::nfa::Nfa;
use std::convert::From;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
};

#[derive(Debug)]
pub(crate) struct Dfa<T>
where
    T: Character,
{
    states: BTreeSet<usize>,
    transitions: BTreeMap<usize, BTreeSet<(T, usize)>>,
    accept_states: BTreeSet<usize>,
    start_text: bool,
    end_text: bool,
}

impl<T> From<Nfa<T>> for Dfa<T>
where
    T: Character + Copy,
{
    fn from(nfa: Nfa<T>) -> Self {
        let nfa_to_dfa = NfaToDfaIter::new(nfa);
        Dfa::from(nfa_to_dfa)
    }
}
