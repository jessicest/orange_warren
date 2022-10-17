
use std::collections::hash_map::Values;
use std::collections::{BinaryHeap};
use std::rc::Rc;

use crate::fragment::{Fragments, Zone, Fragment, Shard::*, IdType};
use crate::entry::Entry;
use crate::task::Task;

type Timestamp = u128;

struct Unit {
    pub id: IdType,
    pub task: Option<Box<dyn Task>>,
    pub next_tick: Timestamp,
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
        self.units.peek().map(|entry| entry.0).unwrap_or(0)
    }

    pub fn tick(&mut self) {
        if let Some(Entry(_, unit)) = self.units.pop() {
            todo!("interact with the task");
            self.units.push(Entry(unit.next_tick, unit));
        }
    }

    pub fn add_unit(&mut self, id: IdType, zone: Zone) {
        let unit = Unit {
            id: id.clone(),
            task: None,
            next_tick: self.next_tick(),
        };
        self.fragments.add(Rc::new(Fragment::new(id, IdType::from(zone), "UnitIsInZone", UnitIsInZone(zone))));
        self.units.push(Entry(unit.next_tick, unit));
    }

    pub fn get_fragments<'a>(&'a self, first_id: &IdType, shard_name: &str) -> Values<'a, IdType, Rc<Fragment>> {
        self.fragments.get(first_id, shard_name)
    }
}