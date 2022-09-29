
use crate::fragment::{Fragments};

pub struct World {
    pub fragments: Fragments,
}

impl World {
    pub fn new() -> Self {
        World {
            fragments: Fragments::new(),
        }
    }
}