use druid::PlatformError;
use fragment::{Zone, Fragment, Shard::*};

use crate::world::World;

mod fragment;
mod world;
mod world_view;

fn make_sample_world() -> World {
    let mut world = World::new();
    world.fragments.add(Fragment::new("player", "Zone(0, 0, 1)", "UnitIsInZone", UnitIsInZone(Zone(0, 0, 1))));
    world.fragments.add(Fragment::new("u1", "Zone(3, 2, 1)", "UnitIsInZone", UnitIsInZone(Zone(3, 2, 1))));
    world.fragments.add(Fragment::new("u2", "Zone(-4, 2, 1)", "UnitIsInZone", UnitIsInZone(Zone(-4, 2, 1))));
    world.fragments.add(Fragment::new("player", "tomahawk", "UnitOwns", UnitOwns(14)));
    world.fragments.add(Fragment::new("player", "hp", "UnitHasAttribute", UnitHasAttribute(38.5)));
    world
}

fn main() -> Result<(), PlatformError> {
    println!("Hello, world!");
    //window::do_a_window()

    let world = make_sample_world();
    world_view::do_a_window(world)
}
