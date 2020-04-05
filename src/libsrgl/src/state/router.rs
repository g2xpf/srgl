use super::{State, StateID};
use std::collections::HashMap;
use std::marker::PhantomData;

pub type Router<'a, G, I, O> = &'a mut RawRouter<Ready, G, I, O>;
pub type StateBox<G, I, O> = Box<dyn State<Global = G, InCtx = I, OutCtx = O> + 'static>;
pub type StateEntry<G, I, O> = (StateID, StateBox<G, I, O>);

pub mod mode {
    pub struct Preparing;
    pub struct Ready;
}

use mode::*;

pub struct RawRouter<M, G, I, O> {
    states: HashMap<StateID, StateBox<G, I, O>>,
    pub(super) next_state: Option<StateEntry<G, I, O>>,
    mode: PhantomData<M>,
}

impl<M, G, I, O> RawRouter<M, G, I, O> {
    pub fn insert<S>(&mut self, state_box: StateBox<G, I, O>)
    where
        S: State,
    {
        let state_id = StateID::of::<S>();
        self.states.insert(state_id, state_box);
    }

    pub fn insert_entry(&mut self, state_entry: StateEntry<G, I, O>) {
        let (state_id, state_box) = state_entry;
        self.states.insert(state_id, state_box);
    }

    pub(crate) fn remove<S>(&mut self) -> Option<StateBox<G, I, O>>
    where
        S: State,
    {
        let state_id = StateID::of::<S>();
        self.states.remove(&state_id)
    }
}

impl<G, I, O> RawRouter<Preparing, G, I, O> {
    pub fn new() -> Self {
        RawRouter {
            states: HashMap::new(),
            next_state: None,
            mode: PhantomData,
        }
    }

    pub fn run(self) -> RawRouter<Ready, G, I, O> {
        RawRouter {
            states: self.states,
            next_state: None,
            mode: PhantomData,
        }
    }
}

impl<G, I, O> RawRouter<Ready, G, I, O> {
    pub fn take_next(&mut self) -> Option<StateEntry<G, I, O>> {
        self.next_state.take()
    }
}
