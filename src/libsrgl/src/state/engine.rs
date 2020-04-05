use super::router::{
    mode::{Preparing, Ready},
    RawRouter, StateEntry,
};
use super::{State, StateID};

use std::mem;

pub struct StateEngine<M, G, I, O> {
    current_state: StateEntry<G, I, O>,
    pub router: RawRouter<M, G, I, O>,
}

impl<G, I, O> StateEngine<Preparing, G, I, O> {
    pub fn new<S>(initial_state: S) -> Self
    where
        S: State<Global = G, InCtx = I, OutCtx = O>,
    {
        let state_id = StateID::of::<S>();
        let state_box = Box::new(initial_state);
        StateEngine {
            router: RawRouter::new(),
            current_state: (state_id, state_box),
        }
    }

    pub fn insert<S>(&mut self, state: S)
    where
        S: State<Global = G, InCtx = I, OutCtx = O>,
    {
        self.router.insert::<S>(Box::new(state));
    }

    pub fn insert_entry(&mut self, state_entry: StateEntry<G, I, O>) {
        self.router.insert_entry(state_entry);
    }

    pub fn run(self) -> StateEngine<Ready, G, I, O> {
        StateEngine {
            router: self.router.run(),
            current_state: self.current_state,
        }
    }
}

impl<G, I, O> StateEngine<Ready, G, I, O>
where
    G: 'static,
    I: 'static,
    O: 'static,
{
    pub fn initialize(&mut self, global: &mut G) {
        self.current_state.1.initialize(global);
    }

    pub fn input(&mut self, global: &mut G, ctx: &I) {
        self.current_state.1.input(global, ctx);
    }

    pub fn update(&mut self, global: &mut G) {
        self.current_state.1.update(global, &mut self.router);

        if let Some(mut next_state) = self.router.take_next() {
            self.current_state.1.finalize(global);
            next_state.1.initialize(global);
            mem::swap(&mut self.current_state, &mut next_state);
            self.router.insert_entry(next_state);
        }
    }

    pub fn output(&self, global: &G, ctx: &mut O) {
        self.current_state.1.output(global, ctx)
    }
}
