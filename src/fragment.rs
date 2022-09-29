
use crate::squares::ZoneId;

pub type Id = String;
pub type UnitId = Id;
pub type ItemType = Id;
pub type AttributeId = Id;

pub enum Fragment {
    UnitZone(UnitId, ZoneId),
    UnitHolds(UnitId, ItemType, usize),
    UnitEquips(UnitId, ItemType, usize),
    UnitHasAttribute(UnitId, AttributeId, f64),
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

