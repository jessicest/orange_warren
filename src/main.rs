use druid::PlatformError;
use squares::Pos;

use crate::world::World;
use crate::fragment::Fragment::*;

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
    world.add_fragment(UnitZone(String::from("u0"), Pos::new(0, 0).to_zone_id()));
    world
}

fn main() -> Result<(), PlatformError> {
    println!("Hello, world!");
    //window::do_a_window()

    let mut world = World::new();
    world_view::do_a_window(world)
}
