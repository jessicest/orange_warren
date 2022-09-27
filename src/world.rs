
use crate::fragment::Id;
use std::collections::HashMap;
use std::vec::Vec;

struct World {
    truth: HashMap<Id, Vec<Fragment>>,
}

enum Relation {
    IsAtLocation,
    Holds(usize),
    Equips(usize),
    HasAttribute(f64),
}


pub struct Fragment {
    actor: Id,
    relation: Relation,
    target: Id,
}

//  ItemPosition(ZoneId, ItemType, f64),
//  UnitPosition(ZoneId, UnitId),
//  UnitPossession(UnitId, ItemType, f64),

// a unit:
//   what items do they possess?
//   what zone are they in?
// a zone:
//   what items are on it?
//   what people are in it?
// an item type:
//   where is it?
//   which people have it?

