use druid::PlatformError;

use crate::world::World;

mod fragment;
mod squares;
mod window;
mod world;
mod world_view;
mod item;

fn main() -> Result<(), PlatformError> {
    println!("Hello, world!");
    //window::do_a_window()

    let mut world = World::new();
    world_view::do_a_window(world)
}
