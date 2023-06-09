use std::collections::HashSet;

use crate::vectors::Vector2Int;

use super::room::{Room, RoomGenerator};
use super::tunneler::Tunneler;

pub struct Area {
    pub rooms: Vec<Room>,
    pub paths: Vec<Vec<Vector2Int>>,
    pub tunneler: Box<dyn Tunneler>,
    pub room_generator: Box<dyn RoomGenerator>
}
impl Area {
    pub fn new(tunneler: Box<dyn Tunneler>, room_generator: Box<dyn RoomGenerator>) -> Self {
        Area { rooms: Vec::new(), paths: Vec::new(), tunneler, room_generator }
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
    pub fn join_rooms(&self, a: &Room, b: &Room) -> Vec<Vector2Int> {
        self.tunneler.connect(a.random_point(), b.random_point())
    }
    pub fn generate_rooms(&mut self) {
        let result = self.room_generator.generate();
        self.rooms = result.rooms;
        self.paths = result.connections.iter()
            .map(|a| self.join_rooms(&self.rooms[a.0], &self.rooms[a.1]))
            .collect();
    }
    fn find_closest_room_pair<'a>(&'a self, other: &'a Area) -> (&'a Room, &'a Room) {
        // find closest room pair between two areas
        // based on corner distances only
        let mut pairs = Vec::new();
        for ra in self.rooms.iter() {
            for rb in other.rooms.iter() {
                // find min corner dist
                let d= ra.corners().iter()
                    .map(|ca| rb.corners().iter()
                        .map(|cb| ca.manhattan(*cb))
                        .collect::<Vec<_>>()
                    )
                    .flatten()
                    .min()
                    .unwrap();
                pairs.push((d, ra, rb));
            }
        }
        // sort by corner dist
        pairs.sort_by(|a, b| a.0.cmp(&b.0));
        (pairs[0].1, pairs[0].2)
    }
    pub fn join_area(&self, other: &Area) -> Vec<Vector2Int> {
        let rooms = self.find_closest_room_pair(other);
        self.join_rooms(rooms.0, rooms.1)
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
