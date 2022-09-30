use std::{collections::{HashMap, hash_map::Values}, string::String, option::Iter, hash::Hash};

use derive_more::From;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Zone(pub i64, pub i64, pub u64);

impl Zone {
    pub fn adjust(&self, x: i64, y: i64) -> Self {
        Zone(self.0 + x, self.1 + y, self.2)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, From)]
pub enum IdType {
    String(String),
    Zone(Zone),
}

pub type Id = String;
pub type UnitId = Id;
pub type ItemId = Id;
pub type AttributeId = Id;
pub type ZoneId = Id;

pub struct Fragments {
    fragments: HashMap<Id, HashMap<Id, Fragment>>,
    empty_subfragment: HashMap<Id, Fragment>,
}

impl Fragments {
    pub fn check(&self, fragment: &Fragment) -> (bool, bool) {
        let a = self.fragments.get(&fragment.a)
        .and_then(|subfragments| subfragments.get(&fragment.b))
        .map(|f| f == fragment)
        .unwrap_or(false);

        let b = self.fragments.get(&fragment.b)
        .and_then(|subfragments| subfragments.get(&fragment.a))
        .map(|f| f == fragment)
        .unwrap_or(false);

        (a, b)
    }

    pub fn add(&mut self, fragment: Fragment) {
        assert_eq!(self.check(&fragment), (false, false), "can't re-add {:?}", fragment);

        // TODO: if we make all the ID keys in the hashmap references into the fragment itself, they no longer need String and
        // thus we can cut all these clones away
        self.fragments.entry(fragment.a.clone())
            .or_default()
            .insert(fragment.b.clone(), fragment.clone());
        self.fragments.entry(fragment.b.clone())
            .or_default()
            .insert(fragment.a.clone(), fragment.clone());
    }

    pub fn remove(&mut self, fragment: &Fragment) {
        assert_eq!(self.check(&fragment), (true, true), "can't re-remove {:?}", fragment);
        self.fragments.get_mut(&fragment.a).unwrap().remove(&fragment.b);
        self.fragments.get_mut(&fragment.b).unwrap().remove(&fragment.a);
    }

    pub fn new() -> Self {
        Fragments {
            fragments: HashMap::new(),
            empty_subfragment: HashMap::new(),
        }
    }

    pub fn get_all<'a>(&'a self, id: &str) -> Values<'a, Id, Fragment> {
        self.fragments.get(id)
            .map(|subfragments| subfragments.values())
            .unwrap_or(self.empty_subfragment.values())
    }
}

type Timestamp = String;

pub struct FragmentExpiry {
    fragment: Fragment,
    expiry: Timestamp,
}

#[derive(Debug,PartialEq,Clone)]
pub struct Fragment {
    pub a: Id,
    pub b: Id,
    pub shard: Shard,
}

impl Fragment {
    pub fn new(a: &str, b: &str, shard: Shard) -> Self {
        Fragment {
            a: String::from(a),
            b: String::from(b),
            shard,
        }
    }
}

#[derive(Debug,PartialEq,Clone)]
pub enum Shard {
    UnitIsInZone(Zone),
    UnitOwns(usize),
    UnitHasAttribute(f64),
    ItemIsInZone(Zone, usize),
}

// a unit:
//   what items do they possess?
//   what zone are they in?
// a zone:
//   what items are on it?
//   what people are in it?
// an item type:
//   where is it?
//   which people have it?
