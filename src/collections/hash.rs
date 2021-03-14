/*//! Hash trait implementations for f32, f64

use std::hash::{Hash, Hasher};

impl Hash for f32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self as u32).hash(state)
    }
}

impl Hash for f64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self as u32).hash(state)
    }
}*/