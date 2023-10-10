use std::{fmt::Debug, hash::Hash};

use anyhow::Result;

pub mod mealy;
mod test;

pub trait FSM<I: Alphabet, O: Alphabet, S: State> {
    fn add_state(&mut self, state: S);
    fn try_add_transition(&mut self, from: S, input: I, to: S, output: O) -> Result<()>;
    fn try_set_state(&self, state: S) -> Result<()>;
    fn state(&self) -> S;
    fn next(&self, input: I) -> Option<O>;
}

pub trait Alphabet: Clone + Hash + Eq + Debug + Send {}
impl<T: Clone + Hash + Eq + Debug + Send> Alphabet for T {}

pub trait State: Clone + Hash + Eq + Debug + Send {}
impl<T: Clone + Hash + Eq + Debug + Send> State for T {}
