use druid::PlatformError;
use fragment::{Zone, Fragment, IdType};

use crate::world::World;

mod fragment;
mod squares;
mod window;
mod world;
mod world_view;
mod item;

    //UnitZone(UnitId, ZoneId),
    //UnitHolds(UnitId, ItemType, usize),
    //UnitEquips(UnitId, ItemType, usize),
    //UnitHasAttribute(UnitId, AttributeId, f64),
fn make_sample_world() -> World {
    let mut world = World::new();
    //world.fragments.add(Fragment::new(IdType::from(String::from("u0")), IdType::from(Zone(0, 0, 1)), fragment::Shard::UnitIsInZone()));
    world.fragments.add(Fragment::new("u0", "z0,0#1", fragment::Shard::UnitIsInZone()));
    world
}

fn main() -> Result<(), PlatformError> {
    println!("Hello, world!");
    //window::do_a_window()

    let mut world = World::new();
    world_view::do_a_window(world)
}
