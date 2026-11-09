//! Kafra storage: one per account, 100 entries (athena-817 MAX_STORAGE,
//! storage.c). Opened by scripts (`open_storage()`); the client then moves
//! items with 0x00f3 / 0x00f5 and closes with 0x00f7. Storage indices are
//! 1-based on the wire, as Athena sends them.

use crate::game::Game;
use crate::model::InvItem;
use crate::packets;

pub const MAX_STORAGE: usize = 100;
const MAX_STACK: u32 = 30000;

impl Game {
    fn account_of_unit(&self, unit: u32) -> Option<u32> {
        self.world.units.get(&unit).and_then(|u| u.player()).map(|p| p.account_id)
    }

    fn send_storage(&mut self, unit: u32) {
        let (Some(sid), Some(aid)) = (self.sid_of(unit), self.account_of_unit(unit)) else { return };
        let Some(acc) = self.store.accounts.get(&aid) else { return };
        let mut normal = Vec::new();
        let mut equips = Vec::new();
        for (i, it) in acc.storage.iter().enumerate() {
            let idx = i as u16 + 1;
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
        let pkts = [
            packets::storage_items(&normal),
            packets::storage_equips(&equips),
            packets::storage_info(acc.storage.len() as u16, MAX_STORAGE as u16),
        ];
        for p in pkts {
            self.send(sid, p);
        }
    }

    pub(crate) fn open_storage(&mut self, unit: u32) {
        if let Some(p) = self.world.units.get_mut(&unit).and_then(|u| u.player_mut()) {
            p.storage_open = true;
        }
        self.send_storage(unit);
    }

    fn storage_is_open(&self, unit: u32) -> bool {
        self.world.units.get(&unit).and_then(|u| u.player()).is_some_and(|p| p.storage_open)
    }

    /// 0x00f3: inventory index (client numbering) -> storage.
    pub(crate) fn storage_put(&mut self, unit: u32, inv_idx: u16, amount: u32) {
        if !self.storage_is_open(unit) {
            return;
        }
        let Some(aid) = self.account_of_unit(unit) else { return };
        let slot = inv_idx.wrapping_sub(packets::INV_OFFSET) as usize;
        let Some(it) = self.char_of_unit(unit).and_then(|c| c.inventory.get(slot)).cloned() else { return };
        if amount == 0 || amount > it.amount || it.equip != 0 {
            return;
        }
        let stackable = self.db.item(it.nameid).is_some_and(|d| !d.is_equip() || d.kind == 10);
        let Some(acc) = self.store.accounts.get_mut(&aid) else { return };
        let pos = if stackable { acc.storage.iter().position(|s| s.nameid == it.nameid && s.cards == it.cards) } else { None };
        let idx = match pos {
            Some(i) if acc.storage[i].amount + amount <= MAX_STACK => {
                acc.storage[i].amount += amount;
                i
            }
            Some(_) => return,
            None if acc.storage.len() >= MAX_STORAGE => return,
            None => {
                acc.storage.push(InvItem { amount, equip: 0, ..it.clone() });
                acc.storage.len() - 1
            }
        };
        let stored = acc.storage[idx].clone();
        let n = acc.storage.len();
        self.remove_item(unit, slot, amount);
        if let Some(sid) = self.sid_of(unit) {
            self.send(sid, packets::storage_add(idx as u16 + 1, amount, &stored));
            self.send(sid, packets::storage_info(n as u16, MAX_STORAGE as u16));
        }
    }

    /// 0x00f5: storage index (1-based) -> inventory.
    pub(crate) fn storage_take(&mut self, unit: u32, idx: u16, amount: u32) {
        if !self.storage_is_open(unit) {
            return;
        }
        let Some(aid) = self.account_of_unit(unit) else { return };
        let i = idx.wrapping_sub(1) as usize;
        let Some(it) = self.store.accounts.get(&aid).and_then(|a| a.storage.get(i)).cloned() else { return };
        if amount == 0 || amount > it.amount {
            return;
        }
        if !self.add_item(unit, InvItem { amount, ..it.clone() }) {
            return;
        }
        let Some(acc) = self.store.accounts.get_mut(&aid) else { return };
        acc.storage[i].amount -= amount;
        let emptied = acc.storage[i].amount == 0;
        if emptied {
            acc.storage.remove(i);
        }
        if let Some(sid) = self.sid_of(unit) {
            self.send(sid, packets::storage_remove(idx, amount));
        }
        if emptied {
            // later indices shifted: resend the lot so the client agrees
            self.send_storage(unit);
        }
    }

    /// 0x00f7: close (and save, like Athena's storage_storageclose).
    pub(crate) fn storage_close(&mut self, unit: u32) {
        if let Some(p) = self.world.units.get_mut(&unit).and_then(|u| u.player_mut()) {
            p.storage_open = false;
        }
        if let Some(aid) = self.account_of_unit(unit) {
            if let Err(e) = self.store.save_account(aid) {
                tracing::warn!("save storage of {aid}: {e:#}");
            }
        }
        if let Some(sid) = self.sid_of(unit) {
            self.send(sid, packets::storage_closed());
        }
    }
}
