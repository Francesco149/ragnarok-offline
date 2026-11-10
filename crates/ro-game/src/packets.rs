//! Server -> client packet builders for the 2003-12-18 client.
//!
//! Layouts: the classic Athena layouts, cross-checked against the client's
//! receive code where noted ([client] = read out of Ragexe.exe, see
//! docs/PROTOCOL.md). Every builder goes through `Writer::finish`, which
//! asserts the length against the client's own table.

use ro_proto::Writer;
use ro_proto::ids::*;

use crate::model::{Character, InvItem};

/// What other clients need to draw a unit.
#[derive(Debug, Clone, Default)]
pub struct UnitLook {
    pub id: u32,
    pub speed: u16,
    pub opt1: u16,
    pub opt2: u16,
    pub option: u16,
    pub class: u16,
    pub hair: u16,
    pub weapon: u16,
    pub head_bottom: u16,
    pub shield: u16,
    pub head_top: u16,
    pub head_mid: u16,
    pub hair_color: u16,
    pub clothes_color: u16,
    pub head_dir: u16,
    pub guild_id: u32,
    pub emblem_id: u32,
    pub manner: u16,
    pub karma: u8,
    pub sex: u8,
    pub x: u16,
    pub y: u16,
    pub dir: u8,
    /// 0 standing, 1 dead, 2 sitting
    pub state: u8,
}

fn look_common(w: &mut Writer, u: &UnitLook) {
    w.u32(u.id).u16(u.speed).u16(u.opt1).u16(u.opt2).u16(u.option).u16(u.class).u16(u.hair).u16(u.weapon).u16(u.head_bottom);
}

/// 0x0078 (52 bytes in this client = Athena's 54-byte layout without the
/// trailing u16 level, which kRO added in 2004).
pub fn stand_entry(u: &UnitLook) -> Vec<u8> {
    let mut w = Writer::new(ZC_NOTIFY_STANDENTRY);
    look_common(&mut w, u);
    w.u16(u.shield).u16(u.head_top).u16(u.head_mid).u16(u.hair_color).u16(u.clothes_color).u16(u.head_dir);
    w.u32(u.guild_id).u32(u.emblem_id).u16(u.manner).u8(u.karma).u8(u.sex);
    w.pos3(u.x, u.y, u.dir).u8(5).u8(5).u8(u.state);
    w.finish()
}

/// 0x0079 (51): a unit that just appeared (spawn/warp in).
pub fn new_entry(u: &UnitLook) -> Vec<u8> {
    let mut w = Writer::new(ZC_NOTIFY_NEWENTRY);
    look_common(&mut w, u);
    w.u16(u.shield).u16(u.head_top).u16(u.head_mid).u16(u.hair_color).u16(u.clothes_color).u16(u.head_dir);
    w.u32(u.guild_id).u32(u.emblem_id).u16(u.manner).u8(u.karma).u8(u.sex);
    w.pos3(u.x, u.y, u.dir).u8(5).u8(5);
    w.finish()
}

/// 0x007b (58): a unit seen while walking.
pub fn move_entry(u: &UnitLook, tick: u32, to_x: u16, to_y: u16) -> Vec<u8> {
    let mut w = Writer::new(ZC_NOTIFY_MOVEENTRY);
    look_common(&mut w, u);
    w.u32(tick);
    w.u16(u.shield).u16(u.head_top).u16(u.head_mid).u16(u.hair_color).u16(u.clothes_color).u16(u.head_dir);
    w.u32(u.guild_id).u32(u.emblem_id).u16(u.manner).u8(u.karma).u8(u.sex);
    w.move6(u.x, u.y, to_x, to_y).u8(5).u8(5);
    w.finish()
}

/// 0x0080: unit leaves view. type: 0 out of sight, 1 died, 2 logged out, 3 teleported.
pub fn vanish(id: u32, kind: u8) -> Vec<u8> {
    Writer::new(ZC_NOTIFY_VANISH).u32(id).u8(kind).finish()
}

/// 0x0086: another unit moves.
pub fn unit_move(id: u32, x0: u16, y0: u16, x1: u16, y1: u16, tick: u32) -> Vec<u8> {
    Writer::new(ZC_NOTIFY_MOVE).u32(id).move6(x0, y0, x1, y1).u32(tick).finish()
}

/// 0x0087: your own move was accepted.
pub fn player_move(tick: u32, x0: u16, y0: u16, x1: u16, y1: u16) -> Vec<u8> {
    Writer::new(ZC_NOTIFY_PLAYERMOVE).u32(tick).move6(x0, y0, x1, y1).finish()
}

/// 0x0088: snap a unit to a cell (stop).
pub fn stop_move(id: u32, x: u16, y: u16) -> Vec<u8> {
    Writer::new(ZC_STOPMOVE).u32(id).u16(x).u16(y).finish()
}

/// 0x0069 [client: ids at +4/+8/+12, sex at +46, 32-byte server records from +47]
pub fn accept_login(login1: u32, account_id: u32, login2: u32, sex: u8, servers: &[(u32, u16, &str, u16)]) -> Vec<u8> {
    let mut w = Writer::new(AC_ACCEPT_LOGIN);
    w.u32(login1).u32(account_id).u32(login2).u32(0).str_fixed("", 24).u16(0).u8(sex);
    for (ip, port, name, users) in servers {
        w.u32(*ip).u16(*port).str_fixed(name, 20).u16(*users).u16(0).u16(0);
    }
    w.finish()
}

/// 0x006a: 0 unregistered id, 1 wrong password, 2 expired, 3 rejected, 4 blocked, 5 old client ...
pub fn refuse_login(code: u8) -> Vec<u8> {
    Writer::new(AC_REFUSE_LOGIN).u8(code).str_fixed("", 20).finish()
}

/// The 106-byte character record [client: record size 106, slot at +104,
/// class at +52].
pub fn char_record(w: &mut Writer, c: &Character, view: &CharView, max_hp: u32, max_sp: u32) {
    let cl = |v: u64| v.min(i32::MAX as u64) as u32;
    w.u32(c.id).u32(cl(c.base_exp)).u32(c.zeny).u32(cl(c.job_exp)).u32(c.job_level as u32);
    w.u32(0).u32(0).u32(c.option).u32(c.karma).u32(c.manner as u32);
    w.u16(c.status_point).u16(c.hp.min(32767) as u16).u16(max_hp.min(32767) as u16);
    w.u16(c.sp.min(32767) as u16).u16(max_sp.min(32767) as u16);
    w.u16(150).u16(c.class).u16(c.hair).u16(view.weapon).u16(c.base_level).u16(c.skill_point);
    w.u16(view.head_bottom).u16(view.shield).u16(view.head_top).u16(view.head_mid);
    w.u16(c.hair_color).u16(c.clothes_color).str_fixed(&c.name, 24);
    for s in c.stats {
        w.u8(s.min(255) as u8);
    }
    w.u8(c.slot).u8(0);
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CharView {
    pub weapon: u16,
    pub shield: u16,
    pub head_top: u16,
    pub head_mid: u16,
    pub head_bottom: u16,
}

pub struct CharEntry<'a> {
    pub ch: &'a Character,
    pub view: CharView,
    pub max_hp: u32,
    pub max_sp: u32,
}

/// 0x006b: header u16 id, u16 len, 20 reserved bytes, then 106-byte records.
/// [client: count = (len-4)/106, records start at 4 + (len-4)%106, so the
/// Athena-era 20 reserved bytes are skipped either way]
pub fn char_list(chars: &[CharEntry]) -> Vec<u8> {
    let mut w = Writer::new(HC_ACCEPT_ENTER);
    w.zeros(20);
    for e in chars {
        char_record(&mut w, e.ch, &e.view, e.max_hp, e.max_sp);
    }
    w.finish()
}

pub fn char_created(e: &CharEntry) -> Vec<u8> {
    let mut w = Writer::new(HC_ACCEPT_MAKECHAR);
    char_record(&mut w, e.ch, &e.view, e.max_hp, e.max_sp);
    w.finish()
}

/// 0x006e: 0x00 name exists, 0xff denied.
pub fn char_create_refused(code: u8) -> Vec<u8> {
    Writer::new(HC_REFUSE_MAKECHAR).u8(code).finish()
}

pub fn char_deleted() -> Vec<u8> {
    Writer::new(HC_ACCEPT_DELETECHAR).finish()
}

pub fn char_delete_refused(code: u8) -> Vec<u8> {
    Writer::new(HC_REFUSE_DELETECHAR).u8(code).finish()
}

/// 0x0071: go to the map server.
pub fn zone_server(char_id: u32, map: &str, ip: u32, port: u16) -> Vec<u8> {
    Writer::new(HC_NOTIFY_ZONESVR).u32(char_id).str_fixed(&format!("{map}.gat"), 16).u32(ip).u16(port).finish()
}

pub fn refuse_enter(code: u8) -> Vec<u8> {
    Writer::new(HC_REFUSE_ENTER).u8(code).finish()
}

/// 0x0073: map entry accepted.
pub fn accept_enter(tick: u32, x: u16, y: u16, dir: u8) -> Vec<u8> {
    Writer::new(ZC_ACCEPT_ENTER).u32(tick).pos3(x, y, dir).u8(5).u8(5).finish()
}

pub fn server_tick(tick: u32) -> Vec<u8> {
    Writer::new(ZC_NOTIFY_TIME).u32(tick).finish()
}

/// 0x008d: chat bubble + log line from a unit ("name : text").
pub fn unit_chat(id: u32, text: &str) -> Vec<u8> {
    Writer::new(ZC_NOTIFY_CHAT).u32(id).str_nul(text).finish()
}

/// 0x008e: chat line shown as your own.
pub fn self_chat(text: &str) -> Vec<u8> {
    Writer::new(ZC_NOTIFY_PLAYERCHAT).str_nul(text).finish()
}

/// 0x009a: broadcast (yellow, top of screen).
pub fn broadcast(text: &str) -> Vec<u8> {
    Writer::new(ZC_BROADCAST).str_nul(text).finish()
}

pub fn name_ack(id: u32, name: &str) -> Vec<u8> {
    Writer::new(ZC_ACK_REQNAME).u32(id).str_fixed(name, 24).finish()
}

pub fn change_dir(id: u32, head: u16, dir: u8) -> Vec<u8> {
    Writer::new(ZC_CHANGE_DIRECTION).u32(id).u16(head).u8(dir).finish()
}

pub fn emotion(id: u32, kind: u8) -> Vec<u8> {
    Writer::new(ZC_EMOTION).u32(id).u8(kind).finish()
}

/// 0x0091: change map on the same server.
pub fn map_move(map: &str, x: u16, y: u16) -> Vec<u8> {
    Writer::new(ZC_NPCACK_MAPMOVE).str_fixed(&format!("{map}.gat"), 16).u16(x).u16(y).finish()
}

// ---- NPC dialog ----
pub fn npc_mes(npc: u32, text: &str) -> Vec<u8> {
    Writer::new(ZC_SAY_DIALOG).u32(npc).str_nul(text).finish()
}
pub fn npc_next(npc: u32) -> Vec<u8> {
    Writer::new(ZC_WAIT_DIALOG).u32(npc).finish()
}
pub fn npc_close(npc: u32) -> Vec<u8> {
    Writer::new(ZC_CLOSE_DIALOG).u32(npc).finish()
}
pub fn npc_menu(npc: u32, options: &[String]) -> Vec<u8> {
    Writer::new(ZC_MENU_LIST).u32(npc).str_nul(&options.join(":")).finish()
}
pub fn npc_input_number(npc: u32) -> Vec<u8> {
    Writer::new(ZC_OPEN_EDITDLG).u32(npc).finish()
}
pub fn npc_input_string(npc: u32) -> Vec<u8> {
    Writer::new(ZC_OPEN_EDITDLGSTR).u32(npc).finish()
}
/// 0x01b3: cutin picture; pos 0 left, 1 center, 2 right, 255 remove.
pub fn cutin(name: &str, pos: u8) -> Vec<u8> {
    Writer::new(ZC_SHOW_IMAGE2).str_fixed(name, 64).u8(pos).finish()
}

// ---- shops ----
pub fn shop_select(npc: u32) -> Vec<u8> {
    Writer::new(ZC_SELECT_DEALTYPE).u32(npc).finish()
}
/// 0x00c6: (price, discounted price, client item type, item id) × n
pub fn shop_buy_list(items: &[(u32, u8, u32)]) -> Vec<u8> {
    let mut w = Writer::new(ZC_PC_PURCHASE_ITEMLIST);
    for &(price, kind, id) in items {
        w.u32(price).u32(price).u8(kind).u16(id as u16);
    }
    w.finish()
}
/// 0x00c7: (inventory index, price, overcharged price) × n
pub fn shop_sell_list(items: &[(u16, u32)]) -> Vec<u8> {
    let mut w = Writer::new(ZC_PC_SELL_ITEMLIST);
    for &(idx, price) in items {
        w.u16(idx).u32(price).u32(price);
    }
    w.finish()
}
/// 0x00ca: 0 ok, 1 no zeny, 2 overweight
pub fn shop_buy_result(code: u8) -> Vec<u8> {
    Writer::new(ZC_PC_PURCHASE_RESULT).u8(code).finish()
}
pub fn shop_sell_result(code: u8) -> Vec<u8> {
    Writer::new(ZC_PC_SELL_RESULT).u8(code).finish()
}

// ---- status ----
/// 0x00b0 (u16 type, u32 value) for most parameters.
pub fn par(kind: u16, value: u32) -> Vec<u8> {
    Writer::new(ZC_PAR_CHANGE).u16(kind).u32(value).finish()
}
/// 0x00b1 for exp / zeny / next exp.
pub fn long_par(kind: u16, value: u32) -> Vec<u8> {
    Writer::new(ZC_LONGPAR_CHANGE).u16(kind).u32(value).finish()
}
/// 0x0141 (u32 type 13..18, u32 base, u32 bonus) for str..luk.
pub fn stat_par(kind: u32, base: u32, bonus: u32) -> Vec<u8> {
    Writer::new(ZC_COUPLESTATUS).u32(kind).u32(base).u32(bonus).finish()
}
/// 0x00be (u16 type 32..37, u8 cost) status point cost to raise a stat.
pub fn stat_cost(kind: u16, cost: u8) -> Vec<u8> {
    Writer::new(ZC_STATUS_CHANGE).u16(kind).u8(cost).finish()
}
/// 0x00bc: result of raising a stat.
pub fn stat_up_ack(kind: u16, ok: bool, value: u8) -> Vec<u8> {
    Writer::new(ZC_STATUS_CHANGE_ACK).u16(kind).u8(ok as u8).u8(value).finish()
}

#[derive(Debug, Clone, Default)]
pub struct FullStatus {
    pub status_point: u16,
    pub stats: [u8; 6],
    pub costs: [u8; 6],
    pub atk: u16,
    pub atk2: u16,
    pub matk_max: u16,
    pub matk_min: u16,
    pub def: u16,
    pub def2: u16,
    pub mdef: u16,
    pub mdef2: u16,
    pub hit: u16,
    pub flee: u16,
    pub flee2: u16,
    pub crit: u16,
    pub karma: u16,
    pub manner: u16,
}

/// 0x00bd (44)
pub fn full_status(s: &FullStatus) -> Vec<u8> {
    let mut w = Writer::new(ZC_STATUS);
    w.u16(s.status_point);
    for i in 0..6 {
        w.u8(s.stats[i]).u8(s.costs[i]);
    }
    w.u16(s.atk).u16(s.atk2).u16(s.matk_max).u16(s.matk_min).u16(s.def).u16(s.def2).u16(s.mdef).u16(s.mdef2);
    w.u16(s.hit).u16(s.flee).u16(s.flee2).u16(s.crit).u16(s.karma).u16(s.manner);
    w.finish()
}

/// 0x0119: option/status change (sitting is not here; cart/peco/hide are).
pub fn state_change(id: u32, opt1: u16, opt2: u16, option: u16, karma: u8) -> Vec<u8> {
    Writer::new(ZC_STATE_CHANGE).u32(id).u16(opt1).u16(opt2).u16(option).u8(karma).finish()
}

/// 0x008a: action / damage. kind: 0 hit, 1 pick up, 2 sit, 3 stand, 8 double, 10 crit, 11 lucky dodge
#[allow(clippy::too_many_arguments)]
pub fn action(src: u32, dst: u32, tick: u32, src_delay: u32, dst_delay: u32, damage: u16, div: u16, kind: u8, damage2: u16) -> Vec<u8> {
    Writer::new(ZC_NOTIFY_ACT)
        .u32(src).u32(dst).u32(tick).u32(src_delay).u32(dst_delay)
        .u16(damage).u16(div).u8(kind).u16(damage2)
        .finish()
}

/// 0x013a: attack range.
pub fn attack_range(range: u16) -> Vec<u8> {
    Writer::new(ZC_ATTACK_RANGE).u16(range).finish()
}

// ---- items ----
pub const INV_OFFSET: u16 = 2; // client inventory index = slot + 2 (Athena)

/// 0x00a3: stackable items, 10 bytes each.
pub fn item_list(items: &[(u16, &InvItem, u8)]) -> Vec<u8> {
    let mut w = Writer::new(ZC_NORMAL_ITEMLIST);
    for &(idx, it, kind) in items {
        w.u16(idx).u16(it.nameid as u16).u8(kind).u8(it.identify as u8).u16(it.amount.min(30000) as u16).u16(0);
    }
    w.finish()
}

/// 0x00a4: equipment, 20 bytes each.
pub fn equip_list(items: &[(u16, &InvItem, u8, u32)]) -> Vec<u8> {
    let mut w = Writer::new(ZC_EQUIPMENT_ITEMLIST);
    for &(idx, it, kind, loc) in items {
        w.u16(idx).u16(it.nameid as u16).u8(kind).u8(it.identify as u8).u16(loc as u16).u16(it.equip as u16);
        w.u8(it.attribute).u8(it.refine);
        for c in it.cards {
            w.u16(c);
        }
    }
    w.finish()
}

/// 0x00a0: item added. fail: 0 ok, 1 can't carry, 2 overweight.
pub fn item_add(idx: u16, amount: u16, it: &InvItem, loc: u32, kind: u8, fail: u8) -> Vec<u8> {
    let mut w = Writer::new(ZC_ITEM_PICKUP_ACK);
    w.u16(idx).u16(amount).u16(it.nameid as u16).u8(it.identify as u8).u8(it.attribute).u8(it.refine);
    for c in it.cards {
        w.u16(c);
    }
    w.u16(loc as u16).u8(kind).u8(fail);
    w.finish()
}

/// 0x00af: item removed / reduced.
pub fn item_remove(idx: u16, amount: u16) -> Vec<u8> {
    Writer::new(ZC_ITEM_THROW_ACK).u16(idx).u16(amount).finish()
}

/// 0x00a8: use item result.
pub fn use_item_ack(idx: u16, amount_left: u16, ok: bool) -> Vec<u8> {
    Writer::new(ZC_USE_ITEM_ACK).u16(idx).u16(amount_left).u8(ok as u8).finish()
}

/// 0x00aa / 0x00ac: equip / unequip result.
pub fn equip_ack(idx: u16, loc: u32, ok: bool) -> Vec<u8> {
    Writer::new(ZC_REQ_WEAR_EQUIP_ACK).u16(idx).u16(loc as u16).u8(ok as u8).finish()
}
pub fn unequip_ack(idx: u16, loc: u32, ok: bool) -> Vec<u8> {
    Writer::new(ZC_REQ_TAKEOFF_EQUIP_ACK).u16(idx).u16(loc as u16).u8(ok as u8).finish()
}

/// 0x009d: floor item already there.
pub fn floor_item(id: u32, nameid: u32, identify: bool, x: u16, y: u16, amount: u16, subx: u8, suby: u8) -> Vec<u8> {
    Writer::new(ZC_ITEM_ENTRY).u32(id).u16(nameid as u16).u8(identify as u8).u16(x).u16(y).u16(amount).u8(subx).u8(suby).finish()
}
/// 0x009e: floor item falling (drop animation).
pub fn floor_item_drop(id: u32, nameid: u32, identify: bool, x: u16, y: u16, subx: u8, suby: u8, amount: u16) -> Vec<u8> {
    Writer::new(ZC_ITEM_FALL_ENTRY).u32(id).u16(nameid as u16).u8(identify as u8).u16(x).u16(y).u8(subx).u8(suby).u16(amount).finish()
}
pub fn floor_item_gone(id: u32) -> Vec<u8> {
    Writer::new(ZC_ITEM_DISAPPEAR).u32(id).finish()
}

/// One 0x010f entry.
#[derive(Debug, Clone)]
pub struct SkillEntry {
    pub id: u16,
    /// Target type (inf): 0 passive, 1 enemy, 2 place, 4 self, 16 friend.
    pub inf: u16,
    pub level: u16,
    pub sp: u16,
    pub range: u16,
    /// Aegis name the client shows if it lacks its own (24 bytes).
    pub name: String,
    pub upgradable: bool,
}

/// 0x010f: skill list, 37 bytes per skill (Athena clif_skillinfoblock).
pub fn skill_list(skills: &[SkillEntry]) -> Vec<u8> {
    let mut w = Writer::new(ZC_SKILLINFO_LIST);
    for s in skills {
        w.u16(s.id).u16(s.inf).u16(0).u16(s.level).u16(s.sp).u16(s.range).str_fixed(&s.name, 24).u8(s.upgradable as u8);
    }
    w.finish()
}

/// 0x0110: skill failed. For skill 1 (Basic Skill), btype picks the message:
/// 0 trade, 1 emotion, 2 sit, 3 chat room, 4 party (Athena clif_skill_fail).
pub fn skill_fail(skill: u16, btype: u16, kind: u8) -> Vec<u8> {
    Writer::new(ZC_ACK_TOUSESKILL).u16(skill).u16(btype).u16(0).u8(0).u8(kind).finish()
}

/// 0x010e: one skill changed (after a level-up).
pub fn skill_update(id: u16, level: u16, sp: u16, range: u16, upgradable: bool) -> Vec<u8> {
    Writer::new(ZC_SKILLINFO_UPDATE).u16(id).u16(level).u16(sp).u16(range).u8(upgradable as u8).finish()
}

/// 0x0144: mark on the mini-map (Athena clif_viewpoint). kind 1 show, 2 remove.
pub fn viewpoint(npc: u32, kind: u32, x: u32, y: u32, id: u8, color: u32) -> Vec<u8> {
    Writer::new(ZC_COMPASS).u32(npc).u32(kind).u32(x).u32(y).u8(id).u32(color).finish()
}

/// 0x0189: "Unable to Teleport in this area" (kind 0); 1 = can't memo here.
pub fn map_info(kind: u16) -> Vec<u8> {
    Writer::new(0x0189).u16(kind).finish()
}

/// 0x0188: refine result (0 success, 1 failure).
pub fn refine_result(result: u16, idx: u16, refine: u16) -> Vec<u8> {
    Writer::new(ZC_ACK_ITEMREFINING).u16(result).u16(idx).u16(refine).finish()
}

/// 0x019b: misc effect on a unit (Athena clif_misceffect: 0 base level up, 1 job level up,
/// 2 refine failure, 3 refine success).
pub fn misc_effect(id: u32, effect: u32) -> Vec<u8> {
    Writer::new(ZC_NOTIFY_EFFECT).u32(id).u32(effect).finish()
}

/// 0x0121: cart counts (items, max items, weight, max weight).
pub fn cart_info(num: u16, max: u16, weight: u32, max_weight: u32) -> Vec<u8> {
    Writer::new(ZC_NOTIFY_CARTITEM_COUNTINFO).u16(num).u16(max).u32(weight).u32(max_weight).finish()
}

/// 0x0123: stackable cart items (10 bytes each, as 0x00a3).
pub fn cart_items(items: &[(u16, &InvItem, u8)]) -> Vec<u8> {
    let mut w = Writer::new(ZC_CART_NORMAL_ITEMLIST);
    for &(idx, it, kind) in items {
        w.u16(idx).u16(it.nameid as u16).u8(kind).u8(it.identify as u8).u16(it.amount.min(30000) as u16).u16(0);
    }
    w.finish()
}

/// 0x0122: cart equipment (20 bytes each, as 0x00a4).
pub fn cart_equips(items: &[(u16, &InvItem, u8, u32)]) -> Vec<u8> {
    let mut w = Writer::new(ZC_CART_EQUIPMENT_ITEMLIST);
    for &(idx, it, kind, loc) in items {
        w.u16(idx).u16(it.nameid as u16).u8(kind).u8(it.identify as u8).u16(loc as u16).u16(0);
        w.u8(it.attribute).u8(it.refine);
        for c in it.cards {
            w.u16(c);
        }
    }
    w.finish()
}

/// 0x0124: item put into the cart.
pub fn cart_add(idx: u16, amount: u32, it: &InvItem) -> Vec<u8> {
    let mut w = Writer::new(ZC_ADD_ITEM_TO_CART);
    w.u16(idx).u32(amount).u16(it.nameid as u16).u8(it.identify as u8).u8(it.attribute).u8(it.refine);
    for c in it.cards {
        w.u16(c);
    }
    w.finish()
}

/// 0x0125: item taken out of the cart.
pub fn cart_remove(idx: u16, amount: u32) -> Vec<u8> {
    Writer::new(ZC_DELETE_ITEM_FROM_CART).u16(idx).u32(amount).finish()
}

/// 0x0196: status icon on/off (Athena clif_status_change).
pub fn status_icon(kind: u16, id: u32, on: bool) -> Vec<u8> {
    Writer::new(0x0196).u16(kind).u32(id).u8(on as u8).finish()
}

/// 0x019e: the client enters taming mode (pick a monster).
pub fn catch_process() -> Vec<u8> {
    Writer::new(0x019e).finish()
}

/// 0x01a0: taming result.
pub fn catch_result(ok: bool) -> Vec<u8> {
    Writer::new(0x01a0).u8(ok as u8).finish()
}

/// 0x01a2: pet status window.
pub fn pet_status(name: &str, renamed: bool, level: u16, hungry: i32, intimate: i32, equip: u32) -> Vec<u8> {
    Writer::new(0x01a2)
        .str_fixed(name, 24)
        .u8(renamed as u8)
        .u16(level)
        .u16(hungry.clamp(0, 100) as u16)
        .u16(intimate.clamp(0, 1000) as u16)
        .u16(equip as u16)
        .finish()
}

/// 0x01a3: feeding result.
pub fn pet_food(ok: bool, food: u32) -> Vec<u8> {
    Writer::new(0x01a3).u8(ok as u8).u16(food as u16).finish()
}

/// 0x01a4: a pet value (0 spawn, 1 intimacy, 2 hunger, 3 accessory, 4 performance, 5 0x14).
pub fn pet_data(kind: u8, pet: u32, value: u32) -> Vec<u8> {
    Writer::new(0x01a4).u8(kind).u32(pet).u32(value).finish()
}

/// 0x01a6: inventory indices of the eggs that can hatch.
pub fn egg_list(idx: &[u16]) -> Vec<u8> {
    let mut w = Writer::new(0x01a6);
    for i in idx {
        w.u16(*i);
    }
    w.finish()
}

/// 0x01aa: pet emote.
pub fn pet_emotion(pet: u32, emotion: u32) -> Vec<u8> {
    Writer::new(0x01aa).u32(pet).u32(emotion).finish()
}

/// 0x00f2: storage counts.
pub fn storage_info(num: u16, max: u16) -> Vec<u8> {
    Writer::new(ZC_NOTIFY_STOREITEM_COUNTINFO).u16(num).u16(max).finish()
}

/// 0x00a5: stackable storage items, 10 bytes each (same layout as 0x00a3).
pub fn storage_items(items: &[(u16, &InvItem, u8)]) -> Vec<u8> {
    let mut w = Writer::new(ZC_STORE_NORMAL_ITEMLIST);
    for &(idx, it, kind) in items {
        w.u16(idx).u16(it.nameid as u16).u8(kind).u8(it.identify as u8).u16(it.amount.min(30000) as u16).u16(0);
    }
    w.finish()
}

/// 0x00a6: storage equipment, 20 bytes each (same layout as 0x00a4).
pub fn storage_equips(items: &[(u16, &InvItem, u8, u32)]) -> Vec<u8> {
    let mut w = Writer::new(ZC_STORE_EQUIPMENT_ITEMLIST);
    for &(idx, it, kind, loc) in items {
        w.u16(idx).u16(it.nameid as u16).u8(kind).u8(it.identify as u8).u16(loc as u16).u16(0);
        w.u8(it.attribute).u8(it.refine);
        for c in it.cards {
            w.u16(c);
        }
    }
    w.finish()
}

/// 0x00f4: item put into storage.
pub fn storage_add(idx: u16, amount: u32, it: &InvItem) -> Vec<u8> {
    let mut w = Writer::new(ZC_ADD_ITEM_TO_STORE);
    w.u16(idx).u32(amount).u16(it.nameid as u16).u8(it.identify as u8).u8(it.attribute).u8(it.refine);
    for c in it.cards {
        w.u16(c);
    }
    w.finish()
}

/// 0x00f6: item taken out of storage.
pub fn storage_remove(idx: u16, amount: u32) -> Vec<u8> {
    Writer::new(ZC_DELETE_ITEM_FROM_STORE).u16(idx).u32(amount).finish()
}

/// 0x00f8: storage window closed.
pub fn storage_closed() -> Vec<u8> {
    Writer::new(ZC_CLOSE_STORE).finish()
}

/// 0x00b3: back to character select.
pub fn restart_ack(ok: bool) -> Vec<u8> {
    Writer::new(ZC_RESTART_ACK).u8(ok as u8).finish()
}

pub fn quit_ack(result: u16) -> Vec<u8> {
    Writer::new(ZC_ACK_REQ_DISCONNECT).u16(result).finish()
}

pub fn user_count(n: u32) -> Vec<u8> {
    Writer::new(ZC_USER_COUNT).u32(n).finish()
}

/// 0x00c3: look change (type 0 class, 1 hair, 2 weapon, 3 head bottom,
/// 4 head top, 5 head mid, 6 hair color, 7 clothes color, 8 shield).
pub fn look(id: u32, kind: u8, value: u8) -> Vec<u8> {
    Writer::new(ZC_SPRITE_CHANGE).u32(id).u8(kind).u8(value).finish()
}

/// 0x01d7: look change with 16-bit values (weapon+shield split).
pub fn look2(id: u32, kind: u8, v1: u16, v2: u16) -> Vec<u8> {
    Writer::new(ZC_SPRITE_CHANGE2).u32(id).u8(kind).u16(v1).u16(v2).finish()
}

// ---- skills (layouts: mod0631 clif.c, PACKETVER 4; lengths are the client's) ----

/// 0x013e: casting bar. `element` colours the cast circle.
pub fn skill_casting(src: u32, dst: u32, x: u16, y: u16, skill: u16, element: u32, cast_ms: u32) -> Vec<u8> {
    Writer::new(0x013e).u32(src).u32(dst).u16(x).u16(y).u16(skill).u32(element).u32(cast_ms).finish()
}

/// 0x01b9: the cast was interrupted.
pub fn skill_cast_cancel(id: u32) -> Vec<u8> {
    Writer::new(0x01b9).u32(id).finish()
}

/// 0x01de: skill damage (32-bit damage). type: 6 single hit, 8 multi-hit, 5 splash, 9 endure.
#[allow(clippy::too_many_arguments)]
pub fn skill_damage(skill: u16, src: u32, dst: u32, tick: u32, sdelay: u32, ddelay: u32, damage: i32, lv: u16, div: u16, kind: u8) -> Vec<u8> {
    Writer::new(0x01de).u16(skill).u32(src).u32(dst).u32(tick).u32(sdelay).u32(ddelay).i32(damage).u16(lv).u16(div).u8(kind).finish()
}

/// 0x0115: skill damage that also throws the target to (x, y).
#[allow(clippy::too_many_arguments)]
pub fn skill_damage_blown(skill: u16, src: u32, dst: u32, tick: u32, sdelay: u32, ddelay: u32, x: u16, y: u16, damage: u16, lv: u16, div: u16, kind: u8) -> Vec<u8> {
    Writer::new(0x0115).u16(skill).u32(src).u32(dst).u32(tick).u32(sdelay).u32(ddelay).u16(x).u16(y).u16(damage).u16(lv).u16(div).u8(kind).finish()
}

/// 0x011a: skill without damage (buffs, heals; `heal` is the number shown).
pub fn skill_nodamage(skill: u16, heal: u16, dst: u32, src: u32, ok: u8) -> Vec<u8> {
    Writer::new(0x011a).u16(skill).u16(heal).u32(dst).u32(src).u8(ok).finish()
}

/// 0x0117: skill effect on a cell.
pub fn skill_poseffect(skill: u16, src: u32, lv: u16, x: u16, y: u16, tick: u32) -> Vec<u8> {
    Writer::new(0x0117).u16(skill).u32(src).u16(lv).u16(x).u16(y).u32(tick).finish()
}

/// 0x01c9: a ground skill cell appears (97 bytes; the tail is Graffiti text).
pub fn skill_unit(id: u32, src: u32, x: u16, y: u16, unit_id: u8) -> Vec<u8> {
    Writer::new(0x01c9).u32(id).u32(src).u16(x).u16(y).u8(unit_id).u8(1).zeros(81).finish()
}

/// 0x0120: a ground skill cell is gone.
pub fn skill_unit_gone(id: u32) -> Vec<u8> {
    Writer::new(0x0120).u32(id).finish()
}

/// 0x011c: Teleport / Warp Portal destination list (four 16-byte map names).
pub fn skill_warp_point(skill: u16, maps: &[String]) -> Vec<u8> {
    let mut w = Writer::new(0x011c);
    w.u16(skill);
    for i in 0..4 {
        w.str_fixed(maps.get(i).map(String::as_str).unwrap_or(""), 16);
    }
    w.finish()
}

/// 0x011e: /memo result (0 saved, 1 skill too low, 2 no Warp Portal).
pub fn skill_memo(flag: u8) -> Vec<u8> {
    Writer::new(0x011e).u8(flag).finish()
}

/// 0x018c: Sense (monster information).
#[allow(clippy::too_many_arguments)]
pub fn estimation(class: u16, level: u16, size: u16, hp: u32, def: u16, race: u16, mdef: u16, element: u16, fix: &[u8]) -> Vec<u8> {
    let mut w = Writer::new(0x018c);
    w.u16(class).u16(level).u16(size).u32(hp).u16(def).u16(race).u16(mdef).u16(element);
    for i in 0..9 {
        w.u8(fix.get(i).copied().unwrap_or(100));
    }
    w.finish()
}

/// 0x013d: "recovered N HP/SP" (type 5 HP, 7 SP).
pub fn heal_notice(kind: u16, amount: u16) -> Vec<u8> {
    Writer::new(0x013d).u16(kind).u16(amount).finish()
}

/// 0x0148: a dead player stands up again.
pub fn resurrection(id: u32, kind: u16) -> Vec<u8> {
    Writer::new(0x0148).u32(id).u16(kind).finish()
}

/// 0x0192: a map cell changed type (Ice Wall: 5 blocked, 0 free).
pub fn change_map_cell(x: u16, y: u16, kind: u16, map: &str) -> Vec<u8> {
    Writer::new(0x0192).u16(x).u16(y).u16(kind).str_fixed(&format!("{map}.gat"), 16).finish()
}

/// 0x013b: arrow message (0 "equip arrows first", 3 "arrow equipped").
pub fn arrow_fail(kind: u16) -> Vec<u8> {
    Writer::new(0x013b).u16(kind).finish()
}

/// 0x013c: the arrow in the quiver (client inventory index; 0 = none).
pub fn arrow_equip(idx: u16) -> Vec<u8> {
    Writer::new(0x013c).u16(idx).finish()
}

/// 0x01ad: items Arrow Crafting can use.
pub fn arrow_create_list(ids: &[u16]) -> Vec<u8> {
    let mut w = Writer::new(0x01ad);
    for i in ids {
        w.u16(*i);
    }
    w.finish()
}

/// 0x0177: unidentified items (client inventory indices).
pub fn identify_list(idx: &[u16]) -> Vec<u8> {
    let mut w = Writer::new(0x0177);
    for i in idx {
        w.u16(*i);
    }
    w.finish()
}

/// 0x0179: identify result (0 = identified).
pub fn identified(idx: u16, fail: u8) -> Vec<u8> {
    Writer::new(0x0179).u16(idx).u8(fail).finish()
}

/// 0x017b: equipment a card can go into (client inventory indices).
pub fn card_targets(idx: &[u16]) -> Vec<u8> {
    let mut w = Writer::new(0x017b);
    for i in idx {
        w.u16(*i);
    }
    w.finish()
}

/// 0x017d: card inserted (fail 0) or not (1).
pub fn card_inserted(equip_idx: u16, card_idx: u16, fail: u8) -> Vec<u8> {
    Writer::new(0x017d).u16(equip_idx).u16(card_idx).u8(fail).finish()
}

/// 0x0147: a skill an item lets you use once (Fly Wing's Teleport, scrolls).
pub fn item_skill(skill: u16, inf: u16, lv: u16, sp: u16, range: u16, name: &str) -> Vec<u8> {
    Writer::new(0x0147).u16(skill).u16(inf).u16(0).u16(lv).u16(sp).u16(range).str_fixed(name, 24).u8(0).finish()
}

/// 0x010c: MVP effect over a player.
pub fn mvp_effect(id: u32) -> Vec<u8> {
    Writer::new(0x010c).u32(id).finish()
}

/// 0x010a: MVP reward item.
pub fn mvp_item(nameid: u16) -> Vec<u8> {
    Writer::new(0x010a).u16(nameid).finish()
}

/// 0x010b: MVP reward EXP.
pub fn mvp_exp(exp: u32) -> Vec<u8> {
    Writer::new(0x010b).u32(exp).finish()
}
