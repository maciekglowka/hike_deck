use rand::prelude::*;
use std::collections::HashSet;

use crate::vectors::Vector2Int;

use bevy::prelude::*;

pub struct Room {
    pub a: Vector2Int,
    pub b: Vector2Int
}
impl Room {
    pub fn new(a: Vector2Int, b: Vector2Int) -> Self {
        // sort the coordinates so `a` is always the left-bottom
        // and `b` is top-right
        Room {
            a: Vector2Int::new(a.x.min(b.x), a.y.min(b.y)),
            b: Vector2Int::new(a.x.max(b.x), a.y.max(b.y))
        }
    }
    pub fn corners(&self) -> [Vector2Int; 4] {
        [
            Vector2Int::new(self.a.x, self.a.y), Vector2Int::new(self.b.x, self.a.y),
            Vector2Int::new(self.b.x, self.b.y), Vector2Int::new(self.a.x, self.b.y)
        ]
    }
    pub fn random_point(&self) -> Vector2Int {
        let mut rng = thread_rng();
        let x = rng.gen_range(self.a.x..=self.b.x);
        let y = rng.gen_range(self.a.y..=self.b.y);
        Vector2Int::new(x, y)
    }
    pub fn to_tiles(&self) -> HashSet<Vector2Int> {
        (self.a.y..=self.b.y).map(|y| {
            (self.a.x..=self.b.x).map(move |x| {
                Vector2Int::new(x, y)
            })
        })
        .flatten()
        .collect()
    }
}