
use std::collections::{HashMap, BinaryHeap};

use crate::fragment::{Fragments, UnitId, Zone, Fragment, Shard::*};
use crate::decider::Decider;
use crate::entry::Entry;

type Timestamp = u128;

struct Unit {
    id: UnitId,
    decider: Decider,
    next_tick: Timestamp,
}

pub struct World {
    pub fragments: Fragments,
    units: BinaryHeap<Entry<Timestamp, Unit>>,
}

impl World {
    pub fn new() -> Self {
        World {
            fragments: Fragments::new(),
            units: BinaryHeap::new(),
        }
    }

    pub fn next_tick(&self) -> Timestamp {
        self.units.peek().map(|entry| entry.key).unwrap_or(0)
    }

    pub fn tick(&mut self) {
        todo!();
    }

    pub fn add_unit(&mut self, id: &str, zone: Zone) {
        let unit = Unit {
            id: String::from(id),
            decider: Decider::new(),
            next_tick: self.next_tick(),
        };
        self.fragments.add(Fragment::new(id, &format!("{:#?}", zone), "UnitIsInZone", UnitIsInZone(zone)));
    }
}