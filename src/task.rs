use crate::{fragment::{Zone, IdType}, world::World};


pub trait Task {
    fn is_done(&self, world: &World, unit_id: &IdType) -> bool;
}

pub struct Decide {
}

impl Task for Decide {
    fn is_done(&self, world: &World, unit_id: &IdType) -> bool {
        false
    }
}

pub struct BeAt(pub Zone);

impl Task for BeAt {
    fn is_done(&self, world: &World, unit_id: &IdType) -> bool {
        for fragment in world.fragments.get(unit_id, "UnitIsInZone") {
            if let IdType::Zone(inner) = fragment.b {
                if self.0.contains(&inner) {
                    return true;
                }
            }
        }
        false
    }
}