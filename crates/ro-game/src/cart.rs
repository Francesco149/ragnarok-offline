//! Pushcart: 100 entries, 8000 weight (athena-817 MAX_CART, battle_athena.conf
//! max_cart_weight, which battle.c:7837 multiplies by 10: weights are in tenths). Rented from Kafras (`set_cart`, cart
//! option bits); the client moves items with 0x0126 / 0x0127 and drops the
//! cart with 0x012a. Cart indices are slot + 2 on the wire, as in Athena.

use crate::game::Game;
use crate::model::InvItem;
use crate::npcrt::CART_BITS;
use crate::packets;

pub const MAX_CART: usize = 100;
pub const MAX_CART_WEIGHT: u32 = 80_000;
const MAX_STACK: u32 = 30000;

impl Game {
    fn cart_on(&self, unit: u32) -> bool {
        self.char_of_unit(unit).is_some_and(|c| c.option & CART_BITS != 0)
    }

    fn cart_weight(&self, unit: u32) -> u32 {
        self.char_of_unit(unit)
            .map(|c| c.cart.iter().map(|i| self.db.item(i.nameid).map(|d| d.weight).unwrap_or(0) * i.amount).sum())
            .unwrap_or(0)
    }

    /// 0x0123 / 0x0122 / 0x0121: the cart's contents and counts.
    pub(crate) fn send_cart(&mut self, unit: u32) {
        let Some(sid) = self.sid_of(unit) else { return };
        let Some(c) = self.char_of_unit(unit) else { return };
        let mut normal = Vec::new();
        let mut equips = Vec::new();
        for (i, it) in c.cart.iter().enumerate() {
            let idx = i as u16 + packets::INV_OFFSET;
            let d = self.db.item(it.nameid);
            let kind = match d.map(|d| d.kind).unwrap_or(3) {
                11 => 2,
                k => k,
            };
            if d.is_some_and(|d| d.is_equip() && d.kind != 10) {
                equips.push((idx, it, kind, d.map(|d| d.loc).unwrap_or(0)));
            } else {
                normal.push((idx, it, kind));
            }
        }
        let n = c.cart.len() as u16;
        let pkts = [packets::cart_items(&normal), packets::cart_equips(&equips)];
        let w = self.cart_weight(unit);
        for p in pkts {
            self.send(sid, p);
        }
        self.send(sid, packets::cart_info(n, MAX_CART as u16, w, MAX_CART_WEIGHT));
    }

    /// 0x0126: inventory (client index) -> cart.
    pub(crate) fn cart_put(&mut self, unit: u32, inv_idx: u16, amount: u32) {
        if !self.cart_on(unit) || self.dialogs.contains_key(&unit) {
            return;
        }
        let slot = inv_idx.wrapping_sub(packets::INV_OFFSET) as usize;
        let Some(it) = self.char_of_unit(unit).and_then(|c| c.inventory.get(slot)).cloned() else { return };
        if amount == 0 || amount > it.amount || it.equip != 0 {
            return;
        }
        let Some(d) = self.db.item(it.nameid).cloned() else { return };
        if self.cart_weight(unit) + d.weight * amount > MAX_CART_WEIGHT {
            return;
        }
        let stackable = !d.is_equip() || d.kind == 10;
        let c = self.char_of_unit_mut(unit).unwrap();
        let pos = if stackable { c.cart.iter().position(|s| s.nameid == it.nameid && s.cards == it.cards) } else { None };
        let idx = match pos {
            Some(i) if c.cart[i].amount + amount <= MAX_STACK => {
                c.cart[i].amount += amount;
                i
            }
            Some(_) => return,
            None if c.cart.len() >= MAX_CART => return,
            None => {
                c.cart.push(InvItem { amount, equip: 0, ..it.clone() });
                c.cart.len() - 1
            }
        };
        let stored = c.cart[idx].clone();
        self.remove_item(unit, slot, amount);
        if let Some(sid) = self.sid_of(unit) {
            self.send(sid, packets::cart_add(idx as u16 + packets::INV_OFFSET, amount, &stored));
        }
        self.send_cart_info(unit);
    }

    /// 0x0127: cart (client index) -> inventory.
    pub(crate) fn cart_take(&mut self, unit: u32, cart_idx: u16, amount: u32) {
        if !self.cart_on(unit) || self.dialogs.contains_key(&unit) {
            return;
        }
        let i = cart_idx.wrapping_sub(packets::INV_OFFSET) as usize;
        let Some(it) = self.char_of_unit(unit).and_then(|c| c.cart.get(i)).cloned() else { return };
        if amount == 0 || amount > it.amount {
            return;
        }
        if !self.add_item(unit, InvItem { amount, ..it.clone() }) {
            return;
        }
        let c = self.char_of_unit_mut(unit).unwrap();
        c.cart[i].amount -= amount;
        let emptied = c.cart[i].amount == 0;
        if emptied {
            c.cart.remove(i);
        }
        if let Some(sid) = self.sid_of(unit) {
            self.send(sid, packets::cart_remove(cart_idx, amount));
        }
        if emptied {
            self.send_cart(unit); // later indices shifted
        } else {
            self.send_cart_info(unit);
        }
    }

    fn send_cart_info(&mut self, unit: u32) {
        let Some(sid) = self.sid_of(unit) else { return };
        let n = self.char_of_unit(unit).map(|c| c.cart.len()).unwrap_or(0) as u16;
        let w = self.cart_weight(unit);
        self.send(sid, packets::cart_info(n, MAX_CART as u16, w, MAX_CART_WEIGHT));
    }
}
