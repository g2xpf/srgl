#[cfg(feature = "default-backend")]
pub struct World<G, R = libsrgl_reaction::Context, E = libsrgl_event::Context> {
    global_resource: G,
    reaction_context: R,
    event_context: E,
}

#[cfg(not(feature = "default-backend"))]
pub struct World<G, R, E> {
    global_resource: G,
    reaction_context: R,
    event_context: E,
}
