
pub type Id = String;

pub enum Fragment {
    IntFragment(Id, Id, usize),
    FloatFragment(Id, Id, f64),
    VoidFragment(Id, Id),
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

