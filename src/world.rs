
use std::cmp::Reverse;
use std::collections::hash_map::Values;
use std::collections::{BinaryHeap, HashMap};
use std::rc::Rc;
use std::slice::Iter;

use rand::Rng;

use crate::fragment::{Fragments, Zone, Fragment, Shard::*, IdType, UnitId};
use crate::entry::Entry;
use crate::task::{Task, BeAt, BeAvatar};

type Timestamp = u128;
pub type TimeDiff = u128;

pub struct World {
    pub fragments: Fragments,
    pub next_moves: BinaryHeap<(Reverse<Timestamp>, UnitId)>,
    pub queued_move: (i64, i64),
    pub tasks: HashMap<UnitId, Box<dyn Task>>,
}

impl World {
    pub fn new() -> Self {
        World {
            fragments: Fragments::new(),
            next_moves: BinaryHeap::new(),
            queued_move: (0, 0),
            tasks: HashMap::new(),
        }
    }

    pub fn next_tick(&self) -> Timestamp {
        self.next_moves.peek().map(|entry| entry.0.0).unwrap_or(0)
    }

    pub fn next_unit<'a>(&'a self) -> &'a UnitId {
        self.next_moves.peek().map(|entry| &entry.1).unwrap()
    }

    pub fn advance(&mut self) {
        if let Some((Reverse(mut next_tick), mut unit_id)) = self.next_moves.pop() {
            let mut task = self.tasks.remove(&unit_id);

            if let Some(inner) = &task {
                if inner.is_done(self, &unit_id) {
                    task = None;
                }
            }

            let task = if let Some(inner) = task {
                inner
            } else {
                let mut rng = rand::thread_rng();
                let x = rng.gen_range(-10..10);
                let y = rng.gen_range(-10..10);
                Box::new(BeAt(Zone(x, y, 1)))
            };

            next_tick += task.advance(self, &unit_id);
            self.tasks.insert(unit_id.clone(), task);
            self.next_moves.push((Reverse(next_tick), unit_id));
        }
    }

    pub fn add_unit(&mut self, id: &str, zone: Zone) {
        if id == "player" {
            self.tasks.insert(String::from("player"), Box::new(BeAvatar {}));
        }
        self.fragments.add(Rc::new(Fragment::new(IdType::from(id), IdType::from(zone), "UnitIsInZone", UnitIsInZone(zone))));
        self.next_moves.push((Reverse(self.next_tick()), String::from(id)));
    }

    pub fn get_fragments<'a>(&'a self, shard_name: &'static str, first_id: &IdType) -> Iter<'a, Rc<Fragment>> {
        self.fragments.get(shard_name, first_id)
    }

    pub fn move_unit(&mut self, unit_id: &UnitId, x: i64, y: i64) -> TimeDiff {
        let fragment = self.fragments
            .get("UnitIsInZone", &IdType::from(unit_id))
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