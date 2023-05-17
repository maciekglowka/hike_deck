use std::collections::HashSet;

use crate::vectors::Vector2Int;

use super::room::Room;

pub struct Area {
    pub rooms: Vec<Room>,
    pub paths: Vec<Vec<Vector2Int>>
}
impl Area {
    pub fn new() -> Self {
        Area { rooms: Vec::new(), paths: Vec::new() }
    }
    pub fn get_bounds(&self) -> (Vector2Int, Vector2Int) {
        let min_x = self.rooms.iter().map(|r| r.a.x).min().unwrap();
        let max_x = self.rooms.iter().map(|r| r.b.x).max().unwrap();
        let min_y = self.rooms.iter().map(|r| r.a.y).min().unwrap();
        let max_y = self.rooms.iter().map(|r| r.b.y).max().unwrap();
        (Vector2Int::new(min_x, min_y), Vector2Int::new(max_x, max_y))
    }
    pub fn get_size(&self) -> Vector2Int {
        let bounds = self.get_bounds();
        Vector2Int::new(bounds.1.x - bounds.0.x + 1, bounds.1.y - bounds.0.y + 1)
    }
    pub fn shift(&mut self, offset: Vector2Int) {
        // translate the entire area by a given offset
        let bounds = self.get_bounds();
        let d = offset - bounds.0;

        for room in self.rooms.iter_mut() {
            room.a += d;
            room.b += d;
        }
        for path in self.paths.iter_mut() {
            for v in path.iter_mut() {
                *v += d;
            }
        }
    }
    pub fn generate_rooms(&mut self) {
        self.rooms = vec![
            Room::new(Vector2Int::new(0, 0), Vector2Int::new(4, 6)),
            Room::new(Vector2Int::new(6, 2), Vector2Int::new(12, 8))
        ]
    }
    pub fn to_tiles(&self) -> HashSet<Vector2Int> {
        self.rooms.iter()
            .map(|r| r.to_tiles())
            .flatten()
            .chain(
                self.paths.iter().flatten().map(|v| *v)
            )
            .collect()
    }
}