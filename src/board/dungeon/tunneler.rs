use rand::{
    distributions::WeightedIndex,
    prelude::*
};

use crate::vectors::Vector2Int;

pub trait Tunneler {
    fn connect(&self, a: Vector2Int, b: Vector2Int) -> Vec<Vector2Int>;
}

pub struct LShapeTunneler;
impl Tunneler for LShapeTunneler {
    // connects two points by forming an L shaped connection
    // initial direction (hor / ver) is the one whith the biggest coordinate difference
    fn connect(&self, a: Vector2Int, b: Vector2Int) -> Vec<Vector2Int> {
        let d = b - a;
        let (hor_y, ver_x) = match d.x > d.y {
            true => (a.y, b.x),
            false => (b.y, a.x)
        };

        let hor = (a.x.min(b.x)..=a.x.max(b.x))
            .map(|x| Vector2Int::new(x, hor_y))
            .collect::<Vec<_>>();
        let ver = (a.y.min(b.y)..=a.y.max(b.y))
            .map(|y| Vector2Int::new(ver_x, y))
            .collect::<Vec<_>>();
        [ver, hor].concat()
    }
}

pub struct RandomTunneler;
impl Tunneler for RandomTunneler {
    // connects two points by taking a random direction (hor / ver) towards the target
    // choice chance is determined by a current coordinate difference
    // (it is most likely to pick a dir with the biggest diff)
    fn connect(&self, a: Vector2Int, b: Vector2Int) -> Vec<Vector2Int> {
        let mut cur = a;
        let mut path = Vec::new();
        let mut rng = thread_rng();
    
        while cur != b {
            path.push(cur);
            // 0 is horizontal, 1 is vertical
            let dirs = vec![b.x - cur.x, b.y - cur.y];
            // build weights
            let dist = WeightedIndex::new(dirs.iter().map(|d| d.abs())).unwrap();
            // pick a dir idx (0 or 1)
            let dir_idx = dist.sample(&mut rng);
            // create a normalized step vector in a single direction
            let dv = match dir_idx {
                0 => Vector2Int::new(dirs[0] / dirs[0].abs(), 0),
                1 => Vector2Int::new(0, dirs[1] / dirs[1].abs()),
                _ => panic!()
            };
            cur += dv;
        }
        path
    }
}