use std::rc::Rc;

use druid::PlatformError;
use fragment::{Zone, Fragment, Shard::{*, self}, IdType};
use rand::Rng;

use crate::world::World;

mod decider;
mod entry;
mod fragment;
mod task;
mod world;
mod world_actions;
mod world_view;

fn afss(world: &mut World, a: &str, b: &str, shard_name: &'static str, shard: Shard) {
    world.fragments.add(Rc::new(Fragment::new(IdType::from(a), IdType::from(b), shard_name, shard)));
}

fn afsz(world: &mut World, a: &str, b: Zone, shard_name: &'static str, shard: Shard) {
    world.fragments.add(Rc::new(Fragment::new(IdType::from(a), IdType::from(b), shard_name, shard)));
}

fn make_sample_world() -> World {
    let mut rng = rand::thread_rng();

    let mut world = World::new();
    world.add_unit("player", Zone(0, 0, 1));
    afss(&mut world, "player", "tomahawk", "UnitOwns", UnitOwns(34));
    afss(&mut world, "player", "hp", "UnitHasAttribute", UnitHasAttribute(38.5));

    world.add_unit("u1", Zone(3, 2, 1));
    world.add_unit("u2", Zone(-4, 2, 1));

    for _ in 0..217 {
        let x = rng.gen_range(10..100);
        let y = rng.gen_range(-10..10);
        let zone = Zone(x, y, 1);

        if world.fragments.get_one("ObjectTypeOccupiesZone", &IdType::from("tree"), &IdType::from(zone)).is_none() {
            afsz(&mut world, "tree", zone, "ObjectTypeOccupiesZone", ObjectTypeOccupiesZone(zone));
        }
    }
    world
}

fn main() -> Result<(), PlatformError> {
    println!("Hello, world!");
    //window::do_a_window()

    let world = make_sample_world();
    world_view::do_a_window(world)
}
