//! Cell pathfinding, modelled on Athena's `path.c` of the period:
//! 1. try the "easy" path: diagonal steps while both axes differ, then
//!    straight, stopping at the first blocked step;
//! 2. otherwise A* over 8 directions, cost 10 straight / 14 diagonal,
//!    heuristic 10 × Manhattan distance, diagonals need both orthogonal
//!    neighbours walkable.
//!
//! Tier: athena2003 reconstruction. The client runs its own path finder to
//! animate walks; agreement with it is a TODO (docs/ROADMAP.md).
//!
//! Directions (RO convention): 0 N(+y) 1 NW 2 W 3 SW 4 S 5 SE 6 E 7 NE.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

use ro_data::MapGrid;

pub const DIRS: [(i32, i32); 8] = [(0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1)];

/// Longest path the server accepts (Athena MAX_WALKPATH).
pub const MAX_WALKPATH: usize = 32;

pub fn dir_of(dx: i32, dy: i32) -> u8 {
    DIRS.iter().position(|&d| d == (dx.signum(), dy.signum())).unwrap_or(0) as u8
}

pub fn is_diagonal(dir: u8) -> bool {
    dir & 1 == 1
}

fn can_move(g: &MapGrid, x0: i32, y0: i32, x1: i32, y1: i32) -> bool {
    if !g.walkable(x0, y0) || !g.walkable(x1, y1) {
        return false;
    }
    if x0 == x1 || y0 == y1 {
        return true;
    }
    g.walkable(x0, y1) && g.walkable(x1, y0)
}

/// The "easy" path only: diagonal while both axes differ, then straight;
/// None if a step is blocked (Athena path_search with flag 1: monsters'
/// random walks take no detours).
pub fn find_easy(g: &MapGrid, x0: i32, y0: i32, x1: i32, y1: i32) -> Option<Vec<u8>> {
    if !g.walkable(x1, y1) {
        return None;
    }
    let (dx, dy) = ((x1 - x0).signum(), (y1 - y0).signum());
    let (mut x, mut y) = (x0, y0);
    let mut path = Vec::new();
    while (x, y) != (x1, y1) {
        let (sx, sy) = (if x != x1 { dx } else { 0 }, if y != y1 { dy } else { 0 });
        if path.len() >= MAX_WALKPATH || !can_move(g, x, y, x + sx, y + sy) {
            return None;
        }
        x += sx;
        y += sy;
        path.push(dir_of(sx, sy));
    }
    Some(path)
}

/// Path from (x0,y0) to (x1,y1) as a list of directions, or None.
pub fn find(g: &MapGrid, x0: i32, y0: i32, x1: i32, y1: i32) -> Option<Vec<u8>> {
    if !g.walkable(x1, y1) {
        return None;
    }
    if (x0, y0) == (x1, y1) {
        return Some(vec![]);
    }
    // 1. easy path
    let (dx, dy) = ((x1 - x0).signum(), (y1 - y0).signum());
    let (mut x, mut y) = (x0, y0);
    let mut path = Vec::new();
    while (x, y) != (x1, y1) && path.len() < MAX_WALKPATH {
        let (sx, sy) = (if x != x1 { dx } else { 0 }, if y != y1 { dy } else { 0 });
        if !can_move(g, x, y, x + sx, y + sy) {
            break;
        }
        x += sx;
        y += sy;
        path.push(dir_of(sx, sy));
    }
    if (x, y) == (x1, y1) {
        return Some(path);
    }
    // 2. A* inside a box around the endpoints
    let lim = MAX_WALKPATH as i32;
    let (minx, maxx) = (x0.min(x1) - lim, x0.max(x1) + lim);
    let (miny, maxy) = (y0.min(y1) - lim, y0.max(y1) + lim);
    let h = |x: i32, y: i32| ((x - x1).abs() + (y - y1).abs()) * 10;
    let mut open = BinaryHeap::new();
    let mut best: HashMap<(i32, i32), (i32, (i32, i32), u8)> = HashMap::new();
    best.insert((x0, y0), (0, (x0, y0), 0));
    open.push(Reverse((h(x0, y0), 0, x0, y0)));
    while let Some(Reverse((_, dist, x, y))) = open.pop() {
        if best.get(&(x, y)).is_some_and(|b| b.0 < dist) {
            continue;
        }
        if (x, y) == (x1, y1) {
            let mut dirs = Vec::new();
            let mut cur = (x, y);
            while cur != (x0, y0) {
                let (_, prev, d) = best[&cur];
                dirs.push(d);
                cur = prev;
            }
            dirs.reverse();
            return (dirs.len() <= MAX_WALKPATH).then_some(dirs);
        }
        for (d, &(sx, sy)) in DIRS.iter().enumerate() {
            let (nx, ny) = (x + sx, y + sy);
            if nx < minx || nx > maxx || ny < miny || ny > maxy || !can_move(g, x, y, nx, ny) {
                continue;
            }
            let nd = dist + if sx != 0 && sy != 0 { 14 } else { 10 };
            if best.get(&(nx, ny)).is_none_or(|b| nd < b.0) {
                best.insert((nx, ny), (nd, (x, y), d as u8));
                open.push(Reverse((nd + h(nx, ny), nd, nx, ny)));
            }
        }
    }
    None
}

/// Chebyshev distance (RO's usual "range" metric).
pub fn distance(x0: i32, y0: i32, x1: i32, y1: i32) -> i32 {
    (x0 - x1).abs().max((y0 - y1).abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn straight_and_around_wall() {
        let mut g = MapGrid::open("t", 20, 20);
        assert_eq!(find(&g, 1, 1, 4, 1).unwrap(), vec![6, 6, 6]);
        assert_eq!(find(&g, 1, 1, 3, 3).unwrap(), vec![7, 7]);
        for y in 0..10 {
            g.set_walkable(5, y, false);
        }
        let p = find(&g, 2, 2, 8, 2).unwrap();
        let (mut x, mut y) = (2, 2);
        for d in &p {
            let (dx, dy) = DIRS[*d as usize];
            assert!(can_move(&g, x, y, x + dx, y + dy));
            x += dx;
            y += dy;
        }
        assert_eq!((x, y), (8, 2));
    }

    #[test]
    fn blocked_target() {
        let mut g = MapGrid::open("t", 10, 10);
        g.set_walkable(5, 5, false);
        assert!(find(&g, 1, 1, 5, 5).is_none());
    }
}
