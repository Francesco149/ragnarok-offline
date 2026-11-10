//! Map ("zone") server: packet handlers, spawning units, view management,
//! walking, warps, chat, NPC interaction.

use anyhow::Result;
use ro_proto::{Reader, ids::*};
use serde_json::Value;
use tracing::{debug, info};

use crate::game::{Dialog, Game, State};
use crate::model::Point;
use crate::packets;
use crate::path;
use crate::script::{Ctx, CtxKind, Outcome, Wait};
use crate::world::{AttackIntent, DEFAULT_SPEED, NpcKind, NpcUnit, PlayerUnit, Unit, UnitKind, Walk, in_area};

impl Game {
    pub(crate) fn map_packet(&mut self, sid: u32, pkt: &[u8]) -> Result<()> {
        let mut r = Reader::new(pkt);
        let id = r.id();
        if id == CZ_ENTER {
            let aid = r.u32()?;
            let cid = r.u32()?;
            let l1 = r.u32()?;
            return self.map_enter(sid, aid, cid, l1);
        }
        let Some(State::Map { unit, char_id, .. }) = self.sessions.get(&sid).map(|s| s.state.clone()) else {
            return Ok(());
        };
        match id {
            CZ_NOTIFY_ACTORINIT => self.on_loaded(unit),
            CZ_REQUEST_TIME => self.send(sid, packets::server_tick(self.tick32())),
            CZ_PING => {}
            CZ_REQUEST_MOVE => {
                let (x, y, _) = r.pos3()?;
                // clif_parse_WalkToXY: not while casting, delayed, stunned, snared or hidden
                if !self.can_move(unit) {
                    return Ok(());
                }
                if let Some(p) = self.world.units.get_mut(&unit).and_then(|u| u.player_mut()) {
                    p.attack = None;
                }
                self.walk_to(unit, x, y);
            }
            CZ_REQUEST_CHAT => {
                let text = r.seek(4).str_rest();
                self.on_chat(unit, &text);
            }
            CZ_REQNAME => {
                let target = r.u32()?;
                if let Some(u) = self.world.units.get(&target) {
                    let name = match &u.kind {
                        UnitKind::Mob(m) => self.db.mob(m.mob_id).map(|d| d.display.clone()).unwrap_or_default(),
                        _ => u.name.clone(),
                    };
                    self.send(sid, packets::name_ack(target, &name));
                }
            }
            CZ_CHANGE_DIRECTION => {
                let head = r.u16()?;
                let dir = r.u8()?;
                if let Some(u) = self.world.units.get_mut(&unit) {
                    u.head_dir = head as u8;
                    u.dir = dir;
                }
                self.area_send(unit, packets::change_dir(unit, head, dir), false);
            }
            CZ_REQ_EMOTION => {
                let e = r.u8()?;
                if self.basic_skill_ok(unit, 2, 1) {
                    self.area_send(unit, packets::emotion(unit, e), true);
                }
            }
            CZ_REQUEST_ACT => {
                let target = r.u32()?;
                let action = r.u8()?;
                match action {
                    0 | 7 => {
                        // clif_parse_ActionRequest: no attack during a skill's delay
                        let now = self.now();
                        if self.world.units.get(&unit).and_then(|u| u.player()).is_some_and(|p| p.canact > now) {
                            self.send(sid, packets::skill_fail(1, 0, 4));
                            return Ok(());
                        }
                        self.start_attack(unit, AttackIntent { target, continuous: action == 7 })
                    }
                    2 => {
                        if self.basic_skill_ok(unit, 3, 2) {
                            self.set_sit(unit, true)
                        }
                    }
                    3 => self.set_sit(unit, false),
                    _ => {}
                }
            }
            CZ_CONTACTNPC => {
                let npc = r.u32()?;
                self.npc_click(unit, npc);
            }
            CZ_REQ_NEXT_SCRIPT => self.dialog_answer(unit, Wait::Next, Value::Null),
            CZ_CLOSE_DIALOG => self.dialog_answer(unit, Wait::Close, Value::Null),
            CZ_CHOOSE_MENU => {
                let _npc = r.u32()?;
                let choice = r.u8()?;
                self.dialog_answer(unit, Wait::Menu, Value::from(choice));
            }
            CZ_INPUT_EDITDLG => {
                let _npc = r.u32()?;
                let v = r.u32()? as i32;
                self.dialog_answer(unit, Wait::InputNumber, Value::from(v));
            }
            CZ_INPUT_EDITDLGSTR => {
                let _npc = r.seek(4).u32()?;
                let s = r.str_rest();
                self.dialog_answer(unit, Wait::InputString, Value::from(s));
            }
            CZ_ACK_SELECT_DEALTYPE => {
                let npc = r.u32()?;
                let kind = r.u8()?;
                self.shop_deal(unit, npc, kind);
            }
            CZ_PC_PURCHASE_ITEMLIST => {
                let mut list = Vec::new();
                r.seek(4);
                while r.remaining() >= 4 {
                    let amount = r.u16()?;
                    let id = r.u16()?;
                    list.push((id as u32, amount as u32));
                }
                self.shop_buy(unit, &list);
            }
            CZ_PC_SELL_ITEMLIST => {
                let mut list = Vec::new();
                r.seek(4);
                while r.remaining() >= 4 {
                    let idx = r.u16()?;
                    let amount = r.u16()?;
                    list.push((idx, amount as u32));
                }
                self.shop_sell(unit, &list);
            }
            CZ_ITEM_PICKUP => {
                let item = r.u32()?;
                self.pick_up(unit, item);
            }
            CZ_ITEM_THROW => {
                let idx = r.u16()?;
                let amount = r.u16()?;
                self.drop_item(unit, idx, amount as u32);
            }
            CZ_USE_ITEM => {
                let idx = r.u16()?;
                self.use_item(unit, idx);
            }
            CZ_REQ_WEAR_EQUIP => {
                let idx = r.u16()?;
                let _loc = r.u16()?;
                self.equip(unit, idx);
            }
            CZ_REQ_TAKEOFF_EQUIP => {
                let idx = r.u16()?;
                self.unequip(unit, idx);
            }
            CZ_STATUS_CHANGE => {
                let kind = r.u16()?;
                self.raise_stat(unit, kind);
            }
            CZ_RESTART => {
                let kind = r.u8()?;
                if kind == 0 {
                    self.respawn_player(unit);
                } else {
                    self.leave_world(unit, char_id, 2);
                    if let Some(s) = self.sessions.get_mut(&sid) {
                        s.state = State::Closed;
                    }
                    self.send(sid, packets::restart_ack(true));
                }
            }
            CZ_REQ_QUIT => {
                self.send(sid, packets::quit_ack(0));
            }
            CZ_REQ_USER_COUNT => {
                let n = self.world.all_players().len() as u32;
                self.send(sid, packets::user_count(n));
            }
            CZ_WHISPER => {
                // single player: nobody else to whisper to
                self.send(sid, ro_proto::Writer::new(ZC_ACK_WHISPER).u8(1).finish());
            }
            CZ_UPGRADE_SKILLLEVEL => {
                let skill = r.u16()?;
                self.skill_up(unit, skill);
            }
            CZ_MOVE_ITEM_FROM_BODY_TO_STORE => {
                let idx = r.u16()?;
                let amount = r.u32()?;
                self.storage_put(unit, idx, amount);
            }
            CZ_MOVE_ITEM_FROM_STORE_TO_BODY => {
                let idx = r.u16()?;
                let amount = r.u32()?;
                self.storage_take(unit, idx, amount);
            }
            CZ_CLOSE_STORE => self.storage_close(unit),
            CZ_MOVE_ITEM_FROM_BODY_TO_CART => {
                let idx = r.u16()?;
                let amount = r.u32()?;
                self.cart_put(unit, idx, amount);
            }
            CZ_MOVE_ITEM_FROM_CART_TO_BODY => {
                let idx = r.u16()?;
                let amount = r.u32()?;
                self.cart_take(unit, idx, amount);
            }
            0x019f => {
                let target = r.u32()?;
                self.catch_pet_try(unit, target);
            }
            0x01a1 => {
                let menu = r.u8()?;
                self.pet_menu(unit, menu);
            }
            0x01a5 => {
                let name = r.str_fixed(24)?;
                self.pet_rename(unit, &name);
            }
            0x01a7 => {
                let idx = r.u16()?;
                self.pet_hatch(unit, idx);
            }
            0x01a9 => {
                let e = r.u32()?;
                self.pet_emotion(unit, e);
            }
            CZ_REQ_CARTOFF => {
                // Athena clif_parse_RemoveOption: cart, falcon and peco all go
                self.set_option(unit, 0);
            }
            0x0113 => {
                // use skill on a unit: u16 level, u16 skill, u32 target
                let lv = r.u16()?;
                let skill = r.u16()?;
                let target = r.u32()?;
                self.skill_request(unit, skill, lv, target, 0, 0);
            }
            0x0116 | 0x0190 => {
                // use skill on a cell: u16 level, u16 skill, u16 x, u16 y (0x0190 adds 80 bytes of text)
                let lv = r.u16()?;
                let skill = r.u16()?;
                let x = r.u16()?;
                let y = r.u16()?;
                self.skill_request(unit, skill, lv, 0, x, y);
            }
            0x011b => {
                // Teleport / Warp Portal: the map picked from the list
                let skill = r.u16()?;
                let map = r.str_fixed(16)?;
                let pending = self.world.units.get_mut(&unit).and_then(|u| u.player_mut()).and_then(|p| p.skill_menu.take());
                if let Some((s, lv, x, y)) = pending.filter(|p| p.0 == skill) {
                    self.rule_call("skill_castend_map", serde_json::json!({"src": unit, "skill": s, "lv": lv, "x": x, "y": y, "map": map}));
                }
            }
            0x011d => self.memo_request(unit),
            0x0118 => {
                // stop attacking
                if let Some(p) = self.world.units.get_mut(&unit).and_then(|u| u.player_mut()) {
                    p.attack = None;
                }
            }
            0x0178 => {
                let idx = r.u16()?;
                self.identify_item(unit, idx);
            }
            0x017a => {
                let idx = r.u16()?;
                self.card_use(unit, idx);
            }
            0x017c => {
                let card = r.u16()?;
                let equip = r.u16()?;
                self.card_insert(unit, card, equip);
            }
            0x01ae => {
                // Arrow Crafting: the item picked
                let item = r.u16()?;
                self.rule_call("arrow_create", serde_json::json!({"src": unit, "item": item}));
            }
            0x0193 => {
                // whose char id is this? (the maker's name on a forged weapon)
                let cid = r.u32()?;
                if let Some(name) = self.store.chars.get(&cid).map(|c| c.name.clone()) {
                    self.send(sid, packets::char_name(cid, &name));
                }
            }
            0x018e => {
                // forge / mix: u16 item, 3 x u16 extra materials
                let item = r.u16()?;
                let (m1, m2, m3) = (r.u16()?, r.u16()?, r.u16()?);
                self.rule_call("produce_mix", serde_json::json!({"src": unit, "item": item, "slots": [m1, m2, m3]}));
            }
            other => debug!("map: unhandled {other:#06x} ({} bytes)", pkt.len()),
        }
        Ok(())
    }

    // ------------------------------------------------------------ units

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn spawn_player(&mut self, sid: u32, unit: u32, aid: u32, cid: u32, sex: u8, gm: u8, p: &Point) {
        let c = &self.store.chars[&cid];
        let view = self.char_view(c);
        let u = Unit {
            id: unit,
            map: p.map.clone(),
            x: p.x,
            y: p.y,
            dir: 4,
            head_dir: 0,
            speed: DEFAULT_SPEED,
            class: c.class,
            name: c.name.clone(),
            state: if c.hp == 0 { 1 } else { 0 },
            walk: None,
            kind: UnitKind::Player(Box::new(PlayerUnit {
                sid,
                char_id: cid,
                account_id: aid,
                sex,
                view,
                visible: Default::default(),
                loaded: false,
                attack: None,
                next_attack_at: 0,
                last_regen: self.now(),
                hp_acc: 0,
                sp_acc: 0,
                shop: None,
                storage_open: false,
                catch_class: None,
                pet_unit: None,
                gm_level: gm,
                cast: None,
                canact: 0,
                canmove: 0,
                skill_item: None,
                skill_menu: None,
                inc_hp_acc: 0,
                inc_sp_acc: 0,
                spirit_acc: 0,
                weight_icon: 0,
                died_on: None,
            })),
        };
        self.world.add(u);
        self.populate_map(&p.map);
    }

    /// Client finished loading: send everything it needs to draw the scene.
    fn on_loaded(&mut self, unit: u32) {
        let Some(u) = self.world.units.get_mut(&unit) else { return };
        let first = u.player().is_some_and(|p| !p.loaded);
        if let Some(p) = u.player_mut() {
            p.loaded = true;
        }
        let Some(sid) = self.sid_of(unit) else { return };
        self.send_skill_list(unit);
        self.send_full_status(unit);
        self.send_inventory(unit);
        // Athena clif_parse_LoadEndAck: your own cart/falcon/peco
        if let Some((opt, karma)) = self.char_of_unit(unit).filter(|c| c.option != 0).map(|c| (c.option as u16, c.karma as u8)) {
            self.send(sid, packets::state_change(unit, 0, 0, opt, karma));
        }
        if self.char_of_unit(unit).is_some_and(|c| c.option & crate::npcrt::CART_BITS != 0) {
            self.send_cart(unit);
        }
        self.refresh_status(unit);
        self.send_walls(unit);
        // status changes come along: icons and the option words
        let (opt1, opt2, opt) = self.opts_of(unit);
        if opt1 != 0 || opt2 != 0 {
            self.send(sid, packets::state_change(unit, opt1, opt2, opt, 0));
        }
        let icons: Vec<u16> = self.sc_map(unit).map(|m| m.keys().copied().filter(|k| *k < 64).collect()).unwrap_or_default();
        for k in icons {
            self.send(sid, packets::status_icon(k, unit, true));
        }
        self.send_arrow(unit);
        self.refresh_view(unit, true);
        // the pet out of its egg comes along (login, every map change)
        if self.char_of_unit(unit).is_some_and(|c| c.active_pet != 0) {
            self.spawn_pet(unit);
        }
        if first {
            self.fire_event("login", unit);
        }
        self.fire_event("map_enter", unit);
    }

    pub(crate) fn fire_event(&mut self, event: &str, unit: u32) {
        let Some(engine) = self.scripts.clone() else { return };
        let fns = engine.defs.events.get(event).cloned().unwrap_or_default();
        for f in fns {
            let ctx = Ctx::new(unit, 0, CtxKind::Event);
            if let Err(e) = engine.call(self, ctx, &f, vec![]) {
                self.notice(format!("event {event}: {e:#}"));
            }
        }
    }

    /// Look of a unit as seen by others.
    pub(crate) fn unit_look(&self, u: &Unit) -> packets::UnitLook {
        let mut l = match u.player() {
            Some(p) => match self.store.chars.get(&p.char_id) {
                Some(c) => u.look(Some((c, p.view))),
                None => u.look(None),
            },
            None => u.look(None),
        };
        if u.is_fighter() {
            (l.opt1, l.opt2, l.option) = self.opts_of(u.id);
        }
        l
    }

    /// Packet that makes `u` appear for a viewer.
    pub(crate) fn appear_packet(&self, u: &Unit, fresh: bool) -> Vec<u8> {
        if let crate::world::UnitKind::Item(fi) = &u.kind {
            return packets::floor_item(u.id, fi.item.nameid, fi.item.identify, u.x, u.y, fi.item.amount as u16, fi.subx, fi.suby);
        }
        if let crate::world::UnitKind::Skill(s) = &u.kind {
            return packets::skill_unit(u.id, s.src, u.x, u.y, s.unit_id);
        }
        let look = self.unit_look(u);
        if let Some(w) = &u.walk {
            return packets::move_entry(&look, self.tick32(), w.to.0, w.to.1);
        }
        if fresh { packets::new_entry(&look) } else { packets::stand_entry(&look) }
    }

    /// Recompute what a player sees; send appear/vanish for the difference.
    /// `full` resends everything currently in view (after map load).
    pub(crate) fn refresh_view(&mut self, unit: u32, full: bool) {
        let Some(u) = self.world.units.get(&unit) else { return };
        let Some(p) = u.player() else { return };
        if !p.loaded {
            return;
        }
        let sid = p.sid;
        let now_ids: std::collections::HashSet<u32> = self
            .world
            .area_ids(&u.map, u.x, u.y)
            .into_iter()
            .filter(|&i| i != unit && self.world.units.get(&i).is_some_and(|o| o.shown()))
            .collect();
        let before = p.visible.clone();
        let mut out = Vec::new();
        for id in &now_ids {
            if full || !before.contains(id) {
                if let Some(o) = self.world.units.get(id) {
                    out.push(self.appear_packet(o, false));
                }
            }
        }
        for id in before.difference(&now_ids) {
            match self.world.units.get(id) {
                Some(o) if o.is_item() => out.push(packets::floor_item_gone(*id)),
                Some(o) if o.skill().is_some() => out.push(packets::skill_unit_gone(*id)),
                _ => out.push(packets::vanish(*id, 0)),
            }
        }
        // others start seeing us
        let me = self.world.units.get(&unit).unwrap();
        let mut theirs = Vec::new();
        for id in &now_ids {
            if let Some(o) = self.world.units.get(id) {
                if let Some(op) = o.player() {
                    if op.loaded && !op.visible.contains(&unit) {
                        theirs.push((*id, op.sid, self.appear_packet(me, false)));
                    }
                }
            }
        }
        for (oid, osid, pkt) in theirs {
            self.send(osid, pkt);
            if let Some(op) = self.world.units.get_mut(&oid).and_then(|o| o.player_mut()) {
                op.visible.insert(unit);
            }
        }
        for pkt in out {
            self.send(sid, pkt);
        }
        if let Some(p) = self.world.units.get_mut(&unit).and_then(|u| u.player_mut()) {
            p.visible = now_ids;
        }
    }

    /// After a non-player unit moved/appeared: update every player's view of it.
    pub(crate) fn update_watchers(&mut self, unit: u32, fresh: bool) {
        let Some(u) = self.world.units.get(&unit) else { return };
        if !u.shown() {
            return;
        }
        let map = u.map.clone();
        let players = self.world.players_on(&map);
        for pid in players {
            if pid == unit {
                continue;
            }
            let (sees, sid, loaded, close) = {
                let pu = &self.world.units[&pid];
                let p = pu.player().unwrap();
                (p.visible.contains(&unit), p.sid, p.loaded, in_area(pu, &self.world.units[&unit]))
            };
            if !loaded {
                continue;
            }
            if close && !sees {
                let pkt = self.appear_packet(&self.world.units[&unit], fresh);
                self.send(sid, pkt);
                self.world.units.get_mut(&pid).unwrap().player_mut().unwrap().visible.insert(unit);
            } else if !close && sees {
                let o = &self.world.units[&unit];
                let pkt = if o.is_item() {
                    packets::floor_item_gone(unit)
                } else if o.skill().is_some() {
                    packets::skill_unit_gone(unit)
                } else {
                    packets::vanish(unit, 0)
                };
                self.send(sid, pkt);
                self.world.units.get_mut(&pid).unwrap().player_mut().unwrap().visible.remove(&unit);
            }
        }
    }

    /// Remove a unit and tell everyone who saw it (kind: 0 out of sight, 1 died, 2 logout, 3 teleport).
    pub(crate) fn despawn(&mut self, unit: u32, kind: u8) {
        let Some(u) = self.world.remove(unit) else { return };
        let pkt = if u.is_item() {
            packets::floor_item_gone(unit)
        } else if u.skill().is_some() {
            packets::skill_unit_gone(unit)
        } else {
            packets::vanish(unit, kind)
        };
        let watchers: Vec<(u32, u32)> = self
            .world
            .units
            .values()
            .filter_map(|o| o.player().filter(|p| p.visible.contains(&unit)).map(|p| (o.id, p.sid)))
            .collect();
        for (oid, sid) in watchers {
            self.send(sid, pkt.clone());
            if let Some(p) = self.world.units.get_mut(&oid).and_then(|o| o.player_mut()) {
                p.visible.remove(&unit);
            }
        }
    }

    /// Send to every loaded player who sees `unit` (and to `unit` itself if `include_self`).
    pub(crate) fn area_send(&self, unit: u32, pkt: Vec<u8>, include_self: bool) {
        for o in self.world.units.values() {
            if let Some(p) = o.player() {
                if (o.id == unit && include_self) || (o.id != unit && p.visible.contains(&unit)) {
                    self.send(p.sid, pkt.clone());
                }
            }
        }
    }

    /// Put one NPC definition on its (loaded) map.
    pub(crate) fn place_npc_def(&mut self, def: usize) {
        let Some(n) = self.scripts.as_ref().and_then(|e| e.defs.npcs.get(def)).cloned() else { return };
        let (sprite, hidden) = match n.sprite {
            Some(s) => (s, false),
            None => (111, true),
        };
        let id = self.world.new_npc_id();
        self.world.add(Unit {
            id,
            map: n.map.clone(),
            x: n.x,
            y: n.y,
            dir: n.dir,
            head_dir: 0,
            speed: DEFAULT_SPEED,
            class: sprite,
            name: n.name.clone(),
            state: 0,
            walk: None,
            kind: UnitKind::Npc(NpcUnit { kind: NpcKind::Script { def }, touch: n.touch, hidden }),
        });
        self.update_watchers(id, false);
    }

    /// NPCs, warps and shops declared for `map`.
    pub(crate) fn place_static_units(&mut self, map: &str) {
        let Some(engine) = self.scripts.as_ref() else { return };
        let mut new_units = Vec::new();
        for (i, n) in engine.defs.npcs.iter().enumerate() {
            if n.map == map && !self.npcrt.disabled.contains(&n.id) {
                let (sprite, hidden) = match n.sprite {
                    Some(s) => (s, false),
                    None => (111, true),
                };
                new_units.push((n.x, n.y, n.dir, sprite, hidden, n.name.clone(), NpcKind::Script { def: i }, n.touch));
            }
        }
        for w in &engine.defs.warps {
            if w.map == map {
                // 45 = warp portal sprite; touch area = span
                new_units.push((w.x, w.y, 0, 45, false, String::new(), NpcKind::Warp { to: w.to.clone() }, Some(w.span)));
            }
        }
        for (i, s) in engine.defs.shops.iter().enumerate() {
            if s.map == map {
                new_units.push((s.x, s.y, s.dir, s.sprite, false, s.name.clone(), NpcKind::Shop { def: i }, None));
            }
        }
        for (x, y, dir, sprite, hidden, name, kind, touch) in new_units {
            let id = self.world.new_npc_id();
            self.world.add(Unit {
                id,
                map: map.to_string(),
                x,
                y,
                dir,
                head_dir: 0,
                speed: DEFAULT_SPEED,
                class: sprite,
                name,
                state: 0,
                walk: None,
                kind: UnitKind::Npc(NpcUnit { kind, touch, hidden }),
            });
            self.update_watchers(id, false);
        }
    }

    // ------------------------------------------------------------ movement

    pub(crate) fn walk_to(&mut self, unit: u32, x: u16, y: u16) -> bool {
        let Some(u) = self.world.units.get_mut(&unit) else { return false };
        if u.state == 1 {
            return false;
        }
        if u.state == 2 {
            u.state = 0;
        }
        if let Some(w) = &mut u.walk {
            // mid-walk: change course at the next cell, like Athena
            w.retarget = Some((x, y));
            return true;
        }
        let (map, x0, y0) = (u.map.clone(), u.x, u.y);
        self.start_walk(unit, &map, x0, y0, x, y)
    }

    fn start_walk(&mut self, unit: u32, map: &str, x0: u16, y0: u16, x: u16, y: u16) -> bool {
        let Some(grid) = self.grid(map) else { return false };
        let Some(dirs) = path::find(&grid, x0 as i32, y0 as i32, x as i32, y as i32) else { return false };
        if dirs.is_empty() {
            return false;
        }
        let now = self.now();
        let tick = self.tick32();
        let u = self.world.units.get_mut(&unit).unwrap();
        let first = step_cost(u.speed, dirs[0]) / 2;
        u.walk = Some(Walk { path: dirs, pos: 0, next_at: now + first as u64, to: (x, y), retarget: None });
        if let Some(p) = u.player() {
            let sid = p.sid;
            self.send(sid, packets::player_move(tick, x0, y0, x, y));
        }
        self.area_send(unit, packets::unit_move(unit, x0, y0, x, y, tick), false);
        // watchers that don't see it yet will get a move entry on the next step
        true
    }

    pub(crate) fn tick_walks(&mut self, now: u64) {
        let due: Vec<u32> = self
            .world
            .units
            .values()
            .filter(|u| u.walk.as_ref().is_some_and(|w| w.next_at <= now))
            .map(|u| u.id)
            .collect();
        for id in due {
            // a unit may take several steps if the server stalled
            for _ in 0..4 {
                let still_due = self.world.units.get(&id).and_then(|u| u.walk.as_ref()).is_some_and(|w| w.next_at <= now);
                if !still_due || !self.step(id) {
                    break;
                }
            }
        }
    }

    /// Advance one cell. Returns false when the walk ended.
    fn step(&mut self, id: u32) -> bool {
        let Some(u) = self.world.units.get_mut(&id) else { return false };
        let Some(w) = u.walk.as_mut() else { return false };
        let d = w.path[w.pos];
        let (dx, dy) = path::DIRS[d as usize];
        u.x = (u.x as i32 + dx) as u16;
        u.y = (u.y as i32 + dy) as u16;
        u.dir = d;
        w.pos += 1;
        let cur_cost = step_cost(u.speed, d);
        let retarget = w.retarget.take();
        let done = w.pos >= w.path.len();
        let next_cost = if done { 0 } else { step_cost(u.speed, w.path[w.pos]) };
        w.next_at += (cur_cost / 2 + next_cost / 2) as u64;
        let (map, x, y, is_player) = (u.map.clone(), u.x, u.y, u.is_player());
        if done || retarget.is_some() {
            u.walk = None;
        }
        if is_player {
            self.refresh_view(id, false);
            if self.check_touch(id, &map, x, y) {
                return false; // warped, or stopped by a touch NPC
            }
        } else {
            self.update_watchers(id, false);
        }
        // Athena skill_unit_move: ground skills notice who walks in and out
        if self.world.units.get(&id).is_some_and(|u| u.is_fighter()) && self.has_skill_cells(&map) {
            self.rule_call("skill_unit_move", serde_json::json!({"bl": id}));
            // stopped, thrown or sent elsewhere by what it stepped on
            let ended = done || retarget.is_some();
            if self.world.units.get(&id).is_none_or(|u| u.map != map || (u.x, u.y) != (x, y) || (!ended && u.walk.is_none())) || !self.can_move(id) {
                return false;
            }
        }
        if let Some((tx, ty)) = retarget {
            if (tx, ty) != (x, y) {
                self.start_walk(id, &map, x, y, tx, ty);
            }
            return false;
        }
        if done {
            self.arrived(id);
            return false;
        }
        true
    }

    /// Called when a walk finishes (attack-move continues here).
    fn arrived(&mut self, id: u32) {
        if let Some(intent) = self.world.units.get(&id).and_then(|u| u.player()).and_then(|p| p.attack) {
            self.start_attack(id, intent);
        }
    }

    /// Warp portals and OnTouch NPCs under the player's feet.
    fn check_touch(&mut self, unit: u32, map: &str, x: u16, y: u16) -> bool {
        let hit = self
            .world
            .maps
            .get(map)
            .map(|m| {
                m.units
                    .iter()
                    .filter_map(|id| self.world.units.get(id))
                    .find(|n| match &n.kind {
                        UnitKind::Npc(NpcUnit { touch: Some((xs, ys)), .. }) => {
                            (n.x as i32 - x as i32).abs() <= *xs as i32 && (n.y as i32 - y as i32).abs() <= *ys as i32
                        }
                        _ => false,
                    })
                    .map(|n| (n.id, n.kind.clone()))
            })
            .flatten();
        match hit {
            Some((_, UnitKind::Npc(NpcUnit { kind: NpcKind::Warp { to }, .. }))) => {
                self.warp_player(unit, &to.0, to.1, to.2);
                true
            }
            Some((npc, UnitKind::Npc(NpcUnit { kind: NpcKind::Script { def }, .. }))) => {
                let f = self.scripts.as_ref().and_then(|e| e.defs.npcs.get(def)).and_then(|d| d.on_touch.clone());
                // Athena: no new script while one runs (npc_click: npc_id != 0)
                // Athena npc_touch_areanpc: the walk stops on the first cell of the area
                if let Some(u) = self.world.units.get_mut(&unit) {
                    u.walk = None;
                }
                self.area_send(unit, packets::stop_move(unit, x, y), true);
                if let (Some(f), false) = (f, self.dialogs.contains_key(&unit)) {
                    self.start_dialog(unit, npc, def, &f, CtxKind::Touch, None);
                }
                true
            }
            _ => false,
        }
    }

    /// Move a player to another map / cell (x,y 0 = random walkable cell).
    pub(crate) fn warp_player(&mut self, unit: u32, map: &str, x: u16, y: u16) -> bool {
        let map = map.to_ascii_lowercase();
        if !self.ensure_map(&map) {
            self.notice(format!("warp: map {map} not in client data"));
            return false;
        }
        let grid = self.grid(&map).unwrap();
        let (mut x, mut y) = (x, y);
        if (x == 0 && y == 0) || !grid.walkable(x as i32, y as i32) {
            match random_cell(&grid) {
                Some((rx, ry)) => (x, y) = (rx, ry),
                None => return false,
            }
        }
        let Some(u) = self.world.units.get(&unit) else { return false };
        let Some(p) = u.player() else { return false };
        let (sid, cid, aid, sex, gm) = (p.sid, p.char_id, p.account_id, p.sex, p.gm_level);
        let same_map = u.map == map;
        self.dialogs.remove(&unit);
        self.despawn(unit, 3);
        if let Some(c) = self.store.chars.get_mut(&cid) {
            c.last_point = Point::new(&map, x, y);
        }
        self.spawn_player(sid, unit, aid, cid, sex, gm, &Point::new(&map, x, y));
        if same_map {
            // same map: client keeps the scene; resend view immediately
            if let Some(p) = self.world.units.get_mut(&unit).and_then(|u| u.player_mut()) {
                p.loaded = true;
            }
            self.send(sid, packets::map_move(&map, x, y));
            self.refresh_view(unit, true);
        } else {
            // client loads the new map, then sends 0x007d
            self.send(sid, packets::map_move(&map, x, y));
        }
        info!("warp {unit} -> {map} {x},{y}");
        true
    }

    /// Any ground skill cells on this map?
    pub(crate) fn has_skill_cells(&self, map: &str) -> bool {
        self.world.maps.get(map).is_some_and(|m| m.units.iter().any(|i| self.world.units.get(i).is_some_and(|u| u.skill().is_some())))
    }

    /// /memo (0x011d): remember this cell for Warp Portal. The rule says how
    /// many points the character may keep (mod0631 pc_memo: the newest first,
    /// a map already remembered is replaced).
    fn memo_request(&mut self, unit: u32) {
        let r = self.rule_call("memo", serde_json::json!({"src": unit}));
        let Some(sid) = self.sid_of(unit) else { return };
        let Some(keep) = r.as_ref().and_then(|v| v.as_u64()).filter(|k| *k > 0) else {
            self.send(sid, packets::skill_memo(1));
            return;
        };
        let Some(pt) = self.world.units.get(&unit).map(|u| Point::new(&u.map, u.x, u.y)) else { return };
        if let Some(c) = self.char_of_unit_mut(unit) {
            c.memo.retain(|p| p.map != pt.map);
            c.memo.insert(0, pt);
            c.memo.truncate(keep.min(3) as usize);
        }
        self.send(sid, packets::skill_memo(0));
    }

    fn set_sit(&mut self, unit: u32, sit: bool) {
        let Some(u) = self.world.units.get_mut(&unit) else { return };
        if u.state == 1 || u.walk.is_some() {
            return;
        }
        u.state = if sit { 2 } else { 0 };
        let tick = self.tick32();
        self.area_send(unit, packets::action(unit, 0, tick, 0, 0, 0, 0, if sit { 2 } else { 3 }, 0), true);
    }

    // ------------------------------------------------------------ chat & commands

    fn on_chat(&mut self, unit: u32, text: &str) {
        let Some(u) = self.world.units.get(&unit) else { return };
        let Some(sid) = u.player().map(|p| p.sid) else { return };
        // the client sends "Name : message"
        let msg = text.split_once(" : ").map(|(_, m)| m).unwrap_or(text).trim().to_string();
        if let Some(cmd) = msg.strip_prefix('@') {
            self.run_command(unit, cmd);
            return;
        }
        self.send(sid, packets::self_chat(text));
        self.area_send(unit, packets::unit_chat(unit, text), false);
    }

    fn run_command(&mut self, unit: u32, line: &str) {
        let (name, args) = line.split_once(' ').unwrap_or((line, ""));
        let name = name.to_ascii_lowercase();
        let gm = self.world.units.get(&unit).and_then(|u| u.player()).map(|p| p.gm_level).unwrap_or(0);
        if name == "reload" && gm > 0 {
            let msg = match self.reload_scripts() {
                Ok(m) => m,
                Err(e) => format!("reload failed: {e:#}"),
            };
            self.tell(unit, &msg);
            return;
        }
        let f = self.scripts.as_ref().and_then(|e| e.defs.commands.get(&name).cloned());
        match f {
            Some(f) if gm > 0 => {
                let Some(engine) = self.scripts.clone() else { return };
                let ctx = Ctx::new(unit, 0, CtxKind::Command);
                let r = engine.call(self, ctx, &f, vec![Value::from(args)]);
                match r {
                    Ok(Value::String(s)) => self.tell(unit, &s),
                    Ok(_) => {}
                    Err(e) => {
                        let msg = format!("@{name}: {e:#}");
                        self.tell(unit, &msg);
                        self.notice(msg);
                    }
                }
            }
            Some(_) => self.tell(unit, "You are not allowed to use commands."),
            None => self.tell(unit, &format!("Unknown command: @{name}")),
        }
    }

    /// A chat line only this player sees.
    pub(crate) fn tell(&self, unit: u32, text: &str) {
        if let Some(sid) = self.sid_of(unit) {
            for line in text.lines() {
                self.send(sid, packets::self_chat(line));
            }
        }
    }

    // ------------------------------------------------------------ NPC dialogs

    fn npc_click(&mut self, unit: u32, npc: u32) {
        if self.dialogs.contains_key(&unit) {
            return;
        }
        let Some((n, me)) = self.world.units.get(&npc).zip(self.world.units.get(&unit)) else { return };
        if n.map != me.map || path::distance(n.x as i32, n.y as i32, me.x as i32, me.y as i32) > crate::world::AREA {
            return;
        }
        match &n.kind {
            UnitKind::Npc(NpcUnit { kind: NpcKind::Script { def }, hidden: false, .. }) => {
                let def = *def;
                let f = self.scripts.as_ref().and_then(|e| e.defs.npcs.get(def)).and_then(|d| d.on_click.clone());
                if let Some(f) = f {
                    self.start_dialog(unit, npc, def, &f, CtxKind::Dialog, None);
                }
            }
            UnitKind::Npc(NpcUnit { kind: NpcKind::Shop { .. }, .. }) => {
                if let Some(p) = self.world.units.get_mut(&unit).and_then(|u| u.player_mut()) {
                    p.shop = Some(npc);
                    let sid = p.sid;
                    self.send(sid, packets::shop_select(npc));
                }
            }
            _ => {}
        }
    }

    /// Run an NPC script for a player as a dialog; `at` = event label to enter at.
    pub(crate) fn start_dialog(&mut self, unit: u32, npc: u32, def: usize, f: &mlua::Function, kind: CtxKind, at: Option<&str>) {
        let Some(engine) = self.scripts.clone() else { return };
        let ctx = Ctx { player: unit, npc, kind, def: def as u32 };
        self.dialogs.remove(&unit);
        self.dialog_open.insert(unit, false);
        let r = engine.start(self, ctx, f, at);
        match r {
            Ok((thread, outcome)) => self.after_resume(unit, npc, def as u32, thread, outcome),
            Err(e) => self.script_failed(unit, npc, e),
        }
    }

    fn dialog_answer(&mut self, unit: u32, got: Wait, answer: Value) {
        let Some(d) = self.dialogs.remove(&unit) else { return };
        if d.wait != got {
            // e.g. close clicked while a menu was up: end the dialog
            if got == Wait::Close || answer == Value::from(255u8) {
                return;
            }
            self.dialogs.insert(unit, d);
            return;
        }
        if got == Wait::Close {
            if let Some(sid) = self.sid_of(unit) {
                self.send(sid, packets::cutin("", 255));
            }
        }
        if got == Wait::Menu && answer == Value::from(255u8) {
            // cancel = end the conversation (Athena behaviour)
            if let Some(sid) = self.sid_of(unit) {
                self.send(sid, packets::cutin("", 255));
            }
            return;
        }
        let Some(engine) = self.scripts.clone() else { return };
        let ctx = Ctx { player: unit, npc: d.npc, kind: CtxKind::Dialog, def: d.def };
        self.dialog_open.insert(unit, d.open && got != Wait::Close);
        let r = engine.resume(self, ctx, &d.thread, answer);
        match r {
            Ok(o) => self.after_resume(unit, d.npc, d.def, d.thread, o),
            Err(e) => self.script_failed(unit, d.npc, e),
        }
    }

    fn after_resume(&mut self, unit: u32, npc: u32, def: u32, thread: mlua::Thread, o: Outcome) {
        let open = self.dialog_open.remove(&unit).unwrap_or(false);
        match o {
            Outcome::Suspended(wait) => {
                self.dialogs.insert(unit, Dialog { thread, npc, def, wait, open: open && wait != Wait::Close });
            }
            Outcome::Done(_) => {
                if open {
                    // script ended with text showing: give the player a close button
                    if let Some(sid) = self.sid_of(unit) {
                        self.send(sid, packets::npc_close(npc));
                    }
                }
            }
        }
    }

    fn script_failed(&mut self, unit: u32, npc: u32, e: anyhow::Error) {
        let msg = format!("script error (npc {npc}): {e:#}");
        self.notice(msg.clone());
        if let Some(sid) = self.sid_of(unit) {
            self.send(sid, packets::npc_mes(npc, "^FF0000[script error - see server log]^000000"));
            self.send(sid, packets::npc_close(npc));
        }
        self.dialogs.remove(&unit);
        self.dialog_open.remove(&unit);
    }
}

pub(crate) fn step_cost(speed: u16, dir: u8) -> u32 {
    if path::is_diagonal(dir) { speed as u32 * 14 / 10 } else { speed as u32 }
}

pub(crate) fn random_cell(g: &ro_data::MapGrid) -> Option<(u16, u16)> {
    for _ in 0..1000 {
        let x = rand::random::<u16>() % g.width;
        let y = rand::random::<u16>() % g.height;
        if g.walkable(x as i32, y as i32) {
            return Some((x, y));
        }
    }
    None
}
