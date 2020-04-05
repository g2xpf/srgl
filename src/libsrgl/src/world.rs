use crate::state::{mode::*, State, StateEngine};

#[cfg(feature = "default-backend")]
pub struct World<M, G, I = srgl_input::Context, O = srgl_output::Context> {
    pub global_resource: G,
    pub input_context: I,
    pub output_context: O,
    pub state_engine: StateEngine<M, G, I, O>,

    pub input_before: Option<fn(&mut G, &I)>,
    pub input_after: Option<fn(&mut G, &I)>,

    pub output_before: Option<fn(&mut G, &mut O)>,
    pub output_after: Option<fn(&mut G, &mut O)>,
}

#[cfg(not(feature = "default-backend"))]
pub struct World<M, G, I, O> {
    pub global_resource: G,
    pub input_context: I,
    pub output_context: O,
    pub state_engine: StateEngine<M, G, I, O>,

    pub input_before: Option<fn(&mut G)>,
    pub input_after: Option<fn(&mut G)>,

    pub output_before: Option<fn(&mut O)>,
    pub output_after: Option<fn(&mut O)>,
}

impl<G, I, O> World<Preparing, G, I, O> {
    pub fn new<S>(global_resource: G, input_context: I, output_context: O, initial_state: S) -> Self
    where
        S: State<Global = G, InCtx = I, OutCtx = O>,
    {
        World {
            global_resource,
            input_context,
            output_context,
            state_engine: StateEngine::new(initial_state),

            input_before: None,
            input_after: None,

            output_before: None,
            output_after: None,
        }
    }

    pub fn insert_state<S>(mut self, state: S) -> Self
    where
        S: State<Global = G, InCtx = I, OutCtx = O>,
    {
        self.state_engine.insert(state);
        self
    }

    pub fn finalize(self) -> World<Ready, G, I, O> {
        World {
            global_resource: self.global_resource,
            input_context: self.input_context,
            output_context: self.output_context,
            state_engine: self.state_engine.run(),

            input_before: self.input_before,
            input_after: self.input_after,

            output_before: self.output_before,
            output_after: self.output_after,
        }
    }

    pub fn set_input_before(&mut self, f: fn(&mut G, &I)) {
        self.input_before = Some(f);
    }

    pub fn set_input_after(&mut self, f: fn(&mut G, &I)) {
        self.input_after = Some(f);
    }

    pub fn set_output_before(&mut self, f: fn(&mut G, &mut O)) {
        self.output_before = Some(f);
    }

    pub fn set_output_after(&mut self, f: fn(&mut G, &mut O)) {
        self.output_after = Some(f);
    }
}

impl<G, I, O> World<Ready, G, I, O>
where
    G: 'static,
    I: 'static,
    O: 'static,
{
    pub fn run(&mut self) {
        self.state_engine.initialize(&mut self.global_resource);

        let global_resource = &mut self.global_resource;
        let input_context = &mut self.input_context;
        let output_context = &mut self.output_context;

        loop {
            self.input_before.map(|f| f(global_resource, input_context));
            self.state_engine.input(global_resource, input_context);
            self.input_after.map(|f| f(global_resource, input_context));

            self.state_engine.update(global_resource);

            self.output_before
                .map(|f| f(global_resource, output_context));
            self.state_engine.output(global_resource, output_context);
            self.output_after
                .map(|f| f(global_resource, output_context));
        }
    }
}
