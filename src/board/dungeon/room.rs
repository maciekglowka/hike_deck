use rand::prelude::*;
use std::collections::HashSet;

use crate::vectors::Vector2Int;

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
    pub fn centre(&self) -> Vector2Int {
        Vector2Int::new((self.b.x+self.a.x) / 2, (self.b.y+self.a.y) / 2)
    }
    pub fn intersects(&self, other: &Room, border: Option<u32>) -> bool {
        let b = match border {
            Some(a) => a as i32,
            None => 0
        };
        !(
            other.a.x > self.b.x + b ||
            other.b.x < self.a.x - b ||
            other.a.y > self.b.y + b ||
            other.b.y < self.a.y - b
        )
    }
}

pub trait RoomGenerator {
    fn generate(&self) -> GeneratorResult;
}

pub struct GeneratorResult {
    pub rooms: Vec<Room>,
    // connections are defined by room indices
    pub connections: Vec<(usize, usize)>
}

pub struct BubbleGenerator {
    // bounds for a random room count
    pub room_count: (u32, u32),
    // min max room size
    pub room_size: (u32, u32),
    // min distance between rooms
    pub room_padding: Option<u32>,
    pub extra_connection_chance: f64
}
impl BubbleGenerator {
    fn random_dim(&self) -> (i32, i32) {
        let mut rng = thread_rng();
        (
            rng.gen_range(self.room_size.0..=self.room_size.1) as i32,
            rng.gen_range(self.room_size.0..=self.room_size.1) as i32
        )
    }
}
impl RoomGenerator for BubbleGenerator {
    fn generate(&self) -> GeneratorResult {
        let mut rng = thread_rng();
        let mut connections = Vec::new();

        let (w, h) = self.random_dim();
        let mut rooms = vec![
            Room::new(
                Vector2Int::new(0, 0),
                Vector2Int::new(w, h)
            )
        ];
        // helper value for random point bounds
        let max_dist = self.room_size.1 as i32;

        let count = rng.gen_range(self.room_count.0..=self.room_count.1);
        for _ in 0..=count {
            loop {
                // take a random existing room as a base
                let prev_idx = rng.gen_range(0..rooms.len());
                // pick a random point around prev's centre
                let centre = rooms[prev_idx].centre();
                let a = Vector2Int::new(
                    rng.gen_range(centre.x - max_dist..=centre.x + max_dist),
                    rng.gen_range(centre.y - max_dist..=centre.y + max_dist)
                );
              
                // get random room size
                let (w, h) = self.random_dim();
                // get a second corner in a random direction
                let b = Vector2Int::new(
                    a.x + *[-w, w].choose(&mut rng).unwrap(),
                    a.y + *[-h, h].choose(&mut rng).unwrap()
                );
                
                let r = Room::new(a, b);
                // check for overlaps with the other rooms
                if rooms.iter().any(|other| r.intersects(other, self.room_padding)) {
                    continue;
                }
                connections.push((prev_idx, rooms.len()));

                // try creating a second connection
                if rng.gen_bool(self.extra_connection_chance) {
                    connections.push((rng.gen_range(0..rooms.len()), rooms.len()));
                }
                rooms.push(Room::new(a, b));
                // if the room is valid, we can break the randomize loop
                // and move to the next one
                break;
            }
        }
        GeneratorResult { rooms, connections }
    }
}