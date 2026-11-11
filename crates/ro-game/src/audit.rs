//! `ragnarok-offline check`: can the world be walked?
//!
//! Tests run on synthetic, all-walkable maps, so they cannot see a warp that
//! drops the player into a wall or an NPC behind one. This walks the
//! client's own cells instead: a flood fill from the start point that
//! follows every warp tile it reaches and every `"map", x, y` a script file
//! names once one of that file's NPCs is in sight of a reached cell (so
//! Kafra routes, job-test ushers and quest warps count). It over-reaches a
//! little (a file's destinations open as soon as any of its NPCs is met,
//! whatever the script would ask first) and never under-reaches, so what it
//! reports as unreachable is.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::sync::Arc;

use ro_data::gat::MapGrid;

use crate::game::Game;
use crate::world::AREA;

#[derive(Default)]
pub struct Audit {
    /// Warps whose arrival cell is not walkable, or that have no walkable cell to step on.
    pub bad_warps: Vec<String>,
    /// Maps with NPCs, shops or monsters that no path from the start leads to.
    pub unreached_maps: BTreeMap<String, usize>,
    /// NPCs and shops on reached maps that are never within sight of a reached cell.
    pub unreached_npcs: Vec<String>,
    /// Warp tiles on reached maps that cannot be stepped on from where the player can get.
    pub unreached_warps: Vec<String>,
    /// Script destinations (`"map", x, y`) on cells that are not walkable.
    pub bad_destinations: Vec<String>,
    /// Item and monster ids the scripts use and the database lacks.
    pub unknown_ids: Vec<String>,
    pub maps_reached: usize,
}

struct Reach {
    grids: HashMap<String, Option<Arc<MapGrid>>>,
    seen: HashMap<String, Vec<bool>>,
    queue: VecDeque<(String, i32, i32)>,
}

impl Reach {
    fn grid(&mut self, game: &mut Game, map: &str) -> Option<Arc<MapGrid>> {
        if let Some(g) = self.grids.get(map) {
            return g.clone();
        }
        let g = if game.ensure_map(map) { game.world.maps.get(map).map(|m| m.grid.clone()) } else { None };
        self.grids.insert(map.to_string(), g.clone());
        g
    }

    /// Flood from (x, y); returns the cells newly reached.
    fn flood(&mut self, game: &mut Game, map: &str, x: i32, y: i32) -> Vec<(i32, i32)> {
        let Some(g) = self.grid(game, map) else { return vec![] };
        let w = g.width as i32;
        let seen = self.seen.entry(map.to_string()).or_insert_with(|| vec![false; g.width as usize * g.height as usize]);
        let mut new = Vec::new();
        let mut q = VecDeque::new();
        if g.walkable(x, y) && !seen[(y * w + x) as usize] {
            seen[(y * w + x) as usize] = true;
            q.push_back((x, y));
        }
        while let Some((cx, cy)) = q.pop_front() {
            new.push((cx, cy));
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (nx, ny) = (cx + dx, cy + dy);
                if g.walkable(nx, ny) && !seen[(ny * w + nx) as usize] {
                    seen[(ny * w + nx) as usize] = true;
                    q.push_back((nx, ny));
                }
            }
        }
        new
    }

    fn reached(&self, map: &str, x: i32, y: i32) -> bool {
        let (Some(Some(g)), Some(s)) = (self.grids.get(map), self.seen.get(map)) else { return false };
        g.in_bounds(x, y) && s[(y * g.width as i32 + x) as usize]
    }

    /// Is any reached cell within `r` cells (Chebyshev) of (x, y)?
    fn near(&self, map: &str, x: i32, y: i32, rx: i32, ry: i32) -> bool {
        (-ry..=ry).any(|dy| (-rx..=rx).any(|dx| self.reached(map, x + dx, y + dy)))
    }
}

/// `"name", 12, 34` triples in a script's text, skipping warp{} declarations.
fn destinations(text: &str) -> Vec<(usize, String, i32, i32)> {
    let mut out = Vec::new();
    for (ln, line) in text.lines().enumerate() {
        let ln = ln + 1;
        let t = line.trim_start();
        if t.starts_with("warp {") || t.starts_with("warp{") || t.starts_with("--") {
            continue;
        }
        let b = line.as_bytes();
        let mut i = 0;
        while let Some(p) = line[i..].find('"') {
            let s = i + p + 1;
            let Some(e) = line[s..].find('"') else { break };
            let name = &line[s..s + e];
            i = s + e + 1;
            let mut j = i;
            let mut nums = Vec::new();
            for _ in 0..2 {
                while j < b.len() && (b[j] == b' ' || b[j] == b',') {
                    j += 1;
                }
                let st = j;
                while j < b.len() && b[j].is_ascii_digit() {
                    j += 1;
                }
                if st == j {
                    break;
                }
                nums.push(line[st..j].parse::<i32>().unwrap_or(-1));
            }
            let ok = !name.is_empty() && name.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b'-' || c == b'@');
            if nums.len() == 2 && ok {
                out.push((ln, name.to_ascii_lowercase(), nums[0], nums[1]));
            }
        }
    }
    out
}

/// Literal ids after `call(` in a script's text: `getitem(501, 1)` -> 501.
fn literal_ids(text: &str, call: &str) -> Vec<u32> {
    let mut out = Vec::new();
    let mut i = 0;
    while let Some(p) = text[i..].find(call) {
        let s = i + p + call.len();
        i = s;
        let before = text[..s - call.len()].bytes().last().unwrap_or(b' ');
        if before.is_ascii_alphanumeric() || before == b'_' {
            continue;
        }
        let digits: String = text[s..].chars().skip_while(|c| *c == ' ').take_while(|c| c.is_ascii_digit()).collect();
        if let Ok(n) = digits.parse() {
            out.push(n);
        }
    }
    out
}

fn file_of(source: &str) -> String {
    // "scripts/npc/x.lua:12" (source_of) -> "scripts/npc/x.lua"
    source.rsplit_once(':').map(|(f, _)| f).unwrap_or(source).to_string()
}

/// Line ranges (1-based, inclusive) of an NPC's functions in its file.
fn npc_lines(n: &crate::script::NpcDef) -> Vec<(usize, usize)> {
    [n.on_click.as_ref(), n.on_touch.as_ref()]
        .into_iter()
        .flatten()
        .filter_map(|f| {
            let i = f.info();
            Some((i.line_defined?, i.last_line_defined?))
        })
        .collect()
}

/// The script file an NPC's code is in (its function knows; `source` is the
/// declaring line, which a helper that builds NPCs would hide).
fn npc_file(n: &crate::script::NpcDef) -> String {
    let from_fn = n.on_click.as_ref().or(n.on_touch.as_ref()).and_then(|f| f.info().source);
    match from_fn {
        Some(s) if s.starts_with('@') => s[1..].to_string(),
        _ => file_of(&n.source),
    }
}

pub fn audit(game: &mut Game) -> Audit {
    let mut a = Audit::default();
    let Some(engine) = game.scripts.as_ref() else { return a };
    let warps = engine.defs.warps.clone();
    let npcs: Vec<(String, i32, i32, String, String, bool, Vec<(usize, usize)>)> = engine
        .defs
        .npcs
        .iter()
        .filter(|n| n.map != "-")
        .map(|n| (n.map.clone(), n.x as i32, n.y as i32, n.name.clone(), npc_file(n), n.on_click.is_some() && n.sprite.is_some(), npc_lines(n)))
        .chain(engine.defs.shops.iter().map(|s| (s.map.clone(), s.x as i32, s.y as i32, s.name.clone(), file_of(&s.source), true, vec![])))
        .collect();
    let touch: Vec<(String, i32, i32, i32, i32, String, Vec<(usize, usize)>)> = engine
        .defs
        .npcs
        .iter()
        .filter_map(|n| n.touch.map(|t| (n.map.clone(), n.x as i32, n.y as i32, t.0 as i32, t.1 as i32, npc_file(n), npc_lines(n))))
        .collect();
    let spawns: Vec<(String, u32, Option<String>, String)> =
        engine.defs.spawns.iter().map(|s| (s.map.clone(), s.mob, s.mob_name.clone(), s.source.clone())).collect();
    let shops: Vec<(Vec<u32>, String)> =
        engine.defs.shops.iter().map(|s| (s.items.iter().map(|i| i.0).collect(), s.source.clone())).collect();
    let files: Vec<std::path::PathBuf> = engine.defs.files.clone();

    // Destinations per script file, keyed the way `source` names files.
    let mut dests: HashMap<String, Vec<(usize, String, i32, i32)>> = HashMap::new();
    let mut texts = Vec::new();
    for f in &files {
        let Ok(text) = std::fs::read_to_string(f) else { continue };
        let key = npcs
            .iter()
            .map(|n| &n.4)
            .chain(touch.iter().map(|t| &t.5))
            .find(|k| f.to_string_lossy().ends_with(k.as_str()) || k.ends_with(&*f.to_string_lossy()))
            .cloned();
        if let Some(k) = key {
            dests.insert(k, destinations(&text).into_iter().filter(|d| game.res.maps.contains(&d.1)).collect());
        }
        texts.push((f.clone(), text));
    }

    // --- static: warps and destinations on cells that cannot be stood on
    let mut r = Reach { grids: HashMap::new(), seen: HashMap::new(), queue: VecDeque::new() };
    for w in &warps {
        if let Some(g) = r.grid(game, &w.to.0) {
            if !g.walkable(w.to.1 as i32, w.to.2 as i32) {
                a.bad_warps.push(format!("{} {},{} -> {} {},{}: arrival cell is not walkable ({})", w.map, w.x, w.y, w.to.0, w.to.1, w.to.2, w.source));
            }
        }
        if let Some(g) = r.grid(game, &w.map) {
            let (sx, sy) = (w.span.0 as i32, w.span.1 as i32);
            let any = (-sy..=sy).any(|dy| (-sx..=sx).any(|dx| g.walkable(w.x as i32 + dx, w.y as i32 + dy)));
            if !any {
                a.bad_warps.push(format!("{} {},{}: no walkable cell in its touch area ({})", w.map, w.x, w.y, w.source));
            }
        }
    }
    // Lines inside some NPC's function belong to that NPC; the rest of a
    // file (tables above the NPCs) to every NPC of the file.
    let mut owned: HashMap<&str, Vec<(usize, usize)>> = HashMap::new();
    for (file, ranges) in npcs.iter().map(|n| (&n.4, &n.6)).chain(touch.iter().map(|t| (&t.5, &t.6))) {
        owned.entry(file.as_str()).or_default().extend(ranges.iter().copied());
    }
    let dests_of = |file: &str, ranges: &[(usize, usize)]| -> Vec<(String, i32, i32)> {
        let all = owned.get(file).map(Vec::as_slice).unwrap_or(&[]);
        dests
            .get(file)
            .into_iter()
            .flatten()
            .filter(|d| ranges.iter().any(|r| r.0 <= d.0 && d.0 <= r.1) || !all.iter().any(|r| r.0 <= d.0 && d.0 <= r.1))
            .map(|d| (d.1.clone(), d.2, d.3))
            .collect()
    };
    let mut seen_bad = BTreeSet::new();
    for (file, ds) in &dests {
        for (_, m, x, y) in ds {
            if (*x, *y) == (0, 0) {
                continue; // a random cell
            }
            if let Some(g) = r.grid(game, m) {
                if !g.walkable(*x, *y) && seen_bad.insert((m.clone(), *x, *y)) {
                    a.bad_destinations.push(format!("{m} {x},{y} is not walkable ({file})"));
                }
            }
        }
    }

    // --- reachability from the start point
    let start = game.cfg.start_point.clone();
    r.queue.push_back((start.map.clone(), start.x as i32, start.y as i32));
    let mut opened: HashSet<(u8, usize)> = HashSet::new();
    let mut by_map_warps: HashMap<&str, Vec<&crate::script::WarpDef>> = HashMap::new();
    for w in &warps {
        by_map_warps.entry(w.map.as_str()).or_default().push(w);
    }
    let mut used_warp: HashSet<(String, u16, u16)> = HashSet::new();
    while let Some((map, x, y)) = r.queue.pop_front() {
        let new = r.flood(game, &map, x, y);
        if new.is_empty() {
            continue;
        }
        // warp tiles now touched
        for w in by_map_warps.get(map.as_str()).into_iter().flatten() {
            if !used_warp.contains(&(w.map.clone(), w.x, w.y)) && r.near(&map, w.x as i32, w.y as i32, w.span.0 as i32, w.span.1 as i32) {
                used_warp.insert((w.map.clone(), w.x, w.y));
                r.queue.push_back((w.to.0.clone(), w.to.1 as i32, w.to.2 as i32));
            }
        }
        // NPCs now in sight, and touch areas now stepped on: where their scripts lead
        let mut open = Vec::new();
        for (i, n) in npcs.iter().enumerate() {
            if n.0 == map && !opened.contains(&(0, i)) && r.near(&map, n.1, n.2, AREA, AREA) {
                opened.insert((0, i));
                open.extend(dests_of(&n.4, &n.6));
            }
        }
        for (i, t) in touch.iter().enumerate() {
            if t.0 == map && !opened.contains(&(1, i)) && r.near(&map, t.1, t.2, t.3, t.4) {
                opened.insert((1, i));
                open.extend(dests_of(&t.5, &t.6));
            }
        }
        for (m, dx, dy) in open {
            if (dx, dy) == (0, 0) {
                // a random cell: any of the map's
                if let Some(g) = r.grid(game, &m) {
                    for y in 0..g.height as i32 {
                        for x in 0..g.width as i32 {
                            if g.walkable(x, y) && !r.reached(&m, x, y) {
                                r.queue.push_back((m.clone(), x, y));
                            }
                        }
                    }
                }
            } else {
                r.queue.push_back((m, dx, dy));
            }
        }
    }
    a.maps_reached = r.seen.values().filter(|s| s.iter().any(|c| *c)).count();
    let reached_map = |r: &Reach, m: &str| r.seen.get(m).is_some_and(|s| s.iter().any(|c| *c));

    for (map, x, y, name, file, clickable, _) in &npcs {
        if !game.res.maps.contains(map) {
            continue;
        }
        if !reached_map(&r, map) {
            *a.unreached_maps.entry(map.clone()).or_default() += 1;
        } else if *clickable && !r.near(map, *x, *y, AREA, AREA) {
            a.unreached_npcs.push(format!("{map} {x},{y} {name} ({file})"));
        }
    }
    for (map, ..) in &spawns {
        if game.res.maps.contains(map) && !reached_map(&r, map) {
            *a.unreached_maps.entry(map.clone()).or_default() += 1;
        }
    }
    for w in &warps {
        if reached_map(&r, &w.map) && !used_warp.contains(&(w.map.clone(), w.x, w.y)) {
            a.unreached_warps.push(format!("{} {},{} -> {} ({})", w.map, w.x, w.y, w.to.0, w.source));
        }
    }

    // --- ids
    let mut bad = BTreeSet::new();
    for (_, mob, name, source) in &spawns {
        match name {
            Some(n) if !game.db.mobs_by_name.contains_key(n) => {
                bad.insert(format!("monster {n} ({source})"));
            }
            None if *mob != 0 && !game.db.mobs.contains_key(mob) => {
                bad.insert(format!("monster {mob} ({source})"));
            }
            _ => {}
        }
    }
    for (items, source) in &shops {
        for i in items {
            if !game.db.items.contains_key(i) {
                bad.insert(format!("item {i} in a shop ({source})"));
            }
        }
    }
    for (f, text) in &texts {
        for call in ["getitem(", "delitem(", "countitem(", "getitem2(", "checkweight("] {
            for id in literal_ids(text, call) {
                if id >= 500 && !game.db.items.contains_key(&id) {
                    bad.insert(format!("item {id} in {call}) ({})", f.display()));
                }
            }
        }
    }
    a.unknown_ids = bad.into_iter().collect();
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_destinations_and_ids() {
        let t = "  warp(\"izlude\", 176, 182)\n  { \"geffen\", 120, 39, 1200 },\nwarp { map = \"a\", to = { \"prt_in\", 1, 2 } }\n  mes \"hello\", 1, 2\n";
        assert_eq!(destinations(t), vec![(1, "izlude".into(), 176, 182), (2, "geffen".into(), 120, 39), (4, "hello".into(), 1, 2)]);
        assert_eq!(literal_ids("getitem(501, 1) countitem( 909) xgetitem(7)", "getitem("), vec![501]);
    }
}
