
use crate::fragment::{Fragment, Id};
use crate::squares::{Zone, ZoneId};
use std::collections::HashMap;
use std::vec::Vec;

pub struct World {
    truths: HashMap<Id, Vec<Fragment>>,
    pub zones: HashMap<ZoneId, Zone>,
}

impl World {
    pub fn get_truths(&self, id: &str) -> &Vec<Fragment> {
        self.truths.get(&String::from(id)).expect(&format!("no truths exist for {}", id))
    }

    pub fn new() -> Self {
        World {
            truths: HashMap::new(),
            zones: HashMap::new(),
        }
    }

    pub fn add_fragment(&mut self, fragment: Fragment) {
        match fragment {
            Fragment::UnitZone(uid, zid) => todo!(),
            Fragment::UnitHolds(_, _, _) => todo!(),
            Fragment::UnitEquips(_, _, _) => todo!(),
            Fragment::UnitHasAttribute(_, _, _) => todo!(),
        }
    }
}