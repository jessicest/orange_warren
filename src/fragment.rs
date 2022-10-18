use std::{collections::{HashMap, hash_map::Values}, string::String, hash::Hash, rc::Rc, borrow::Borrow, slice::Iter};

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
    paired: HashMap<(&'static str, IdType, IdType), Rc<Fragment>>,
    a_to_b: HashMap<(&'static str, IdType), Vec<Rc<Fragment>>>,
    a_to_all: HashMap<IdType, Vec<Rc<Fragment>>>,
    empty_vec: Vec<Rc<Fragment>>,
}

impl Fragments {
    pub fn get_one<'a>(&'a self, shard_name: &'static str, a: &IdType, b: &IdType) -> Option<&'a Rc<Fragment>> {
        self.paired.get(&(shard_name, a.clone(), b.clone()))
    }

    pub fn get<'a>(&'a self, shard_name: &'static str, id: &IdType) -> Iter<'a, Rc<Fragment>> {
        self.a_to_b.get(&(shard_name, id.clone()))
            .unwrap_or(&self.empty_vec)
            .iter()
    }

    pub fn get_all(&self, id: &IdType) -> Iter<Rc<Fragment>> {
        self.a_to_all.get(id)
            .unwrap_or(&self.empty_vec)
            .iter()
    }

    pub fn add(&mut self, fragment: Rc<Fragment>) {
        self.paired.insert((fragment.shard_name, fragment.a.clone(), fragment.b.clone()), fragment.clone());
        self.a_to_b.entry((fragment.shard_name, fragment.a.clone()))
            .or_insert(Vec::new())
            .push(fragment.clone());
        self.a_to_b.entry((fragment.shard_name, fragment.b.clone()))
            .or_insert(Vec::new())
            .push(fragment.clone());
        self.a_to_all.entry(fragment.a.clone())
            .or_insert(Vec::new())
            .push(fragment.clone());
        self.a_to_all.entry(fragment.b.clone())
            .or_insert(Vec::new())
            .push(fragment.clone());
    }

    pub fn remove(&mut self, fragment: &Rc<Fragment>) {
        self.paired.remove(&(fragment.shard_name, fragment.a.clone(), fragment.b.clone()));

        if let Some(v) = self.a_to_b.get_mut(&(fragment.shard_name, fragment.a.clone())) {
            v.retain(|f| f != fragment);
        }

        if let Some(v) = self.a_to_b.get_mut(&(fragment.shard_name, fragment.a.clone())) {
            v.retain(|f| f != fragment);
        }

        if let Some(v) = self.a_to_b.get_mut(&(fragment.shard_name, fragment.b.clone())) {
            v.retain(|f| f != fragment);
        }

        if let Some(v) = self.a_to_all.get_mut(&fragment.a) {
            v.retain(|f| f != fragment);
        }

        if let Some(v) = self.a_to_all.get_mut(&fragment.b) {
            v.retain(|f| f != fragment);
        }
    }

    pub fn new() -> Self {
        Fragments {
            a_to_b: HashMap::new(),
            paired: HashMap::new(),
            a_to_all: HashMap::new(),
            empty_vec: Vec::new(),
        }
    }
}

#[derive(Debug,PartialEq,Clone)]
pub struct Fragment {
    pub shard_name: &'static str,
    pub a: IdType,
    pub b: IdType,
    pub shard: Shard,
}

impl Fragment {
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