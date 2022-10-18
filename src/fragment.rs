use std::{collections::{HashMap, hash_map::Values}, string::String, hash::Hash, rc::Rc, borrow::Borrow};

use derive_more::From;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Zone(pub i64, pub i64, pub u64);

impl Zone {
    pub fn adjust(&self, x: i64, y: i64) -> Self {
        Zone(self.0 + x, self.1 + y, self.2)
    }

    pub fn contains(&self, inner: &Zone) -> bool {
        self.0 - self.2 as i64 <= inner.0 - inner.2 as i64
        && self.0 + self.2 as i64 >= inner.0 + inner.2 as i64
        && self.1 - self.2 as i64 <= inner.1 - inner.2 as i64
        && self.1 + self.2 as i64 >= inner.1 + inner.2 as i64
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, From)]
pub enum IdType {
    String(String),
    Zone(Zone),
}
impl IdType {
    pub fn to_string(&self) -> String {
        match self {
            IdType::String(s) => s.clone(),
            IdType::Zone(zone) => format!("{:?}", zone),
        }
    }
}

impl From<&str> for IdType {
    fn from(s: &str) -> Self {
        Self::from(String::from(s))
    }
}

impl From<&String> for IdType {
    fn from(s: &String) -> Self {
        Self::from(s.clone())
    }
}

pub type Id = String;
pub type UnitId = Id;
pub type ItemId = Id;
pub type AttributeId = Id;
pub type ZoneId = Id;

pub struct Fragments {
    fragments: HashMap<IdType, HashMap<&'static str, HashMap<IdType, Rc<Fragment>>>>,
    empty_subfragment: HashMap<IdType, Rc<Fragment>>,
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

    pub fn add(&mut self, fragment: Rc<Fragment>) {
        assert_eq!(self.check(&fragment), (false, false), "can't re-add {:?}", fragment);

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

    pub fn get_precise<'a>(&'a self, a: &IdType, shard_name: &str, b: &IdType) -> Option<Rc<Fragment>> {
        self.fragments.get(a)
            .and_then(|p| p.get(shard_name))
            .and_then(|p| p.get(b).cloned())
    }

    pub fn get<'a>(&'a self, id: &IdType, shard_name: &str) -> Values<'a, IdType, Rc<Fragment>> {
        self.fragments.get(id)
            .and_then(|p| p.get(shard_name))
            .map(|q| q.values())
            .unwrap_or(self.empty_subfragment.values())
    }

    pub fn get_all<'a>(&'a self, id: &IdType) -> Vec<&Rc<Fragment>> {
        self.fragments.get(id)
            .map(|p| {
                p.values()
                .map(|q| q.values())
                .flatten()
                .collect::<Vec<_>>()
            })
            .unwrap_or(vec![])
    }
}

#[derive(Debug,PartialEq,Clone)]
pub struct Fragment {
    pub a: IdType,
    pub b: IdType,
    pub shard_name: &'static str,
    pub shard: Shard,
}

impl Fragment {
    pub fn new_str(a: &str, b: &str, shard_name: &'static str, shard: Shard) -> Self {
        Self::new(IdType::from(a), IdType::from(b), shard_name, shard)
    }

    pub fn new(a: IdType, b: IdType, shard_name: &'static str, shard: Shard) -> Self {
        Fragment {
            a,
            b,
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
    ItemTypeIsInZone(Zone, usize),
    ObjectTypeOccupiesZone(Zone),
}