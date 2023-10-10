use std::{
    collections::{HashMap, HashSet},
    fmt,
    sync::Mutex,
};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::{Alphabet, State, FSM};

#[derive(Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct TransitionKey<K1, K2>((K1, K2));

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct TransitionValue<V1, V2>((V1, V2));

#[derive(Debug, Deserialize, Serialize)]
pub struct TransitionMap<I: Alphabet, O: Alphabet, S: State>(
    HashMap<TransitionKey<S, I>, TransitionValue<S, O>>,
);

#[derive(Deserialize, Serialize)]
pub struct Mealy<I: Alphabet, O: Alphabet, S: State> {
    cur_state: Mutex<S>,
    pub states: HashSet<S>,
    pub transitions: TransitionMap<I, O, S>,
}

impl<I: Alphabet, O: Alphabet, S: State> Mealy<I, O, S> {
    pub fn new(initial_state: S) -> Self {
        Self {
            cur_state: Mutex::new(initial_state.clone()),
            states: HashSet::from([initial_state]),
            transitions: TransitionMap(HashMap::new()),
        }
    }
}

impl<I: Alphabet, O: Alphabet, S: State> FSM<I, O, S> for Mealy<I, O, S> {
    fn add_state(&mut self, state: S) {
        self.states.insert(state);
    }

    fn try_add_transition(&mut self, from: S, input: I, to: S, output: O) -> Result<()> {
        if !self.states.contains(&from) {
            return Err(anyhow!("Unknown origin state: {:?}", from));
        }
        if !self.states.contains(&to) {
            return Err(anyhow!("Unknown destination state: {:?}", to));
        }
        self.transitions
            .0
            .insert(TransitionKey((from, input)), TransitionValue((to, output)));
        Ok(())
    }

    fn try_set_state(&self, state: S) -> Result<()> {
        if self.states.contains(&state) {
            let mut cur_state = self.cur_state.lock().unwrap();
            *cur_state = state.clone();
        } else {
            return Err(anyhow!(format!("Unknown state: {:?}", state)));
        }
        Ok(())
    }

    fn state(&self) -> S {
        let cur_state = self.cur_state.lock().unwrap();
        cur_state.clone()
    }

    fn next(&self, input: I) -> Option<O> {
        let mut cur_state = self.cur_state.lock().unwrap();
        if let Some(value) = self
            .transitions
            .0
            .get(&TransitionKey((cur_state.clone(), input)))
        {
            *cur_state = value.0 .0.clone();
            return Some(value.0 .1.clone());
        }
        None
    }
}

impl<I: Alphabet, O: Alphabet, S: State> PartialEq for Mealy<I, O, S> {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
            && self.states == other.states
            && self.transitions.0 == other.transitions.0
    }
}

impl<I: Alphabet, O: Alphabet, S: State> fmt::Debug for Mealy<I, O, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Mealy")
            .field("cur_state", &*self.cur_state.lock().unwrap())
            .field("states", &self.states)
            .field("transitions", &self.transitions.0)
            .finish()
    }
}
