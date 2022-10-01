use druid::PlatformError;
use fragment::{Zone, Fragment, IdType};

use crate::world::World;

mod fragment;
mod world;
mod world_view;

fn make_sample_world() -> World {
    let mut world = World::new();
    world.fragments.add(Fragment::new("u0", "Zone(0, 0, 1)", fragment::Shard::UnitIsInZone(Zone(0, 0, 1))));
    world.fragments.add(Fragment::new("u1", "Zone(3, 2, 1)", fragment::Shard::UnitIsInZone(Zone(3, 2, 1))));
    world.fragments.add(Fragment::new("u2", "Zone(-4, 2, 1)", fragment::Shard::UnitIsInZone(Zone(-4, 2, 1))));
    world
}

fn main() -> Result<(), PlatformError> {
    println!("Hello, world!");
    //window::do_a_window()

    let world = make_sample_world();
    world_view::do_a_window(world)
}
