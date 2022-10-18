
use std::cmp::Reverse;
use std::collections::hash_map::Values;
use std::collections::{BinaryHeap};
use std::rc::Rc;

use rand::Rng;

use crate::fragment::{Fragments, Zone, Fragment, Shard::*, IdType};
use crate::entry::Entry;
use crate::task::{Task, BeAt, BeAvatar};

type Timestamp = u128;
pub type TimeDiff = u128;

#[derive(Debug)]
struct Unit {
    pub id: IdType,
    pub task: Option<Box<dyn Task>>,
    pub next_tick: Timestamp,
}

pub struct World {
    pub fragments: Fragments,
    units: BinaryHeap<Entry<Reverse<Timestamp>, Unit>>,
    pub queued_move: (i64, i64),
}

impl World {
    pub fn new() -> Self {
        World {
            fragments: Fragments::new(),
            units: BinaryHeap::new(),
            queued_move: (0, 0),
        }
    }

    pub fn next_tick(&self) -> Timestamp {
        self.units.peek().map(|entry| entry.0.0).unwrap_or(0)
    }

    pub fn next_unit<'a>(&'a self) -> &'a IdType {
        self.units.peek().map(|entry| &entry.1.id).unwrap()
    }

    pub fn advance(&mut self) {
        if let Some(Entry(_, mut unit)) = self.units.pop() {
            if let Some(task) = &unit.task {
                if task.is_done(self, &unit.id) {
                    unit.task = None;
                } else {
                    unit.next_tick += task.advance(self, &unit.id);
                }
            } else {
                let mut rng = rand::thread_rng();
                let x = rng.gen_range(-10..10);
                let y = rng.gen_range(-10..10);
                unit.task = Some(Box::new(BeAt(Zone(x, y, 1))));
            }
            self.units.push(Entry(Reverse(unit.next_tick), unit));
        }
    }

    pub fn add_unit(&mut self, id: IdType, zone: Zone) {
        let mut unit = Unit {
            id: id.clone(),
            task: None,
            next_tick: self.next_tick(),
        };
        if id == IdType::from("player") {
            unit.task = Some(Box::new(BeAvatar {}));
        }
        self.fragments.add(Rc::new(Fragment::new(id, IdType::from(zone), "UnitIsInZone", UnitIsInZone(zone))));
        self.units.push(Entry(Reverse(unit.next_tick), unit));
    }

    pub fn get_fragments<'a>(&'a self, first_id: &IdType, shard_name: &str) -> Values<'a, IdType, Rc<Fragment>> {
        self.fragments.get(first_id, shard_name)
    }

    pub fn move_unit(&mut self, unit_id: &IdType, x: i64, y: i64) -> TimeDiff {
        let fragment = self.fragments
            .get(unit_id, "UnitIsInZone")
            .find(|f| matches!(f.shard, UnitIsInZone(_)))
            .expect("avatar should exist")
            .clone();
        self.fragments.remove(&fragment);
        if let UnitIsInZone(Zone(zx, zy, 1)) = fragment.shard {
            let zone = Zone(zx + x, zy + y, 1);
            let fragment = Fragment::new(
                fragment.a.clone(),
                IdType::from(zone.clone()),
                "UnitIsInZone",
                UnitIsInZone(zone));
            self.fragments.add(Rc::new(fragment));
        }
        1000
    }
}