use crate::{fragment::{Zone, IdType}, world::{World, TimeDiff}, world_actions};

use std::fmt::Debug;


pub trait Task: Debug {
    fn is_done(&self, world: &World, unit_id: &IdType) -> bool;
    fn advance(&self, world: &mut World, unit_id: &IdType) -> TimeDiff;
}

#[derive(Debug)]
pub struct BeAvatar {
}

impl Task for BeAvatar {
    fn is_done(&self, _: &World, _: &IdType) -> bool {
        false
    }

    fn advance(&self, world: &mut World, unit_id: &IdType) -> TimeDiff {
        let (x, y) = world.queued_move;
        world.move_unit(unit_id, x, y)
    }
}

#[derive(Debug)]
pub struct BeAt(pub Zone);

impl <'a> BeAt {
    pub fn target(&'a self) -> &'a Zone {
        &self.0
    }
}

impl Task for BeAt {
    fn is_done(&self, world: &World, unit_id: &IdType) -> bool {
        for fragment in world.fragments.get(unit_id, "UnitIsInZone") {
            if let IdType::Zone(zone) = fragment.b {
                if self.target().contains(&zone) {
                    return true;
                }
            }
        }
        false
    }

    fn advance(&self, world: &mut World, unit_id: &IdType) -> TimeDiff {
        let zone = world.get_fragments(unit_id, "UnitIsInZone")
            .map(|fragment| if let IdType::Zone(zone) = &fragment.b { zone } else { panic!("bogus zone")})
            .next()
            .unwrap();

        let x = match self.target().0.cmp(&zone.0) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        };
        let y = match self.target().1.cmp(&zone.1) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        };
        world.move_unit(unit_id, x, y)
    }
}