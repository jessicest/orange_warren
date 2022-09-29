use std::collections::{HashMap, HashSet};


pub type UnitId = String;
pub type ItemId = String;
pub type AttributeId = String;
pub type ZoneId = (i64, i64, u64);

struct ZoneKnowledge {
    units: HashSet<UnitId>,
    treasure: HashSet<(ItemId, usize)>,
}

struct UnitKnowledge {
    zones: HashSet<ZoneId>,
    possessions: HashSet<(ItemId, usize)>,
    equipments: HashSet<(ItemId, usize)>,
    attributes: HashSet<(AttributeId, f64)>,
}

struct ItemKnowledge {
    owners: HashSet<(UnitId, usize)>,
    places: HashSet<(ZoneId, usize)>,
}

pub struct Knowledges {
    zone_knowledges: HashMap<ZoneId, ZoneKnowledge>,
    unit_knowledges: HashMap<UnitId, UnitKnowledge>,
    item_knowledges: HashMap<ItemId, ItemKnowledge>,
}

type Timestamp = String;

pub struct ExpiringKnowledge {
    fragment: Fragment,
    expiry: Timestamp,
}

pub enum Fragment {
    UnitIsInZone(UnitId, ZoneId),
}

impl Fragment {
    fn matches(&self, knowledges: &Knowledges) -> bool {
        match self {
            Fragment::UnitIsInZone(uid, zid) => {
                let a = knowledges.unit_knowledges
                    .get(uid)
                    .map(|unit_knowledge| unit_knowledge.zones)
                    .map(|zone| zone.contains(zid))
                    .unwrap_or(false);

                let b = knowledges.zone_knowledges
                    .get(zid)
                    .map(|zone_knowledge| zone_knowledge.units.iter().any(|u| u == uid))
                    .unwrap_or(false);

                a && b
            }
        }
    }

    fn remove(&self, knowledges: &mut Knowledges) -> bool {
        if self.matches(knowledges) {
            match self {
                Fragment::UnitIsInZone(uid, zid) => {
                    knowledges.unit_knowledges.get_mut(uid).unwrap().zone = None;
                    knowledges.zone_knowledges.get_mut(zid).unwrap().units.retain(|u| u != uid);
                },
            }
            true
        } else {
            false
        }
    }
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
