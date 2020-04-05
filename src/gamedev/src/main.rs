use srgl::{ReceiveFrom, Router, State, World};
use srgl_input::Context as InCtx;
use srgl_output::Context as OutCtx;

struct GlobalResource;

struct S1(i32);
struct S2(i32);

impl State for S1 {
    type Global = GlobalResource;
    type InCtx = InCtx;
    type OutCtx = OutCtx;

    fn input(&mut self, _: &mut Self::Global, _: &Self::InCtx) {}

    fn update(
        &mut self,
        _: &mut Self::Global,
        router: Router<Self::Global, Self::InCtx, Self::OutCtx>,
    ) {
        self.0 += 1;
        if self.0 % 50 == 0 {
            self.send_with::<S2>(router, self.0);
        }
    }

    fn output(&self, _: &Self::Global, _: &mut Self::OutCtx) {
        std::thread::sleep(std::time::Duration::from_millis(16));
        println!("S1: {}", self.0);
    }
}

impl State for S2 {
    type Global = GlobalResource;
    type InCtx = InCtx;
    type OutCtx = OutCtx;

    fn update(
        &mut self,
        _: &mut Self::Global,
        router: Router<Self::Global, Self::InCtx, Self::OutCtx>,
    ) {
        self.0 += 1;
        if self.0 % 50 == 0 {
            self.send_with::<S1>(router, self.0);
        }
    }

    fn output(&self, _: &Self::Global, _: &mut Self::OutCtx) {
        std::thread::sleep(std::time::Duration::from_millis(16));
        println!("S2: {}", self.0);
    }
}

impl ReceiveFrom<S1> for S2 {
    type Message = i32;
    fn receive(&mut self, message: Self::Message) {
        self.0 = message;
    }
}

impl ReceiveFrom<S2> for S1 {
    type Message = i32;
    fn receive(&mut self, message: Self::Message) {
        self.0 = message;
    }
}

fn main() {
    let s1 = S1(0);
    let s2 = S2(0);

    println!("{:?}", std::any::Any::type_id(&s2));

    let mut world = World::new(GlobalResource, InCtx, OutCtx, s1).insert_state(s2);

    world.set_input_before(|&mut GlobalResource, &InCtx| {
        println!("input before");
    });

    world.set_input_after(|&mut GlobalResource, &InCtx| {
        println!("input after");
    });

    world.set_output_before(|&mut GlobalResource, &mut OutCtx| {
        println!("output before");
    });

    world.set_output_after(|&mut GlobalResource, &mut OutCtx| {
        println!("output after");
    });

    let mut world = world.finalize();

    world.run();
}
