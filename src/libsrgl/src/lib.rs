#[cfg(feature = "default-backend")]
pub struct World<G, R = libsrgl_reaction::Context, E = libsrgl_event::Context> {
    pub global_resource: G,
    pub reaction_context: R,
    pub event_context: E,
}

#[cfg(not(feature = "default-backend"))]
pub struct World<G, R, E> {
    pub global_resource: G,
    pub reaction_context: R,
    pub event_context: E,
}

impl<G, R, E> World<G, R, E> {
    pub fn new(global_resource: G, reaction_context: R, event_context: E) -> Self {
        World {
            global_resource,
            reaction_context,
            event_context,
        }
    }

    pub fn run(&mut self) {}
}
