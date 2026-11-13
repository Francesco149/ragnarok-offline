//! Cell pathfinding.
//!
//! [`find`] is the client's own path finder (tier `client`: Ragexe
//! 2003-12-18, `CPathFinder` at `0x4d2300`), checked against the exe's code
//! run under an emulator (`tests/client_paths.rs`). The client walks its own
//! path to the cell it is told; the server walks the same one, so a unit
//! stands on the cell the client draws it on. [`find_easy`] is Athena's
//! straight-line path (path.c, flag 1) for monsters' random walks, which
//! take no detours; [`can_shoot`] is Athena's battle_check_range.
//!
//! Directions (RO convention): 0 N(+y) 1 NW 2 W 3 SW 4 S 5 SE 6 E 7 NE.

use std::collections::HashMap;

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

/// Milliseconds a step takes at `speed` (ms a cell), as the client times its
/// walk (`0x4d2cf0`): a diagonal step is `(int)(speed * 1.414f)` (the float
/// at `0x5e4440`), where Athena has `speed * 14 / 10`.
pub fn step_time(speed: u32, dir: u8) -> u32 {
    if is_diagonal(dir) { (speed as f64 * 1.414f32 as f64) as u32 } else { speed }
}

/// When the client has a walking unit at the centre of each cell of its path
/// (the start cell, then one per step), in ms after the move's tick:
/// `CPathFinder::FindPath` (`0x4d2300`) and the path it builds (`0x4d2cf0`).
/// The unit starts from a point within its cell, `x - 0.5 + sub/16` (only
/// `sub/16` on column or row 0; sub 8 is the centre, all the server sends).
/// The first step takes `(int)(speed * sqrtf(distance²))` from that point to
/// the next cell's centre, the product rounded to a float: Direct3D 7 keeps
/// the x87 at 24-bit precision (the exe never asks for `DDSCL_FPUPRESERVE`).
/// Every later step takes [`step_time`]. Halfway between two of these times
/// the unit crosses into the next cell. Checked under an emulator
/// (`tests/client_paths.rs`).
pub fn client_times(x0: i32, y0: i32, sub: (u8, u8), dirs: &[u8], speed: u32) -> Vec<u32> {
    let mut t = Vec::with_capacity(dirs.len() + 1);
    t.push(0u32);
    let Some(&d0) = dirs.first() else { return t };
    // all exact in a float: whole cells and sixteenths
    let at = |c: i32, s: u8| s as f32 * 0.0625 + if c > 0 { c as f32 - 0.5 } else { 0.0 };
    let (dx, dy) = DIRS[d0 as usize];
    let ex = (x0 + dx) as f32 - at(x0, sub.0);
    let ey = (y0 + dy) as f32 - at(y0, sub.1);
    // sqrtf at 0x4085a0; the product is exact in a double, then rounded to
    // a float as the x87 does at 24 bits, then truncated (_ftol, 0x5bec90)
    let first = (speed as f32 as f64 * (ex * ex + ey * ey).sqrt() as f64) as f32 as u32;
    t.push(first);
    for &d in &dirs[1..] {
        t.push(t[t.len() - 1].wrapping_add(step_time(speed, d)));
    }
    t
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

/// Can something at (x0,y0) shoot or strike at (x1,y1)? Athena's
/// battle_check_range: the straight ("easy") path must be free, where cliffs
/// that cannot be walked may still be shot over (path_search flag 0x10001).
pub fn can_shoot(g: &MapGrid, x0: i32, y0: i32, x1: i32, y1: i32) -> bool {
    let free = |x: i32, y: i32| g.flags(x, y) & ro_data::gat::SHOOT != 0;
    if (x0, y0) == (x1, y1) {
        return true;
    }
    if !free(x1, y1) {
        return false;
    }
    let (dx, dy) = ((x1 - x0).signum(), (y1 - y0).signum());
    let (mut x, mut y) = (x0, y0);
    for _ in 0..64 {
        if (x, y) == (x1, y1) {
            return true;
        }
        let (sx, sy) = (if x != x1 { dx } else { 0 }, if y != y1 { dy } else { 0 });
        let (nx, ny) = (x + sx, y + sy);
        // a diagonal step needs both cells beside it free, as walking does
        if !free(nx, ny) || (sx != 0 && sy != 0 && (!free(x, ny) || !free(nx, y))) {
            return false;
        }
        (x, y) = (nx, ny);
    }
    false
}

/// Path from (x0,y0) to (x1,y1) as a list of directions, or None: the
/// client's own path finder (`CPathFinder`, Ragexe 2003-12-18 `0x4d2300`),
/// so that the cells the server walks are the cells the client draws.
/// Paths longer than [`MAX_WALKPATH`] are refused, as Athena does.
pub fn find(g: &MapGrid, x0: i32, y0: i32, x1: i32, y1: i32) -> Option<Vec<u8>> {
    if !g.walkable(x1, y1) {
        return None;
    }
    if (x0, y0) == (x1, y1) {
        return Some(vec![]);
    }
    client_search(g, x0, y0, x1, y1).filter(|p| p.len() <= MAX_WALKPATH)
}

/// Nodes the client's path finder can hold (`cmp eax, 0x95` at `0x4d2902`);
/// a search that needs more fails ("find path aborted : buffer overflow").
pub const CLIENT_NODES: usize = 149;

/// The neighbours in the order the client tries them (`0x4d2524`-`0x4d26c6`):
/// (dx, dy, cost, direction).
const CLIENT_NEIGHBOURS: [(i32, i32, i32, u8); 8] =
    [(1, -1, 14, 5), (1, 0, 10, 6), (1, 1, 14, 7), (0, 1, 10, 0), (-1, 1, 14, 1), (-1, 0, 10, 2), (-1, -1, 14, 3), (0, -1, 10, 4)];

struct Node {
    x: i32,
    y: i32,
    parent: usize, // usize::MAX: none
    g: i32,
    f: i32,
    state: u8, // 0 new, 1 open, 2 closed
    dir: u8,
}

/// `0x4d2be0`: the target cell walkable and, for a diagonal step, both
/// cells beside it. The cell stepped from is not looked at.
fn client_can_move(g: &MapGrid, x0: i32, y0: i32, x1: i32, y1: i32) -> bool {
    g.walkable(x1, y1) && (x0 == x1 || y0 == y1 || (g.walkable(x0, y1) && g.walkable(x1, y0)))
}

/// MSVC 6's `_Push_heap` on the open list ordered by f, smallest first.
fn sift_up(heap: &mut [usize], nodes: &[Node], mut hole: usize, top: usize, v: usize) {
    while hole > top {
        let parent = (hole - 1) / 2;
        if nodes[heap[parent]].f <= nodes[v].f {
            break;
        }
        heap[hole] = heap[parent];
        hole = parent;
    }
    heap[hole] = v;
}

/// MSVC 6's `pop_heap` (`_Adjust_heap`, `0x4d4770`): on equal f the right
/// child moves up.
fn pop(heap: &mut Vec<usize>, nodes: &[Node]) -> usize {
    let n = heap.len();
    let front = heap[0];
    let last = heap[n - 1];
    heap[n - 1] = front;
    let len = n - 1;
    let (mut hole, mut j) = (0, 2);
    while j < len {
        if nodes[heap[j]].f > nodes[heap[j - 1]].f {
            j -= 1;
        }
        heap[hole] = heap[j];
        hole = j;
        j = 2 * j + 2;
    }
    if j == len {
        heap[hole] = heap[j - 1];
        hole = j - 1;
    }
    sift_up(heap, nodes, hole, 0, last);
    heap.pop();
    front
}

/// The client's A* (`0x4d2300`), step for step: f = g + 10 x the Manhattan
/// distance left, 10 a straight step and 14 a diagonal, the node pool of
/// [`CLIENT_NODES`], and its quirk: a closed node that is reached more
/// cheaply is marked open again but not put back on the open list
/// (`0x4d2a1f`), so it is never expanded again.
pub fn client_search(g: &MapGrid, x0: i32, y0: i32, x1: i32, y1: i32) -> Option<Vec<u8>> {
    if (x0, y0) == (x1, y1) {
        return None;
    }
    let h = |x: i32, y: i32| ((x - x1).abs() + (y - y1).abs()) * 10;
    let mut nodes: Vec<Node> = Vec::with_capacity(CLIENT_NODES);
    let mut at: HashMap<(i32, i32), usize> = HashMap::new();
    let mut heap: Vec<usize> = Vec::new();
    nodes.push(Node { x: x0, y: y0, parent: usize::MAX, g: 0, f: h(x0, y0), state: 1, dir: 0 });
    at.insert((x0, y0), 0);
    heap.push(0);
    while !heap.is_empty() {
        let cur = pop(&mut heap, &nodes);
        let (cx, cy) = (nodes[cur].x, nodes[cur].y);
        if (cx, cy) == (x1, y1) {
            let mut dirs = Vec::new();
            let mut n = cur;
            while nodes[n].parent != usize::MAX {
                dirs.push(nodes[n].dir);
                n = nodes[n].parent;
            }
            dirs.reverse();
            return Some(dirs);
        }
        for &(dx, dy, cost, dir) in &CLIENT_NEIGHBOURS {
            let (nx, ny) = (cx + dx, cy + dy);
            if !client_can_move(g, cx, cy, nx, ny) {
                continue;
            }
            // 0x4d28b0
            let ng = nodes[cur].g + cost;
            let i = match at.get(&(nx, ny)) {
                Some(&i) => i,
                None => {
                    if nodes.len() >= CLIENT_NODES {
                        return None;
                    }
                    nodes.push(Node { x: nx, y: ny, parent: usize::MAX, g: 0, f: 0, state: 0, dir: 0 });
                    at.insert((nx, ny), nodes.len() - 1);
                    nodes.len() - 1
                }
            };
            let n = &mut nodes[i];
            if n.state != 0 && n.g <= ng {
                continue;
            }
            n.parent = cur;
            n.g = ng;
            n.f = ng + h(nx, ny);
            n.dir = dir;
            match n.state {
                0 => {
                    n.state = 1;
                    heap.push(i);
                    let last = heap.len() - 1;
                    sift_up(&mut heap, &nodes, last, 0, i);
                }
                1 => {
                    if let Some(pos) = heap.iter().position(|&k| nodes[k].x == nx && nodes[k].y == ny) {
                        sift_up(&mut heap, &nodes, pos, 0, i);
                    }
                }
                _ => n.state = 1,
            }
        }
        nodes[cur].state = 2;
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
    fn diagonal_steps_take_the_clients_time() {
        assert_eq!(step_time(150, 6), 150);
        assert_eq!(step_time(150, 7), 212); // Athena: 210
        assert_eq!(step_time(100, 1), 141);
        assert_eq!(step_time(200, 3), 282);
    }

    #[test]
    fn blocked_target() {
        let mut g = MapGrid::open("t", 10, 10);
        g.set_walkable(5, 5, false);
        assert!(find(&g, 1, 1, 5, 5).is_none());
    }
}

#[cfg(test)]
mod shoot_tests {
    use super::*;

    #[test]
    fn walls_block_a_shot() {
        let mut g = MapGrid::open("t", 10, 10);
        assert!(can_shoot(&g, 1, 1, 6, 1));
        assert!(can_shoot(&g, 1, 1, 5, 4));
        g.set_walkable(3, 1, false); // not walkable but still shootable: a cliff (or an Ice Wall)
        assert!(can_shoot(&g, 1, 1, 6, 1), "a cliff can be shot over");
        g.set_flags(3, 1, 0);
        assert!(!can_shoot(&g, 1, 1, 6, 1), "a wall cannot");
        assert!(can_shoot(&g, 1, 2, 6, 2), "the row beside it is free");
        // a diagonal step may not cut a wall's corner
        g.set_flags(2, 2, 0);
        assert!(!can_shoot(&g, 1, 1, 3, 3));
    }
}
