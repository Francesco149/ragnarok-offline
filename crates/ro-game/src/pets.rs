//! Pets, as athena-dev-2.1.1-mod0631 `map/pet.c` (2003-12-18) has them:
//! taming with an item (`catch_pet`, 0x019e/0x019f/0x01a0), eggs carrying the
//! pet id in their card slots (card0 0xff00, card1-2 id, card3 renamed),
//! hatching with the Pet Incubator (0x01a6/0x01a7), a pet that follows its
//! owner, hunger and intimacy, the pet menu (0x01a1: status, feed,
//! performance, back to egg, take accessory off), renaming once (0x01a5),
//! accessories, pet emotes (0x01a9). Data: data/db/pets.toml (mod0631 pet_db).

use crate::game::Game;
use crate::model::{InvItem, Pet, Slots};
use crate::packets;
use crate::path;
use crate::world::{PetUnit, Unit, UnitKind};

const EGG_MARK: u16 = 0xff00;

/// The pet id an egg carries, if it is a pet egg.
pub(crate) fn egg_pet_id(it: &InvItem) -> Option<u32> {
    (it.cards[0] == EGG_MARK).then(|| it.cards[1] as u32 | (it.cards[2] as u32) << 16)
}

fn egg_item(egg: u32, pet: &Pet) -> InvItem {
    let mut it = InvItem::new(egg, 1);
    it.cards = [EGG_MARK, pet.id as u16, (pet.id >> 16) as u16, pet.renamed as u16];
    it
}

impl Game {
    fn active_pet(&self, unit: u32) -> Option<Pet> {
        let c = self.char_of_unit(unit)?;
        c.pets.get(&c.active_pet).cloned()
    }

    fn pet_unit(&self, unit: u32) -> Option<u32> {
        self.world.units.get(&unit).and_then(|u| u.player()).and_then(|p| p.pet_unit)
    }

    /// 0x01a4: one pet value changed (type 0 spawn, 1 intimacy, 2 hunger, 3 accessory, 4 performance, 5 0x14).
    fn pet_data(&self, unit: u32, kind: u8, value: u32) {
        let (Some(sid), Some(pid)) = (self.sid_of(unit), self.pet_unit(unit)) else { return };
        self.send(sid, packets::pet_data(kind, pid, value));
    }

    fn pet_status(&self, unit: u32) {
        let (Some(sid), Some(p)) = (self.sid_of(unit), self.active_pet(unit)) else { return };
        self.send(sid, packets::pet_status(&p.name, p.renamed, p.level, p.hungry, p.intimate, p.equip));
    }

    // ------------------------------------------------------------ taming

    /// Script `pet(mob)` (a taming item): the client lets the player pick a target.
    pub(crate) fn catch_pet_start(&mut self, unit: u32, mob: u32) {
        if let Some(p) = self.world.units.get_mut(&unit).and_then(|u| u.player_mut()) {
            p.catch_class = Some(mob);
        }
        if let Some(sid) = self.sid_of(unit) {
            self.send(sid, packets::catch_process());
        }
    }

    /// 0x019f: the target. mod0631 pet_catch_process2.
    pub(crate) fn catch_pet_try(&mut self, unit: u32, target: u32) {
        let class = self.world.units.get_mut(&unit).and_then(|u| u.player_mut()).and_then(|p| p.catch_class.take());
        let Some(sid) = self.sid_of(unit) else { return };
        let mob = self.world.units.get(&target).and_then(|u| u.mob().map(|m| (m.mob_id, m.hp, m.max_hp)));
        let def = mob.and_then(|(id, _, _)| self.db.pet_by_mob(id).cloned());
        tracing::debug!("taming try: target {target} mob {mob:?} pet def {} class {class:?}", def.is_some());
        let (Some((mob_id, hp, max_hp)), Some(def), Some(class)) = (mob, def, class) else {
            self.send(sid, packets::catch_result(false));
            return;
        };
        if mob_id != class {
            self.send(sid, packets::catch_result(false));
            return;
        }
        let Some(c) = self.char_of_unit(unit).cloned() else { return };
        let mob_lv = self.db.mob(mob_id).map(|m| m.level as i64).unwrap_or(1);
        let luk = c.stats[crate::model::LUK] as i64;
        let hp_pct = if max_hp > 0 { hp as i64 * 100 / max_hp as i64 } else { 100 };
        let rate = ((def.capture as i64 + (c.base_level as i64 - mob_lv) * 30 + luk * 20) * (200 - hp_pct) / 100).max(1);
        tracing::debug!("taming {mob_id}: rate {rate}/10000 (capture {}, lv {} vs {mob_lv}, luk {luk}, hp {hp_pct}%)", def.capture, c.base_level);
        if (rand::random::<u32>() % 10000) as i64 >= rate {
            self.send(sid, packets::catch_result(false));
            return;
        }
        self.despawn(target, 0);
        self.send(sid, packets::catch_result(true));
        let Ok(id) = self.store.new_pet_id() else { return };
        let pet = Pet {
            id,
            mob: mob_id,
            name: self.db.mob(mob_id).map(|m| m.display.clone()).unwrap_or_else(|| def.name.clone()),
            level: mob_lv as u16,
            equip: 0,
            intimate: def.intimate,
            hungry: 100,
            renamed: false,
        };
        let egg = egg_item(def.egg, &pet);
        if let Some(c) = self.char_of_unit_mut(unit) {
            c.pets.insert(id, pet);
        }
        if !self.add_item(unit, egg.clone()) {
            let (m, x, y) = self.world.units.get(&unit).map(|u| (u.map.clone(), u.x, u.y)).unwrap();
            self.drop_floor_item(&m, x, y, egg, true);
        }
    }

    // ------------------------------------------------------------ eggs

    /// Script `birth_pet()` (Pet Incubator): list the eggs.
    pub(crate) fn pet_egg_list(&mut self, unit: u32) {
        let Some(sid) = self.sid_of(unit) else { return };
        if self.pet_unit(unit).is_some() {
            return;
        }
        let eggs: Vec<u16> = self
            .char_of_unit(unit)
            .map(|c| c.inventory.iter().enumerate().filter(|(_, it)| egg_pet_id(it).is_some()).map(|(i, _)| i as u16 + packets::INV_OFFSET).collect())
            .unwrap_or_default();
        self.send(sid, packets::egg_list(&eggs));
    }

    /// 0x01a7: hatch the egg at this inventory index.
    pub(crate) fn pet_hatch(&mut self, unit: u32, idx: u16) {
        let slot = idx.wrapping_sub(packets::INV_OFFSET) as usize;
        let Some(it) = self.char_of_unit(unit).and_then(|c| c.inventory.slot(slot)).cloned() else { return };
        let Some(pid) = egg_pet_id(&it) else { return };
        if self.pet_unit(unit).is_some() || !self.char_of_unit(unit).is_some_and(|c| c.pets.contains_key(&pid)) {
            return;
        }
        self.remove_item(unit, slot, 1);
        if let Some(c) = self.char_of_unit_mut(unit) {
            c.active_pet = pid;
        }
        self.spawn_pet(unit);
    }

    /// Put the active pet next to its owner (hatching, login, after a warp).
    pub(crate) fn spawn_pet(&mut self, unit: u32) {
        if let Some(old) = self.pet_unit(unit) {
            self.despawn(old, 0);
        }
        let Some(pet) = self.active_pet(unit) else { return };
        let Some(def) = self.db.pet_by_mob(pet.mob).cloned() else { return };
        let Some((map, x, y, dir)) = self.world.units.get(&unit).map(|u| (u.map.clone(), u.x, u.y, u.dir)) else { return };
        let (px, py) = self.pet_spot(&map, x, y, dir);
        let id = self.world.new_npc_id();
        let now = self.now();
        self.world.add(Unit {
            id,
            map,
            x: px,
            y: py,
            dir,
            head_dir: 0,
            speed: if def.speed > 0 { def.speed } else { crate::world::DEFAULT_SPEED },
            class: pet.mob as u16,
            name: pet.name.clone(),
            state: 0,
            walk: None,
            kind: UnitKind::Pet(PetUnit { owner: unit, pet_id: pet.id, equip: pet.equip, next_hunger: now + def.hungry_delay.max(1) as u64 * 1000 }),
        });
        if let Some(p) = self.world.units.get_mut(&unit).and_then(|u| u.player_mut()) {
            p.pet_unit = Some(id);
        }
        self.update_watchers(id, true);
        // mod0631 pet_birth_process
        self.pet_data(unit, 0, 0);
        self.pet_data(unit, 5, 0x14);
        self.area_send(id, packets::pet_data(3, id, pet.equip), true);
        self.pet_status(unit);
    }

    /// A free cell behind the owner (mod0631 pet_calc_pos, simplified).
    fn pet_spot(&self, map: &str, x: u16, y: u16, dir: u8) -> (u16, u16) {
        let Some(g) = self.grid(map) else { return (x, y) };
        let (bx, by) = path::DIRS[((dir as usize) + 4) % 8];
        for (dx, dy) in [(bx, by), (bx * 2, by * 2), (1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if g.walkable(nx, ny) {
                return (nx as u16, ny as u16);
            }
        }
        (x, y)
    }

    /// Pet back into its egg (also on logout and when its owner leaves).
    pub(crate) fn pet_to_egg(&mut self, unit: u32) {
        let Some(pu) = self.pet_unit(unit) else { return };
        self.despawn(pu, 0);
        if let Some(p) = self.world.units.get_mut(&unit).and_then(|u| u.player_mut()) {
            p.pet_unit = None;
        }
        let Some(pet) = self.active_pet(unit) else { return };
        let Some(def) = self.db.pet_by_mob(pet.mob).cloned() else { return };
        if let Some(c) = self.char_of_unit_mut(unit) {
            c.active_pet = 0;
        }
        let egg = egg_item(def.egg, &pet);
        if !self.add_item(unit, egg.clone()) {
            let (m, x, y) = self.world.units.get(&unit).map(|u| (u.map.clone(), u.x, u.y)).unwrap();
            self.drop_floor_item(&m, x, y, egg, true);
        }
    }

    /// Owner leaves the world: the pet goes away but stays out for next login.
    /// The owner died: the pet out of its egg loses `die` intimacy (mod0631 pc.c:2832).
    pub(crate) fn pet_owner_died(&mut self, unit: u32) {
        let Some(pet) = self.active_pet(unit) else { return };
        let Some(die) = self.db.pet_by_mob(pet.mob).map(|d| d.die) else { return };
        let mut now = 0;
        if let Some(p) = self.char_of_unit_mut(unit).and_then(|c| c.pets.get_mut(&pet.id)) {
            p.intimate = (p.intimate - die).max(0);
            now = p.intimate;
        }
        self.pet_data(unit, 1, now as u32);
    }

    pub(crate) fn pet_owner_left(&mut self, unit: u32) {
        if let Some(pu) = self.pet_unit(unit) {
            self.despawn(pu, 0);
        }
    }

    // ------------------------------------------------------------ care

    /// 0x01a1: 0 status, 1 feed, 2 performance, 3 back to egg, 4 accessory off.
    pub(crate) fn pet_menu(&mut self, unit: u32, menu: u8) {
        if self.pet_unit(unit).is_none() {
            return;
        }
        match menu {
            0 => self.pet_status(unit),
            1 => self.pet_feed(unit),
            2 => {
                if let Some(pu) = self.pet_unit(unit) {
                    let n = 1 + rand::random::<u32>() % 3;
                    self.area_send(pu, packets::pet_data(4, pu, n), true);
                }
            }
            3 => self.pet_to_egg(unit),
            4 => self.pet_unequip(unit),
            _ => {}
        }
    }

    /// mod0631 pet_food: intimacy up when hungry, down when very full.
    fn pet_feed(&mut self, unit: u32) {
        let Some(pet) = self.active_pet(unit) else { return };
        let Some(def) = self.db.pet_by_mob(pet.mob).cloned() else { return };
        let Some(sid) = self.sid_of(unit) else { return };
        if !self.take_item(unit, def.food, 1) {
            self.send(sid, packets::pet_food(false, def.food));
            return;
        }
        let Some(c) = self.char_of_unit_mut(unit) else { return };
        let Some(p) = c.pets.get_mut(&pet.id) else { return };
        if p.hungry > 90 {
            p.intimate -= def.r_full;
        } else if p.hungry > 75 {
            p.intimate += (def.r_hungry * 25 / 100).max(1);
        } else {
            p.intimate += def.r_hungry;
        }
        p.intimate = p.intimate.clamp(0, 1000);
        p.hungry = (p.hungry + def.fullness).min(100);
        let (h, i) = (p.hungry, p.intimate);
        self.pet_data(unit, 2, h as u32);
        self.pet_data(unit, 1, i as u32);
        self.send(sid, packets::pet_food(true, def.food));
    }

    /// Wearing a pet accessory (an item of type 8): mod0631 pet_equipitem.
    pub(crate) fn pet_equip(&mut self, unit: u32, slot: usize) -> bool {
        let Some(pet) = self.active_pet(unit) else { return false };
        let Some(def) = self.db.pet_by_mob(pet.mob).cloned() else { return false };
        let Some(it) = self.char_of_unit(unit).and_then(|c| c.inventory.slot(slot)).cloned() else { return false };
        if def.accessory == 0 || it.nameid != def.accessory || pet.equip != 0 {
            return false;
        }
        self.remove_item(unit, slot, 1);
        self.set_pet_equip(unit, pet.id, it.nameid);
        true
    }

    fn pet_unequip(&mut self, unit: u32) {
        let Some(pet) = self.active_pet(unit) else { return };
        if pet.equip == 0 {
            return;
        }
        self.set_pet_equip(unit, pet.id, 0);
        self.add_item(unit, InvItem::new(pet.equip, 1));
    }

    fn set_pet_equip(&mut self, unit: u32, pet_id: u32, item: u32) {
        if let Some(p) = self.char_of_unit_mut(unit).and_then(|c| c.pets.get_mut(&pet_id)) {
            p.equip = item;
        }
        if let Some(pu) = self.pet_unit(unit) {
            if let Some(UnitKind::Pet(p)) = self.world.units.get_mut(&pu).map(|u| &mut u.kind) {
                p.equip = item;
            }
            self.area_send(pu, packets::pet_data(3, pu, item), true);
        }
    }

    /// 0x01a5: a pet may be renamed once (mod0631 pet_rename default).
    pub(crate) fn pet_rename(&mut self, unit: u32, name: &str) {
        let Some(pet) = self.active_pet(unit) else { return };
        if pet.renamed || name.trim().is_empty() {
            return;
        }
        if let Some(p) = self.char_of_unit_mut(unit).and_then(|c| c.pets.get_mut(&pet.id)) {
            // No '%': the client copies its own pet's name with sprintf(name)
            // (0x0095 at 0x516917, 0x01a2 at 0x52933b). mod0631 keeps it.
            p.name = name.chars().filter(|&c| c != '%').take(23).collect();
            p.renamed = true;
        }
        self.spawn_pet(unit);
    }

    /// 0x01a9: a pet emote, shown to everyone around.
    pub(crate) fn pet_emotion(&mut self, unit: u32, emotion: u32) {
        if let Some(pu) = self.pet_unit(unit) {
            self.area_send(pu, packets::pet_emotion(pu, emotion), true);
        }
    }

    // ------------------------------------------------------------ every tick

    /// Hunger (mod0631 pet_hungry) and following the owner.
    pub(crate) fn tick_pets(&mut self, now: u64) {
        let pets: Vec<(u32, u32, u32, u64)> = self
            .world
            .units
            .values()
            .filter_map(|u| match &u.kind {
                UnitKind::Pet(p) => Some((u.id, p.owner, p.pet_id, p.next_hunger)),
                _ => None,
            })
            .collect();
        for (pu, owner, pet_id, next_hunger) in pets {
            let Some(o) = self.world.units.get(&owner) else {
                self.despawn(pu, 0);
                continue;
            };
            let (omap, ox, oy, odir) = (o.map.clone(), o.x, o.y, o.dir);
            if now >= next_hunger {
                let mob = self.char_of_unit(owner).and_then(|c| c.pets.get(&pet_id)).map(|p| p.mob).unwrap_or(0);
                let delay = self.db.pet_by_mob(mob).map(|d| d.hungry_delay.max(1)).unwrap_or(60) as u64 * 1000;
                if let Some(UnitKind::Pet(p)) = self.world.units.get_mut(&pu).map(|u| &mut u.kind) {
                    p.next_hunger = now + delay;
                }
                let mut intimacy_changed = None;
                let mut hunger = None;
                if let Some(p) = self.char_of_unit_mut(owner).and_then(|c| c.pets.get_mut(&pet_id)) {
                    p.hungry -= 1;
                    if p.hungry < 0 {
                        p.hungry = 0;
                        p.intimate = (p.intimate - 1).max(0);
                        intimacy_changed = Some(p.intimate);
                    }
                    hunger = Some(p.hungry);
                }
                if let Some(i) = intimacy_changed {
                    self.pet_data(owner, 1, i as u32);
                }
                if let Some(h) = hunger {
                    self.pet_data(owner, 2, h as u32);
                }
            }
            let Some(u) = self.world.units.get(&pu) else { continue };
            if u.map != omap {
                continue; // spawn_pet moves it after the owner's map change
            }
            let far = path::distance(u.x as i32, u.y as i32, ox as i32, oy as i32) > 2;
            if far && u.walk.is_none() {
                let (tx, ty) = self.pet_spot(&omap, ox, oy, odir);
                self.walk_to(pu, tx, ty);
            }
        }
    }
}
