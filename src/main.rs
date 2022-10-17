use std::rc::Rc;

use druid::PlatformError;
use fragment::{Zone, Fragment, Shard::{*, self}, IdType};

use crate::world::World;

mod decider;
mod entry;
mod fragment;
mod task;
mod world;
mod world_actions;
mod world_view;

fn add_fragment(world: &mut World, a: &str, b: &str, shard_name: &'static str, shard: Shard) {
    world.fragments.add(Rc::new(Fragment::new_str(a, b, shard_name, shard)));
}

fn make_sample_world() -> World {
    let mut world = World::new();
    world.add_unit(IdType::from("player"), Zone(0, 0, 1));
    add_fragment(&mut world, "player", "tomahawk", "UnitOwns", UnitOwns(34));
    add_fragment(&mut world, "player", "hp", "UnitHasAttribute", UnitHasAttribute(38.5));
    world.add_unit(IdType::from("u1"), Zone(3, 2, 1));
    world.add_unit(IdType::from("u2"), Zone(-4, 2, 1));
    world
}

fn main() -> Result<(), PlatformError> {
    println!("Hello, world!");
    //window::do_a_window()

    let world = make_sample_world();
    world_view::do_a_window(world)
}
