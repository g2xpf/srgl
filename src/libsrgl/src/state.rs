pub mod engine;
pub mod router;

use std::any::{type_name, Any, TypeId};

pub use engine::StateEngine;
pub use router::mode;
pub use router::Router;

pub type StateID = TypeId;

pub trait ReceiveFrom<Sender>: State
where
    Sender: State,
{
    type Message;
    fn receive(&mut self, _: Self::Message) {}
}

pub trait State: Any + 'static {
    type Global;
    type InCtx;
    type OutCtx;

    fn initialize(&mut self, _: &mut Self::Global) {}
    fn finalize(&mut self, _: &mut Self::Global) {}

    fn input(&mut self, _: &mut Self::Global, _: &Self::InCtx) {}
    fn update(&mut self, _: &mut Self::Global, _: Router<Self::Global, Self::InCtx, Self::OutCtx>) {
    }
    fn output(&self, _: &Self::Global, _: &mut Self::OutCtx) {}

    fn send<S>(&self, router: Router<Self::Global, Self::InCtx, Self::OutCtx>)
    where
        Self: Sized,
        S: ReceiveFrom<Self>,
        S: State<Global = Self::Global, InCtx = Self::InCtx, OutCtx = Self::OutCtx>,
    {
        if let Some(next_state) = router.next_state.take() {
            router.insert_entry(next_state)
        } else {
            panic!("Cannot set the next state twice");
        }

        let next_state = router
            .remove::<S>()
            .unwrap_or_else(|| panic!("Tried to make a transition to the unregistered state"));
        let next_state_id = StateID::of::<S>();

        router.next_state = Some((next_state_id, next_state));
    }

    fn send_with<S>(
        &self,
        router: Router<Self::Global, Self::InCtx, Self::OutCtx>,
        message: S::Message,
    ) where
        Self: Sized,
        S: ReceiveFrom<Self>,
        S: State<Global = Self::Global, InCtx = Self::InCtx, OutCtx = Self::OutCtx>,
    {
        if router.next_state.is_some() {
            panic!("Cannot set the next state `{}` twice", type_name::<Self>());
        }

        let mut next_state = router
            .remove::<S>()
            .unwrap_or_else(|| panic!("Tried to make a transition to the unregistered state"));
        let next_state_id = StateID::of::<S>();

        next_state
            .downcast_mut::<S>()
            .unwrap_or_else(|| panic!("downcast to `{}` failed", type_name::<S>()))
            .receive(message);

        router.next_state = Some((next_state_id, next_state));
    }
}

impl<G, I, O> dyn State<Global = G, InCtx = I, OutCtx = O>
where
    G: 'static,
    I: 'static,
    O: 'static,
{
    #[inline]
    pub fn is<S: State>(&self) -> bool {
        StateID::of::<S>() == Any::type_id(self)
    }

    #[inline]
    pub fn downcast_ref<S: State>(&self) -> Option<&S> {
        if <dyn State<Global = G, InCtx = I, OutCtx = O>>::is::<S>(self) {
            unsafe {
                Some(&*(self as *const dyn State<Global = G, InCtx = I, OutCtx = O> as *const S))
            }
        } else {
            None
        }
    }

    #[inline]
    pub fn downcast_mut<S: State>(&mut self) -> Option<&mut S> {
        if <dyn State<Global = G, InCtx = I, OutCtx = O>>::is::<S>(self) {
            unsafe {
                Some(&mut *(self as *mut dyn State<Global = G, InCtx = I, OutCtx = O> as *mut S))
            }
        } else {
            None
        }
    }
}
