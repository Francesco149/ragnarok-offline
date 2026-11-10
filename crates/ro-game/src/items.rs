//! Inventory, equipment, floor items, NPC shops.

use crate::game::Game;
use crate::model::InvItem;
use crate::packets::{self, INV_OFFSET};
use crate::script::{Ctx, CtxKind};
use crate::world::{FloorItemUnit, Unit, UnitKind};

const MAX_INVENTORY: usize = 100;
const MAX_STACK: u32 = 30000;
/// Floor items vanish after this long (Athena flooritem_lifetime 60 s).
const FLOOR_LIFETIME_MS: u64 = 60_000;

// equip location bits
const EQP_ACC_L: u32 = 0x0008;
const EQP_ACC_R: u32 = 0x0080;

impl Game {
    /// Client item type for list packets.
    fn client_type(&self, nameid: u32) -> u8 {
        match self.db.item(nameid).map(|d| d.kind).unwrap_or(3) {
            11 => 2,
            k => k,
        }
    }

    pub(crate) fn send_inventory(&mut self, unit: u32) {
        let Some(sid) = self.sid_of(unit) else { return };
        let Some(c) = self.char_of_unit(unit) else { return };
        let mut normal = Vec::new();
        let mut equips = Vec::new();
        for (i, it) in c.inventory.iter().enumerate() {
            let idx = i as u16 + INV_OFFSET;
            let d = self.db.item(it.nameid);
            if d.is_some_and(|d| d.is_equip() && d.kind != 10) {
                equips.push((idx, it, self.client_type(it.nameid), d.map(|d| d.loc).unwrap_or(0)));
            } else {
                normal.push((idx, it, self.client_type(it.nameid)));
            }
        }
        let (a, b) = (packets::item_list(&normal), packets::equip_list(&equips));
        self.send(sid, a);
        self.send(sid, b);
    }

    /// Add items; returns false (and sends the failure) if they don't fit.
    pub(crate) fn add_item(&mut self, unit: u32, item: InvItem) -> bool {
        let Some(sid) = self.sid_of(unit) else { return false };
        let Some(d) = self.db.item(item.nameid).cloned() else { return false };
        let stackable = !d.is_equip() || d.kind == 10;
        let derived = match self.char_of_unit(unit) {
            Some(c) => (self.derive(c).max_weight, self.weight_of(c)),
            None => return false,
        };
        let loc = d.loc;
        let kind = self.client_type(item.nameid);
        if derived.1 + d.weight * item.amount > derived.0 {
            self.send(sid, packets::item_add(0, 0, &item, loc, kind, 2));
            return false;
        }
        let c = self.char_of_unit_mut(unit).unwrap();
        let slot = if stackable {
            c.inventory.iter().position(|i| i.nameid == item.nameid && i.equip == 0 && i.cards == item.cards)
        } else {
            None
        };
        let idx = match slot {
            Some(i) if c.inventory[i].amount + item.amount <= MAX_STACK => {
                c.inventory[i].amount += item.amount;
                i
            }
            Some(_) => {
                self.send(sid, packets::item_add(0, 0, &item, loc, kind, 1));
                return false;
            }
            None if c.inventory.len() >= MAX_INVENTORY => {
                self.send(sid, packets::item_add(0, 0, &item, loc, kind, 1));
                return false;
            }
            None => {
                c.inventory.push(item.clone());
                c.inventory.len() - 1
            }
        };
        let it = c.inventory[idx].clone();
        let amount = item.amount as u16;
        self.send(sid, packets::item_add(idx as u16 + INV_OFFSET, amount, &it, loc, kind, 0));
        self.send_weight(unit);
        true
    }

    /// Remove `amount` from inventory index `idx` (0-based).
    pub(crate) fn remove_item(&mut self, unit: u32, idx: usize, amount: u32) -> bool {
        let Some(sid) = self.sid_of(unit) else { return false };
        let Some(c) = self.char_of_unit_mut(unit) else { return false };
        let Some(it) = c.inventory.get_mut(idx) else { return false };
        if it.amount < amount || amount == 0 {
            return false;
        }
        it.amount -= amount;
        if it.amount == 0 {
            c.inventory.remove(idx);
            // indices shift: resend the whole inventory so the client agrees
            self.send(sid, packets::item_remove(idx as u16 + INV_OFFSET, amount as u16));
            self.send_inventory(unit);
        } else {
            self.send(sid, packets::item_remove(idx as u16 + INV_OFFSET, amount as u16));
        }
        self.send_weight(unit);
        true
    }

    pub(crate) fn send_weight(&mut self, unit: u32) {
        let Some(sid) = self.sid_of(unit) else { return };
        let Some(c) = self.char_of_unit(unit) else { return };
        let (w, mw) = (self.weight_of(c), self.derive(c).max_weight);
        self.send(sid, packets::par(crate::status::SP_WEIGHT, w));
        self.send(sid, packets::par(crate::status::SP_MAXWEIGHT, mw));
        // pc.c:622 pc_checkweighticon: the 50% and 90% icons (35, 36)
        let flag = if w as u64 * 10 >= mw as u64 * 9 {
            2
        } else if w as u64 * 2 >= mw as u64 {
            1
        } else {
            0
        };
        if let Some(p) = self.world.units.get_mut(&unit).and_then(|u| u.player_mut()) {
            let old = std::mem::replace(&mut p.weight_icon, flag);
            if old != flag {
                self.send(sid, packets::status_icon(35, unit, flag == 1));
                self.send(sid, packets::status_icon(36, unit, flag == 2));
            }
        }
    }

    pub(crate) fn count_item(&self, unit: u32, nameid: u32) -> u32 {
        self.char_of_unit(unit)
            .map(|c| c.inventory.iter().filter(|i| i.nameid == nameid).map(|i| i.amount).sum())
            .unwrap_or(0)
    }

    /// Take `amount` of an item id from wherever it is stacked.
    pub(crate) fn take_item(&mut self, unit: u32, nameid: u32, mut amount: u32) -> bool {
        if self.count_item(unit, nameid) < amount {
            return false;
        }
        while amount > 0 {
            let Some(c) = self.char_of_unit(unit) else { return false };
            let Some((idx, have)) = c.inventory.iter().enumerate().find(|(_, i)| i.nameid == nameid && i.equip == 0).map(|(i, it)| (i, it.amount)) else {
                return false;
            };
            let n = have.min(amount);
            self.remove_item(unit, idx, n);
            amount -= n;
        }
        true
    }

    pub(crate) fn use_item(&mut self, unit: u32, idx: u16) {
        let Some(sid) = self.sid_of(unit) else { return };
        let i = idx.wrapping_sub(INV_OFFSET) as usize;
        let Some(it) = self.char_of_unit(unit).and_then(|c| c.inventory.get(i)).cloned() else { return };
        let Some(d) = self.db.item(it.nameid).cloned() else { return };
        // pc_isUseitem: the item's sex, level and jobs
        let (class, lv) = self.char_of_unit(unit).map(|c| (c.class, c.base_level)).unwrap_or((0, 1));
        let sex = self.world.units.get(&unit).and_then(|u| u.player()).map(|p| p.sex).unwrap_or(1);
        let allowed = (d.gender == 2 || d.gender == sex) && lv >= d.equip_level as u16 && (d.jobs == 0 || d.jobs & (1u32 << class.min(31)) != 0);
        if !matches!(d.kind, 0 | 2 | 11) || !allowed || self.world.units.get(&unit).is_some_and(|u| u.state == 1) {
            self.send(sid, packets::use_item_ack(idx, it.amount as u16, false));
            return;
        }
        if d.use_script.is_empty() {
            self.tell(unit, &format!("{}: no use effect yet (Athena script: {})", d.display, d.athena_use));
            self.send(sid, packets::use_item_ack(idx, it.amount as u16, false));
            return;
        }
        let Some(engine) = self.scripts.clone() else { return };
        let ctx = Ctx::new(unit, 0, CtxKind::Item);
        let r = engine.run_item(self, ctx, d.id, &d.use_script);
        match r {
            Ok(true) => {}
            Ok(false) => {
                // refused (e.g. a Fly Wing on a noteleport map): kept
                self.send(sid, packets::use_item_ack(idx, it.amount as u16, false));
                return;
            }
            Err(e) => {
                self.notice(format!("item {}: {e:#}", d.id));
                self.send(sid, packets::use_item_ack(idx, it.amount as u16, false));
                return;
            }
        }
        let left = it.amount - 1;
        if let Some(c) = self.char_of_unit_mut(unit) {
            if left == 0 {
                c.inventory.remove(i);
            } else {
                c.inventory[i].amount = left;
            }
        }
        self.send(sid, packets::use_item_ack(idx, left as u16, true));
        if left == 0 {
            self.send_inventory(unit);
        }
        self.send_weight(unit);
    }

    pub(crate) fn equip(&mut self, unit: u32, idx: u16) {
        let Some(sid) = self.sid_of(unit) else { return };
        let i = idx.wrapping_sub(INV_OFFSET) as usize;
        let Some(c) = self.char_of_unit(unit) else { return };
        let Some(it) = c.inventory.get(i).cloned() else { return };
        let Some(d) = self.db.item(it.nameid).cloned() else { return };
        if d.kind == 8 {
            // pet accessory: goes on the pet (mod0631 pet_equipitem)
            let ok = self.pet_equip(unit, i);
            self.send(sid, packets::equip_ack(idx, 0, ok));
            return;
        }
        let Some(c) = self.char_of_unit(unit) else { return };
        let job_ok = d.jobs & (1u32 << c.class.min(31)) != 0 || d.jobs == 0;
        let sex = self.world.units.get(&unit).and_then(|u| u.player()).map(|p| p.sex).unwrap_or(1);
        if !d.is_equip() || c.base_level < d.equip_level as u16 || !job_ok || (d.gender != 2 && d.gender != sex) {
            self.send(sid, packets::equip_ack(idx, 0, false));
            return;
        }
        // unidentified equipment cannot be worn
        if !it.identify {
            self.send(sid, packets::equip_ack(idx, 0, false));
            return;
        }
        let mut loc = d.loc;
        let used: u32 = c.inventory.iter().map(|x| x.equip).fold(0, |a, b| a | b);
        if loc & (EQP_ACC_L | EQP_ACC_R) == (EQP_ACC_L | EQP_ACC_R) {
            loc = if used & EQP_ACC_L == 0 { EQP_ACC_L } else { EQP_ACC_R };
        }
        // an Assassin with Lefthand Mastery (134) wields a second one-handed
        // weapon in the shield hand (mod0631 pc_equipitem: pos 0x22)
        if d.kind == 4 && d.loc == 2 && c.class == 12 && c.skills.get(&134).copied().unwrap_or(0) > 0 && used & 0x02 != 0 {
            loc = 0x20;
        }
        if d.kind == 10 {
            // arrows go in the quiver: no slot clash, the client gets 0x013c
            let old: Vec<usize> = c.inventory.iter().enumerate().filter(|(_, x)| x.equip & 0x8000 != 0).map(|(j, _)| j).collect();
            if let Some(c) = self.char_of_unit_mut(unit) {
                for j in old {
                    c.inventory[j].equip = 0;
                }
                c.inventory[i].equip = 0x8000;
            }
            self.send(sid, packets::arrow_equip(idx));
            self.send(sid, packets::arrow_fail(3));
            self.refresh_status(unit);
            return;
        }
        // take off whatever occupies those slots
        let clash: Vec<usize> = c.inventory.iter().enumerate().filter(|(_, x)| x.equip & loc != 0).map(|(j, _)| j).collect();
        for j in clash {
            self.unequip(unit, j as u16 + INV_OFFSET);
        }
        if let Some(c) = self.char_of_unit_mut(unit) {
            c.inventory[i].equip = loc;
        }
        self.send(sid, packets::equip_ack(idx, loc, true));
        self.after_equip_change(unit);
    }

    pub(crate) fn unequip(&mut self, unit: u32, idx: u16) {
        let Some(sid) = self.sid_of(unit) else { return };
        let i = idx.wrapping_sub(INV_OFFSET) as usize;
        let Some(c) = self.char_of_unit_mut(unit) else { return };
        let Some(it) = c.inventory.get_mut(i) else { return };
        let loc = it.equip;
        if loc == 0 {
            self.send(sid, packets::unequip_ack(idx, 0, false));
            return;
        }
        it.equip = 0;
        self.send(sid, packets::unequip_ack(idx, loc, true));
        self.after_equip_change(unit);
    }

    fn after_equip_change(&mut self, unit: u32) {
        let Some(c) = self.char_of_unit(unit).cloned() else { return };
        let view = self.char_view(&c);
        if let Some(p) = self.world.units.get_mut(&unit).and_then(|u| u.player_mut()) {
            p.view = view;
        }
        // 0x01d7 type 2 = weapon + shield, 3/4/5 = headgear (Athena clif_changelook)
        self.area_send(unit, packets::look2(unit, 2, view.weapon, view.shield), true);
        self.area_send(unit, packets::look(unit, 4, view.head_top as u8), true);
        self.area_send(unit, packets::look(unit, 5, view.head_mid as u8), true);
        self.area_send(unit, packets::look(unit, 3, view.head_bottom as u8), true);
        self.refresh_status(unit);
        // a status change that needs the weapon ends with it (pc_checkallowskill)
        self.defer_rule("equip_changed", serde_json::json!({"bl": unit}));
        self.run_deferred();
    }

    /// The arrow in the quiver, for a client that just loaded (0x013c).
    pub(crate) fn send_arrow(&mut self, unit: u32) {
        let Some(sid) = self.sid_of(unit) else { return };
        let slot = self.char_of_unit(unit).and_then(|c| c.inventory.iter().position(|i| i.equip & 0x8000 != 0));
        if let Some(i) = slot {
            self.send(sid, packets::arrow_equip(i as u16 + INV_OFFSET));
        }
    }

    // ------------------------------------------------------------ identify, cards

    /// 0x0177: what a Magnifier or Item Appraisal can identify.
    pub(crate) fn send_identify_list(&mut self, unit: u32) {
        let Some(sid) = self.sid_of(unit) else { return };
        let idx: Vec<u16> = self
            .char_of_unit(unit)
            .map(|c| c.inventory.iter().enumerate().filter(|(_, i)| !i.identify).map(|(i, _)| i as u16 + INV_OFFSET).collect())
            .unwrap_or_default();
        self.send(sid, packets::identify_list(&idx));
    }

    /// 0x0178: the item picked from that list.
    pub(crate) fn identify_item(&mut self, unit: u32, idx: u16) {
        let Some(sid) = self.sid_of(unit) else { return };
        let i = idx.wrapping_sub(INV_OFFSET) as usize;
        let ok = self.char_of_unit_mut(unit).and_then(|c| c.inventory.get_mut(i)).filter(|it| !it.identify).map(|it| it.identify = true).is_some();
        self.send(sid, packets::identified(idx, if ok { 0 } else { 1 }));
    }

    /// Can the card at `card` go into the equipment at `equip`? (mod0631 clif_use_card)
    fn card_fits(&self, unit: u32, card: usize, equip: usize) -> bool {
        let Some(c) = self.char_of_unit(unit) else { return false };
        let (Some(ci), Some(ei)) = (c.inventory.get(card), c.inventory.get(equip)) else { return false };
        let (Some(cd), Some(ed)) = (self.db.item(ci.nameid), self.db.item(ei.nameid)) else { return false };
        cd.kind == 6
            && matches!(ed.kind, 4 | 5)
            && ei.identify
            && ei.equip == 0
            && ei.cards[0] != 0x00ff
            && ei.cards[0] != 0xff00
            && ed.loc & cd.loc != 0
            && !(ed.kind == 4 && cd.loc == 32)
            && ei.cards.iter().take(ed.slots as usize).any(|c| *c == 0)
    }

    /// 0x017a: a card was double-clicked; list where it can go (0x017b).
    pub(crate) fn card_use(&mut self, unit: u32, idx: u16) {
        let Some(sid) = self.sid_of(unit) else { return };
        let card = idx.wrapping_sub(INV_OFFSET) as usize;
        let n = self.char_of_unit(unit).map(|c| c.inventory.len()).unwrap_or(0);
        let list: Vec<u16> = (0..n).filter(|&e| self.card_fits(unit, card, e)).map(|e| e as u16 + INV_OFFSET).collect();
        self.send(sid, packets::card_targets(&list));
    }

    /// 0x017c: put the card in (mod0631 pc_insert_card).
    pub(crate) fn card_insert(&mut self, unit: u32, card_idx: u16, equip_idx: u16) {
        let Some(sid) = self.sid_of(unit) else { return };
        let (card, equip) = (card_idx.wrapping_sub(INV_OFFSET) as usize, equip_idx.wrapping_sub(INV_OFFSET) as usize);
        if !self.card_fits(unit, card, equip) {
            self.send(sid, packets::card_inserted(equip_idx, card_idx, 1));
            return;
        }
        let c = self.char_of_unit_mut(unit).unwrap();
        let card_id = c.inventory[card].nameid as u16;
        if let Some(slot) = c.inventory[equip].cards.iter_mut().find(|s| **s == 0) {
            *slot = card_id;
        }
        self.send(sid, packets::card_inserted(equip_idx, card_idx, 0));
        self.remove_item(unit, card, 1);
        // indices may have shifted
        self.send_inventory(unit);
    }

    // ------------------------------------------------------------ floor items

    pub(crate) fn drop_floor_item(&mut self, map: &str, x: u16, y: u16, item: InvItem, animate: bool) -> u32 {
        let id = self.world.new_obj_id();
        let (subx, suby) = (rand::random::<u8>() % 12 + 2, rand::random::<u8>() % 12 + 2);
        let now = self.now();
        let u = Unit {
            id,
            map: map.into(),
            x,
            y,
            dir: 0,
            head_dir: 0,
            speed: 0,
            class: 0,
            name: String::new(),
            state: 0,
            walk: None,
            kind: UnitKind::Item(FloorItemUnit { item: item.clone(), expire_at: now + FLOOR_LIFETIME_MS, subx, suby }),
        };
        self.world.add(u);
        if animate {
            let pkt = packets::floor_item_drop(id, item.nameid, item.identify, x, y, subx, suby, item.amount as u16);
            for pid in self.world.viewers(map, x, y) {
                if let Some(p) = self.world.units.get_mut(&pid).and_then(|u| u.player_mut()) {
                    if p.loaded {
                        p.visible.insert(id);
                        let sid = p.sid;
                        self.send(sid, pkt.clone());
                    }
                }
            }
        } else {
            self.update_watchers(id, false);
        }
        id
    }

    pub(crate) fn pick_up(&mut self, unit: u32, item: u32) {
        let Some((me, it)) = self.world.units.get(&unit).zip(self.world.units.get(&item)) else { return };
        let UnitKind::Item(fi) = &it.kind else { return };
        if me.map != it.map || crate::path::distance(me.x as i32, me.y as i32, it.x as i32, it.y as i32) > 2 {
            return;
        }
        let inv = fi.item.clone();
        if self.add_item(unit, inv) {
            let tick = self.tick32();
            self.area_send(unit, packets::action(unit, item, tick, 0, 0, 0, 0, 1, 0), true);
            self.despawn(item, 0);
        }
    }

    pub(crate) fn drop_item(&mut self, unit: u32, idx: u16, amount: u32) {
        let i = idx.wrapping_sub(INV_OFFSET) as usize;
        let Some(it) = self.char_of_unit(unit).and_then(|c| c.inventory.get(i)).cloned() else { return };
        if it.equip != 0 || amount == 0 || amount > it.amount {
            return;
        }
        let Some((map, x, y)) = self.world.units.get(&unit).map(|u| (u.map.clone(), u.x, u.y)) else { return };
        if self.remove_item(unit, i, amount) {
            let mut dropped = it.clone();
            dropped.amount = amount;
            self.drop_floor_item(&map, x, y, dropped, true);
        }
    }

    pub(crate) fn tick_items(&mut self, now: u64) {
        let expired: Vec<u32> = self
            .world
            .units
            .values()
            .filter(|u| matches!(&u.kind, UnitKind::Item(fi) if fi.expire_at <= now))
            .map(|u| u.id)
            .collect();
        for id in expired {
            self.despawn(id, 0);
        }
    }

    // ------------------------------------------------------------ shops

    /// A shop price as this character pays or gets it (Discount, Overcharge:
    /// rules.buy_price / rules.sell_price).
    fn trade_price(&self, unit: u32, price: u32, buy: bool) -> u32 {
        let Some(c) = self.char_of_unit(unit) else { return price };
        let skills: serde_json::Map<String, serde_json::Value> = c.skills.iter().map(|(k, v)| (k.to_string(), serde_json::json!(v))).collect();
        self.scripts
            .as_ref()
            .and_then(|e| e.rule(if buy { "buy_price" } else { "sell_price" }, &serde_json::json!({"price": price, "skills": skills})).ok().flatten())
            .and_then(|v| crate::status::lua_u64(&v))
            .map(|v| v as u32)
            .unwrap_or(price)
    }

    fn shop_items(&self, npc: u32) -> Vec<(u32, u32)> {
        let Some(UnitKind::Npc(n)) = self.world.units.get(&npc).map(|u| &u.kind) else { return vec![] };
        let crate::world::NpcKind::Shop { def } = n.kind else { return vec![] };
        let Some(s) = self.scripts.as_ref().and_then(|e| e.defs.shops.get(def)) else { return vec![] };
        s.items
            .iter()
            .filter_map(|&(id, price)| self.db.item(id).map(|d| (id, price.unwrap_or(d.buy))))
            .collect()
    }

    pub(crate) fn shop_deal(&mut self, unit: u32, npc: u32, kind: u8) {
        let Some(sid) = self.sid_of(unit) else { return };
        if self.world.units.get(&unit).and_then(|u| u.player()).and_then(|p| p.shop) != Some(npc) {
            return;
        }
        if kind == 0 {
            let list: Vec<(u32, u32, u8, u32)> =
                self.shop_items(npc).into_iter().map(|(id, price)| (price, self.trade_price(unit, price, true), self.client_type(id), id)).collect();
            self.send(sid, packets::shop_buy_list(&list));
        } else {
            let Some(c) = self.char_of_unit(unit) else { return };
            let list: Vec<(u16, u32, u32)> = c
                .inventory
                .iter()
                .enumerate()
                .filter(|(_, it)| it.equip == 0)
                .filter_map(|(i, it)| self.db.item(it.nameid).map(|d| (i as u16 + INV_OFFSET, d.sell_price(), self.trade_price(unit, d.sell_price(), false))))
                .collect();
            self.send(sid, packets::shop_sell_list(&list));
        }
    }

    pub(crate) fn shop_buy(&mut self, unit: u32, list: &[(u32, u32)]) {
        let Some(sid) = self.sid_of(unit) else { return };
        let Some(npc) = self.world.units.get(&unit).and_then(|u| u.player()).and_then(|p| p.shop) else { return };
        let stock = self.shop_items(npc);
        let mut total: u64 = 0;
        for &(id, n) in list {
            match stock.iter().find(|(sid_, _)| *sid_ == id) {
                Some(&(_, price)) => total += self.trade_price(unit, price, true) as u64 * n as u64,
                None => {
                    self.send(sid, packets::shop_buy_result(3));
                    return;
                }
            }
        }
        let zeny = self.char_of_unit(unit).map(|c| c.zeny).unwrap_or(0);
        if total > zeny as u64 {
            self.send(sid, packets::shop_buy_result(1));
            return;
        }
        for &(id, n) in list {
            let equip = self.db.item(id).is_some_and(|d| d.is_equip() && d.kind != 10);
            let ok = if equip {
                (0..n).all(|_| self.add_item(unit, InvItem::new(id, 1)))
            } else {
                self.add_item(unit, InvItem::new(id, n))
            };
            if !ok {
                self.send(sid, packets::shop_buy_result(2));
                break;
            }
            let price = stock.iter().find(|(s, _)| *s == id).map(|(_, p)| self.trade_price(unit, *p, true)).unwrap_or(0);
            if let Some(c) = self.char_of_unit_mut(unit) {
                c.zeny = c.zeny.saturating_sub(price * n);
            }
        }
        let z = self.char_of_unit(unit).map(|c| c.zeny).unwrap_or(0);
        self.send(sid, packets::long_par(crate::status::SP_ZENY, z));
        self.send(sid, packets::shop_buy_result(0));
    }

    pub(crate) fn shop_sell(&mut self, unit: u32, list: &[(u16, u32)]) {
        let Some(sid) = self.sid_of(unit) else { return };
        // sell from the highest index down so earlier indices stay valid
        let mut list = list.to_vec();
        list.sort_by(|a, b| b.0.cmp(&a.0));
        for (idx, n) in list {
            let i = idx.wrapping_sub(INV_OFFSET) as usize;
            let Some(it) = self.char_of_unit(unit).and_then(|c| c.inventory.get(i)).cloned() else { continue };
            let price = self.db.item(it.nameid).map(|d| self.trade_price(unit, d.sell_price(), false)).unwrap_or(0);
            if it.equip == 0 && self.remove_item(unit, i, n) {
                if let Some(c) = self.char_of_unit_mut(unit) {
                    c.zeny = c.zeny.saturating_add(price * n);
                }
            }
        }
        let z = self.char_of_unit(unit).map(|c| c.zeny).unwrap_or(0);
        self.send(sid, packets::long_par(crate::status::SP_ZENY, z));
        self.send(sid, packets::shop_sell_result(0));
    }
}
