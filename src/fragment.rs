use std::{collections::{HashMap, hash_map::Values}, string::String, hash::Hash, rc::Rc};

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
    fragments: HashMap<Id, HashMap<&'static str, HashMap<Id, Rc<Fragment>>>>,
    empty_subfragment: HashMap<Id, Rc<Fragment>>,
}

impl Fragments {
    pub fn check(&self, fragment: &Fragment) -> (bool, bool) {
        let a = self.fragments.get(&fragment.a)
        .and_then(|p| p.get(fragment.shard_name))
        .and_then(|q| q.get(&fragment.b))
        .map(|f| f.as_ref() == fragment)
        .unwrap_or(false);

        let b = self.fragments.get(&fragment.b)
        .and_then(|p| p.get(fragment.shard_name))
        .and_then(|q| q.get(&fragment.a))
        .map(|f| f.as_ref() == fragment)
        .unwrap_or(false);

        (a, b)
    }

    pub fn add(&mut self, fragment: Fragment) {
        assert_eq!(self.check(&fragment), (false, false), "can't re-add {:?}", fragment);

        let fragment = Rc::new(fragment);
        self.fragments.entry(fragment.a.clone())
            .or_default()
            .entry(fragment.shard_name)
            .or_default()
            .insert(fragment.b.clone(), Rc::clone(&fragment));
        self.fragments.entry(fragment.b.clone())
            .or_default()
            .entry(fragment.shard_name)
            .or_default()
            .insert(fragment.a.clone(), fragment);
    }

    pub fn remove(&mut self, fragment: &Fragment) {
        assert_eq!(self.check(&fragment), (true, true), "can't re-remove {:?}", fragment);
        self.fragments.get_mut(&fragment.a).unwrap().get_mut(fragment.shard_name).unwrap().remove(&fragment.b);
        self.fragments.get_mut(&fragment.b).unwrap().get_mut(fragment.shard_name).unwrap().remove(&fragment.a);
    }

    pub fn new() -> Self {
        Fragments {
            fragments: HashMap::new(),
            empty_subfragment: HashMap::new(),
        }
    }

    pub fn get_all<'a>(&'a self, id: &str, shard_name: &str) -> Values<'a, Id, Rc<Fragment>> {
        self.fragments.get(id)
            .and_then(|p| p.get(shard_name))
            .map(|q| q.values())
            .unwrap_or(self.empty_subfragment.values())
    }
}

#[derive(Debug,PartialEq,Clone)]
pub struct Fragment {
    pub a: Id,
    pub b: Id,
    pub shard_name: &'static str,
    pub shard: Shard,
}

impl Fragment {
    pub fn new(a: &str, b: &str, shard_name: &'static str, shard: Shard) -> Self {
        Fragment {
            a: String::from(a),
            b: String::from(b),
            shard_name,
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
