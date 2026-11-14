#!/usr/bin/env python3
"""Translate era Athena NPC scripts into ragnarok-offline Lua.

  athena_npc.py <refs/eathena-archive/x> <repo root> [--report]

Source dialect: Athena/eAthena of Dec 2003 - Feb 2004 (817's map/script.c and
map/npc.c, which 659 shares). The parser mirrors 817 exactly where it
matters:

* a script body runs from the header's '{' to the first line whose last
  non-blank character is '}' (npc.c npc_parse_script);
* a statement is `name arg, arg ...;` with optional commas, or `label:`;
  `if (cond) <statement>` runs any statement, not just goto (buildin_if);
* expression precedence || 0, && 1, comparisons 2, ^ 3, | 4, & 5, + - 6,
  * / % 7, unary binds tightest (parse_subexpr);
* builtins and db/const.txt names are case-insensitive, variables and labels
  are not (add_str);
* variables: `@x`/`l<n>` temporary, `$x` server-wide, anything else a
  permanent character variable (get_val); all are integers;
* `menu "a",L,"b",-;` jumps to L, "-" means the next statement; cancel ends
  the script (buildin_menu); goto to an undefined label ends it (buildin_goto).

The output is plain Lua with goto, one npc{} per Athena NPC, citing file and
line. Labels starting with "On" are event entry points (npc_parse_script).
Fixes to bugs of the source are not made here: they belong in hand-written
overrides under scripts/npc/ (docs/DEVIATIONS.md lists them). The one
exception is SOURCE_FIXES: a one-line slip whose fix is not in doubt (it
stops a long script dead, or a second period source and the client settle
it), where an override would mean copying a long NPC.

The Athena sources are GPL; so is the output (the project is GPL-3.0-or-later).
"""
import json
import re
import sys
from pathlib import Path

REFS = Path(sys.argv[1])
ROOT = Path(sys.argv[2])
REPORT = "--report" in sys.argv
E817 = REFS / "athena-817-src/athena"
E659 = REFS / "athena0659TestEngmod/athena0659TestEngmod"
# Source releases: key -> (npc directory, description). Default 817; a FILES
# entry can name another (src tags then read "659:jobs/npc_wizard.txt:4").
SOURCES = {
    "817": (E817 / "npc", "eathena 817 (2004-02-19)"),
    "659": (E659 / "conf/npc", "athena 659 TestEngmod (2004-01-01)"),
}

def _load_maps():
    import importlib.util
    spec = importlib.util.spec_from_file_location("ro_maps", Path(__file__).with_name("maps.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod.MAP_ALIAS


MAP_ALIAS = _load_maps()  # Athena map name -> this client's (tools/import/maps.py)

# file -> (output path, guard, note). guard: a content flag the file needs
# (scripts/rules/content.lua); None = always. Files not listed are skipped,
# with the reason in SKIP.
FILES = {
    "cities/alberta.txt": ("cities/alberta.lua", None, ""),
    "cities/aldebaran.txt": ("cities/aldebaran.lua", None, ""),
    "cities/comodo.txt": ("cities/comodo.lua", None, ""),
    "cities/geffen.txt": ("cities/geffen.lua", None, ""),
    "cities/izlude.txt": ("cities/izlude.lua", None, ""),
    "cities/lutie.txt": ("cities/lutie.lua", None, ""),
    "cities/morroc.txt": ("cities/morroc.lua", None, ""),
    "cities/payon.txt": ("cities/payon.lua", None, ""),
    "cities/prontera.txt": ("cities/prontera.lua", None, ""),
    "cities/yuno.txt": ("cities/yuno.lua", "juno", "Juno opened on euRO 2004-06-09"),
    "jobs/novice.txt": ("jobs/novice.lua", None, "training ground: new_1-N is this client's new_zone0N (tools/import/maps.py)"),
    "jobs/swordsman.txt": ("jobs/swordsman.lua", None, ""),
    "jobs/magican.txt": ("jobs/magician.lua", None, ""),
    "jobs/archer.txt": ("jobs/archer.lua", None, ""),
    "jobs/acolyte.txt": ("jobs/acolyte.lua", None, ""),
    "jobs/merchant.txt": ("jobs/merchant.lua", None, ""),
    "jobs/thief.txt": ("jobs/thief.lua", None, ""),
    "jobs/knight.txt": ("jobs/knight.lua", None, ""),
    "jobs/priest.txt": ("jobs/priest.lua", None, ""),
    "659:jobs/npc_wizard.txt": ("jobs/wizard.lua", None,
                                "659's guild NPC: 817's wizard.txt is an unfinished conversion (later dialect, "
                                "Job_Magician, missing gotos) that 817 itself could not run"),
    "jobs/blacksmith.txt": ("jobs/blacksmith.lua", None, ""),
    "jobs/hunter.txt": ("jobs/hunter.lua", None, ""),
    "jobs/assassin.txt": ("jobs/assassin.lua", None, ""),
    "jobs/crusader.txt": ("jobs/crusader.lua", "jobs2_2", "2-2 jobs opened on euRO 2004-06-09"),
    "jobs/monk.txt": ("jobs/monk.lua", "jobs2_2", "2-2 jobs opened on euRO 2004-06-09"),
    "jobs/sage.txt": ("jobs/sage.lua", "jobs2_2", "2-2 jobs opened on euRO 2004-06-09"),
    "jobs/rogue.txt": ("jobs/rogue.lua", "jobs2_2", "2-2 jobs opened on euRO 2004-06-09"),
    "jobs/alchemist.txt": ("jobs/alchemist.lua", "jobs2_2", "2-2 jobs opened on euRO 2004-06-09"),
    "jobs/dancer-bard.txt": ("jobs/dancer_bard.lua", "jobs2_2", "2-2 jobs opened on euRO 2004-06-09"),
    "other/kafra.txt": ("other/kafra.lua", None, ""),
    "other/refine.txt": ("other/refine.lua", None, ""),
    "other/guide.txt": ("other/guide.lua", None, ""),
    "other/books.txt": ("other/books.lua", None, ""),
    "other/ctower.txt": ("other/clock_tower.lua", None, ""),
    "other/arena.txt": ("other/arena.lua", "arena",
                        "monster time attack, entered from the middle of Prontera: 817's own placement. kRO only "
                        "opened the arena (in Izlude) on 2004-10-12; off by default (THREADS.md R5)"),
    "other/pvp.txt": ("other/pvp.lua", "pvp", "PvP needs other players; off by default (DEVIATIONS.md)"),
    "other/IceCream.txt": ("events/ice_cream.lua", "events", "seasonal event (Aug 2003)"),
    "other/MrSmile.txt": ("events/mr_smile.lua", "events", "national event"),
    "mobs/gvg.txt": ("castles/monsters.lua", "castle_monsters",
                     "monsters of a guild castle nobody owns, called at WoE start (scripts/npc/castles.lua "
                     "calls them: THREADS.md #17)"),
    "quests/headgearquest.txt": ("quests/headgear.lua", None, ""),
    "quests/bunnyband.txt": ("quests/bunny_band.lua", "events", "Bunny Band festival event"),
    "quests/munak.txt": ("quests/munak.lua", None, ""),
    "quests/bongun.txt": ("quests/bongun.lua", None, ""),
    "quests/bongunsword.txt": ("quests/bongun_sword.lua", None, ""),
    "skillquests/arrowcraft.txt": ("skillquests/arrow_craft.lua", None, ""),
    "skillquests/arrowrepel.txt": ("skillquests/arrow_repel.lua", None, ""),
    "skillquests/autoberserk.txt": ("skillquests/auto_berserk.lua", None, ""),
    "skillquests/backslide.txt": ("skillquests/back_slide.lua", None, ""),
    "skillquests/cartrevolution.txt": ("skillquests/cart_revolution.lua", None, ""),
    "skillquests/changecart.txt": ("skillquests/change_cart.lua", None, ""),
    "skillquests/energycoat.txt": ("skillquests/energy_coat.lua", None, ""),
    "skillquests/fatalblow.txt": ("skillquests/fatal_blow.lua", None, ""),
    "skillquests/holylight.txt": ("skillquests/holy_light.lua", None, ""),
    "skillquests/loud.txt": ("skillquests/loud.lua", None, ""),
    "skillquests/movinghp.txt": ("skillquests/moving_hp.lua", None, ""),
    "skillquests/pickstone.txt": ("skillquests/pick_stone.lua", None, ""),
    "skillquests/throwstone.txt": ("skillquests/throw_stone.lua", None, ""),
}
SKIP = {
    "jobs/jobchange.txt": "emulator convenience (instant job changer 'Jobra')",
    "jobs/wizard.txt": "unfinished conversion 817 couldn't run; 659's npc_wizard.txt is used instead",
    "jobs/reset.txt": "emulator convenience (Reset Girl)",
    "other/breeder.txt": "emulator convenience (cart/falcon/peco rental for everyone)",
    "other/heal.txt": "emulator convenience (free healers)",
    "other/warper.txt": "emulator convenience (warper)",
    "other/card_remover.txt": "emulator convenience",
    "other/arena_mvp.txt": "emulator custom (MVP arena)",
    "other/msg_boards.txt": "emulator custom (town message boards)",
    "other/dye.txt": "needs a 77-palette client patch; dyers are hand-written (scripts/npc/dye.lua)",
    "other/momotaro.txt": "jRO event (Japanese text)",
    "other/tougijou.txt": "jRO event on Gonryun (Japanese text, map not in client)",
    "other/event_alchemist.txt": "test event",
    "quests/all_quest.txt": "emulator custom (Quest Assistant)",
    "quests/quest.txt": "emulator custom (Royal Messenger)",
    "quests/Excalibur.txt": "emulator custom",
    "quests/fashion.txt": "emulator custom ('Daily Job Quest')",
    "quests/Event_ElvenEar.txt": "emulator custom ('Daily Job Quest')",
    "skillquests/Platinum_Skills.txt": "emulator convenience (all quest skills at once)",
    "quests/kahoquest.txt": "emulator custom item quest ('Lord Kaho's Servant')",
    "quests/ironcane_quest.txt": "emulator custom item quest ('Iron Cane Quest')",
    "quests/magicalhatquest.txt": "emulator custom item quest ('Magic Pimp Hat Dude')",
    "other/event_valentine.txt": "jRO Valentine event, Japanese text",
    "skillquests/skillquest.txt": "duplicate of the individual skill quest files (Japanese names)",
    "cities/amatsu.txt": "map not in this client",
    "cities/gonryun.txt": "map not in this client",
    "cities/niflheim.txt": "map not in this client",
    "cities/umbala.txt": "map not in this client",
    "guild/*": "guild castles: needs guilds; single-player stand-ins in scripts/npc/guild.lua",
    "sample/*": "samples",
    "warps/*": "imported from mod0631 by athena_import.py",
    "mobs/*": "imported from mod0631 by athena_import.py (except gvg.txt: script NPCs, above)",
    "shops/*": "imported from mod0631 by athena_import.py",
}

# Single NPCs inside imported files that aren't period content.
SKIP_NPC = {
    "other/kafra.txt:2315": "'FGM Kafra', made for the 'FGM RO' private server",
    "other/kafra.txt:995": "Kafra points exchange: its menu labels start with digits, which 817 parsed as numbers "
                           "(the menu never worked); Kafra points in Dec 2003 are a research thread",
}

LUA_KEYWORDS = set("and break do else elseif end false for function goto if in local nil not or repeat return "
                   "then true until while".split())

# Athena param (const.txt type 1) -> player field
PARAMS = {
    "statuspoint": "status_point", "baselevel": "base_level", "skillpoint": "skill_point", "class": "class",
    "zeny": "zeny", "sex": "sex", "weight": "weight", "maxweight": "max_weight", "joblevel": "job_level",
    "baseexp": "base_exp", "jobexp": "job_exp", "nextbaseexp": "next_base_exp", "nextjobexp": "next_job_exp",
    "hp": "hp", "maxhp": "max_hp", "sp": "sp", "maxsp": "max_sp",
}

# builtin -> (lua name, result type). Types: i int, s string, n none.
BUILTINS = {
    "jobchange": ("job_change", "n"), "warp": ("warp", "n"), "areawarp": ("area_warp", "n"),
    "setlook": ("set_look", "n"), "getitem": ("give_item", "n"), "delitem": ("take_item", "n"),
    "countitem": ("count_item", "i"), "checkweight": ("check_weight", "i"), "viewpoint": ("viewpoint", "n"),
    "heal": ("heal", "n"), "itemheal": ("item_heal", "n"), "percentheal": ("percent_heal", "n"),
    "rand": ("rand", "i"), "readparam": ("read_param", "i"), "getcharid": ("char_id", "i"),
    "getpartyname": ("party_name", "s"), "getguildname": ("guild_name", "s"),
    "getguildmaster": ("guild_master", "s"), "strcharinfo": ("char_info", "s"),
    "getequipname": ("equip_name", "s"), "getequipisequiped": ("equip_worn", "i"),
    "getequipisenableref": ("equip_refinable", "i"), "getequipisidentify": ("equip_identified", "i"),
    "getequiprefinerycnt": ("equip_refine", "i"), "getequipweaponlv": ("equip_weapon_level", "i"),
    "getequippercentrefinery": ("equip_refine_chance", "i"), "successrefitem": ("refine_success", "n"),
    "failedrefitem": ("refine_fail", "n"), "cutin": ("cutin", "n"), "statusup": ("status_up", "n"),
    "statusup2": ("status_up2", "n"), "bonus": ("bonus", "n"), "bonus2": ("bonus2", "n"), "bonus3": ("bonus3", "n"),
    "skill": ("give_skill", "n"), "getskilllv": ("skill_level", "i"), "basicskillcheck": ("basic_skill_check", "i"),
    "getgmlevel": ("gm_level", "i"), "checkoption": ("check_option", "i"), "setoption": ("set_option", "n"),
    "setcart": ("set_cart", "n"), "setfalcon": ("set_falcon", "n"), "setriding": ("set_riding", "n"),
    "savepoint": ("save_point", "n"), "openstorage": ("open_storage", "n"), "itemskill": ("item_skill", "n"),
    "produce": ("produce", "n"), "monster": ("monster", "n"), "areamonster": ("area_monster", "n"),
    "killmonster": ("kill_monster", "n"), "doevent": ("do_event", "n"), "addtimer": ("add_timer", "n"),
    "deltimer": ("del_timer", "n"), "addtimercount": ("add_timer_count", "n"), "announce": ("announce", "n"),
    "mapannounce": ("map_announce", "n"), "areaannounce": ("area_announce", "n"), "getusers": ("get_users", "i"),
    "getmapusers": ("map_users", "i"), "getareausers": ("area_users", "i"), "enablenpc": ("enable_npc", "n"),
    "disablenpc": ("disable_npc", "n"), "enablearena": ("enable_arena", "n"), "disablearena": ("disable_arena", "n"),
    "sc_start": ("sc_start", "n"), "sc_end": ("sc_end", "n"), "debugmes": ("log", "n"), "pet": ("catch_pet", "n"),
    "catchpet": ("catch_pet", "n"), "bpet": ("birth_pet", "n"), "resetstatus": ("reset_stats", "n"),
    "resetskill": ("reset_skills", "n"), "changebase": ("change_base", "n"), "changesex": ("change_sex", "n"),
    "waitingroom": ("waiting_room", "n"), "warpwaitingpc": ("warp_waiting_pc", "n"),
    "setmapflagnosave": ("map_flag_nosave", "n"), "setmapflag": ("set_map_flag", "n"),
    "removemapflag": ("remove_map_flag", "n"), "pvpon": ("pvp_on", "n"), "pvpoff": ("pvp_off", "n"),
    "gvgon": ("gvg_on", "n"), "gvgoff": ("gvg_off", "n"), "emotion": ("npc_emotion", "n"),
    "maprespawnguildid": ("map_respawn_guild", "n"), "agitstart": ("agit_start", "n"), "agitend": ("agit_end", "n"),
    "getcastlename": ("castle_name", "s"), "getcastledata": ("castle_data", "i"),
    "setcastledata": ("set_castle_data", "n"), "mapwarp": ("map_warp", "n"), "inittimer": ("npc_timer_start", "n"),
    "stoptimer": ("npc_timer_stop", "n"), "npctimer": ("npc_timer", "n"), "cmdothernpc": ("cmd_other_npc", "n"),
    "mobcount": ("mob_count", "i"), "getequipcardcnt": ("equip_card_count", "i"),
    "successremovecards": ("remove_cards", "n"), "failedremovecards": ("remove_cards_failed", "n"),
}
# Commands 817 lacked but its scripts used, with an unmistakable meaning. 817
# failed at runtime on them; we run them (docs/DEVIATIONS.md).
BUILTINS["save"] = ("save_point", "n")  # later eAthena alias of savepoint (Prontera inns)
BUILTINS["releaseall"] = ("unequip_all", "n")  # later eAthena; right before jobchange in the
                                               # Merchant and Thief quests, so 817 never changed the job
NOOP_COMMANDS = {"scriptlabel"}  # jAthena label marker: every 817 town guide began with it
# Map-name typos in the sources (not client naming differences: those are maps.py).
MAP_TYPOS = {"morroc": "morocc"}

# Slips in a source line where the fix is not in doubt (see the header). Each is one
# line, replaced before parsing (line numbers stay), named in the generated file and listed in
# docs/DEVIATIONS.md. (file, line): (text in the source, replacement, reason).
SOURCE_FIXES = {
    ("jobs/merchant.txt", 103): (
        "if(@TEMP !=0) goto R10;", "if(@TEMP !=0) goto R11;",
        "the label jumped to itself for ever (one application in eight hung 817); R11 is the "
        "second serial number, as in R00/R01, R20/R21, R30/R31"),
    ("jobs/novice.txt", 637): (
        """menu "'insert' - /Sit",LQ6_1,"'page down' - /Sitting",LQ6_2,"'home' - /Sitdown",LQ6_3,"'delete' - /stand",LQ6_4;""",
        """menu "'insert' - /Sit",LQ6_4,"'page down' - /Sitting",LQ6_2,"'home' - /Sitdown",LQ6_3,"'delete' - /stand",LQ6_1;""",
        "Leo Von Frisch's last question (the key and the command to sit) rewarded 'delete' - /stand; "
        "mod0631 (conf/npc_job_newbie.txt:1111-1122) rewards Insert and /sit, and Insert is this client's sit key"),
    ("jobs/novice.txt", 642): ("`'delete' - /stand`", "`'insert' - /Sit`", "the same slip, in the answer to a wrong choice"),
    ("jobs/novice.txt", 648): ("`'delete' - /stand`", "`'insert' - /Sit`", "the same slip"),
    ("jobs/novice.txt", 654): ("`'delete' - /stand`", "`'insert' - /Sit`", "the same slip"),
    ("jobs/merchant.txt", 244): (
        "goto M0jobmercE9NZ;", "goto M0JobMerQE9NZ;",
        "the label is spelt M0JobMerQE9NZ five lines on: without 500z for the rest of the fee the script "
        "ended with its window open"),
    ("jobs/merchant.txt", 267): (
        "if((job_merchant_q2==7) || (job_merchant_q2==8)) goto JobMerQ2E7o8;",
        "if((job_merchant_q2<7) || (job_merchant_q2>8)) goto JobMerQua1; if(nov_3_merchant==11) goto JobMerQ2E7o8;",
        "the 100z for the letter was paid for being sent to Byalan, letter or no; mod0631 "
        "(conf/npc_job_merchant.txt:70-72) pays when the Kafra got it (she sets nov_3_merchant to 11, "
        "which nothing in 817 read) and says nothing of a letter to those sent elsewhere"),
    ("jobs/merchant.txt", 367): (
        "((countitem(1081)==0) || (countitem(1082)==0) || (countitem(1083)==0))",
        "((countitem(1081)==0) && (countitem(1082)==0) && (countitem(1083)==0))",
        "'no box' was true for everyone (nobody holds all three), so Kay never offered to change a "
        "wrong parcel; mod0631 (conf/npc_job_merchant.txt:537-539, 558-560) asks for none of the three"),
    ("jobs/merchant.txt", 729): (
        "if((job_merchant_q2==3) || (countitem(1081) !=0))", "if((job_merchant_q2==3) && (countitem(1081) !=0))",
        "any box passed for the right one; mod0631 (conf/npc_job_merchant.txt:801-802) wants the errand and its box"),
    ("jobs/merchant.txt", 730): (
        "if((job_merchant_q2==4) || (countitem(1082) !=0))", "if((job_merchant_q2==4) && (countitem(1082) !=0))",
        "the same slip (mod0631 :809-810)"),
    ("jobs/thief.txt", 429): (
        "if (Mushroom2==0) goto NoMushroom;",
        "if ((Mushroom1==0) && (Mushroom2==0)) goto NoMushroom; if (Mushroom2==0) goto TotalMH;",
        "a basket of Orange Net Mushrooms alone (nine are 27 points, a pass) was told it held none; "
        "mod0631 (conf/npc_job_thief.txt:269-270) refuses only when both kinds are missing"),
    ("jobs/archer.txt", 140): ('"+Total_Trunk+"', '"+@Total_Trunk+"', "the score is in @Total_Trunk (line 120); a pass read \"0 / 40\""),
    ("jobs/archer.txt", 147): ('"+Total_Trunk+"', '"+@Total_Trunk+"', "the same slip"),
    ("jobs/archer.txt", 155): ('"+Total_Trunk+"', '"+@Total_Trunk+"', "the same slip"),
    ("jobs/archer.txt", 172): ("delitem 1066;", "delitem 1066,countitem(1066);",
                               "no amount: the trunks handed in stayed in the bag (jAthena //0854, 2004-03-02, "
                               "conf/npc/npc_job_archer.txt:104-107, takes them all)"),
    ("jobs/archer.txt", 173): ("delitem 1067;", "delitem 1067,countitem(1067);", "the same slip"),
    ("jobs/archer.txt", 174): ("delitem 1068;", "delitem 1068,countitem(1068);", "the same slip"),
    ("jobs/archer.txt", 175): ("delitem 1019;", "delitem 1019,countitem(1019);", "the same slip"),
    ("jobs/swordsman.txt", 122): ('"Cancel.",LEnd;', '"Cancel.",L3;',
                                  "no label LEnd: the Sword Master's goodbye is at L3, which nothing reached"),
    ("jobs/swordsman.txt", 372): ("announce @TEMP,3;", "",
                                  "a leftover of testing: this pit told the faller a bare digit, the number of the cave"),
    ("jobs/swordsman.txt", 1041): ("sword_1-1.gat,125,210,4", "sword_1-1.gat,125,203,4",
                                   "pit 2_blank_5_f repeats 2_blank_5_b's cell forty lines up; jAthena //0854 "
                                   "(conf/warp/npc_warp_job.txt:128) has it on row 203, where the strip it closes is"),
    ("jobs/swordsman.txt", 1061): ("sword_1-1.gat,113,201,4", "sword_1-1.gat,125,201,4",
                                   "pit 2_blank_6_b repeats 2_blank_6_a's cell; jAthena (:130) has 125,201"),
    ("jobs/swordsman.txt", 1151): ("139,40,0,{", "139,21,0,{",
                                   "pit 2_blank_9_a is 21 cells to each side like 7_a, 8_a and 8_e above it (jAthena :143), not 40"),
    ("cities/izlude.txt", 56): (
        "if((GetJob==Job_Swordman) || (GetJob==Job_Knight)) goto L00;",
        "if((Class==Job_Swordman) || (Class==Job_Knight)) goto L00;",
        "GetJob is no variable any script sets, so a Swordman or Knight never reached the lines written for "
        "them (\"I knew you'd choose so!!\"); Class is the job everywhere else in 817"),
    ("cities/izlude.txt", 72): (
        "if((GetJob==Job_Swordman) || (GetJob==Job_Knight)) goto L01;",
        "if((Class==Job_Swordman) || (Class==Job_Knight)) goto L01;",
        "the same slip"),
    ("cities/izlude.txt", 116): (
        "if((GetJob==Job_Swordman) || (GetJob==Job_Knight)) goto L00;",
        "if((Class==Job_Swordman) || (Class==Job_Knight)) goto L00;",
        "the same slip"),
    ("cities/izlude.txt", 132): (
        "if((GetJob==Job_Swordman) || (GetJob==Job_Knight)) goto L01;",
        "if((Class==Job_Swordman) || (Class==Job_Knight)) goto L01;",
        "the same slip"),
    ("cities/aldebaran.txt", 824): (
        "if(countitem(7026) < 0) goto CheckKey;", "if(countitem(7027) < 1) goto CheckKey;",
        "no count is below 0, so the Clock Tower's B3 gate let anyone through; mod0631's gatekeeper of "
        "the same gate (conf/npc_cTower.txt:194, \"From jRO, by Kalen\") wants one Key of Underground "
        "(7027), the key for 3F is the Key of Clock Tower (7026)"),
    ("cities/aldebaran.txt", 830): ("delitem 7026,1;", "delitem 7027,1;", "the same: the key it takes (mod0631 :202)"),
    ("cities/aldebaran.txt", 925): (
        "if(countitem(7026) < 0) goto CheckKey;", "if(countitem(7026) < 1) goto CheckKey;",
        "the same test on 3F's gate, which let anyone up to 4F (mod0631 conf/npc_cTower.txt:88)"),
    ("quests/headgearquest.txt", 658): (
        "if(countitem(999)<100) goto LNoBinoculars;",
        "if((countitem(999)<100) || (Zeny<50000)) goto LNoBinoculars;",
        "the 50000z were taken without a test, so with less the Binoculars cost whatever one had; "
        "mod0631's Alchemist tests it (conf/npc_event_making.txt, morocc_in 76,163)"),
    ("quests/headgearquest.txt", 82): (
        "LKeyDork:", "LHeyDork:",
        "both jumps name LHeyDork, as does 817's other copy of this NPC (quests/all_quest.txt:150): "
        "short of Steel or zeny, the Welding Mask smith ended without a word"),
    ("skillquests/arrowrepel.txt", 115): (
        'everything";f', 'everything";',
        "a stray f after the line made the next one an unknown command, so the script stopped "
        "with its window open"),
}

ITEM_ARG = {"getitem", "delitem", "countitem", "checkweight"}
# handled by the statement translator
SPECIAL = {"mes", "next", "close", "menu", "goto", "if", "set", "input", "end"}


def read(p):
    return p.read_bytes().decode("cp1252", "replace").replace("\r\n", "\n").replace("\r", "\n")


def lua_str(s):
    out = ['"']
    for ch in s:
        if ch == '"':
            out.append('\\"')
        elif ch == "\\":
            out.append("\\\\")
        elif ch == "\n":
            out.append("\\n")
        elif ord(ch) < 32:
            out.append("\\%03d" % ord(ch))
        else:
            out.append(ch)
    out.append('"')
    return "".join(out)


def load_consts():
    consts = {}
    for line in read(E817 / "db/const.txt").split("\n"):
        if line.startswith("//"):
            continue
        m = re.match(r"([A-Za-z0-9_]+)[\s,]+(-?\d+)(?:[\s,]+(\d+))?", line)
        if m:
            consts[m.group(1).lower()] = (m.group(1), int(m.group(2)), int(m.group(3) or 0))
    return consts


def load_items():
    items = {}
    cur = {}
    for line in (ROOT / "data/db/items.toml").read_text(encoding="utf-8").split("\n"):
        m = re.match(r'^(id|name) = (?:"([^"]*)"|(\d+))', line)
        if m:
            cur[m.group(1)] = m.group(2) if m.group(2) is not None else int(m.group(3))
            if "id" in cur and "name" in cur:
                items[cur["name"]] = cur["id"]
                cur = {}
    return items


CONSTS = load_consts()
ITEMS = load_items()


class ParseError(Exception):
    pass


# ------------------------------------------------------------------ lexer
class Src:
    """A script body with Athena's skip_space/skip_word semantics."""

    def __init__(self, text, line0):
        self.t = text
        self.p = 0
        self.line0 = line0

    def line(self, p=None):
        return self.line0 + self.t.count("\n", 0, self.p if p is None else p)

    def peek(self, n=0):
        i = self.p + n
        return self.t[i] if i < len(self.t) else ""

    def skip_space(self):
        t = self.t
        while True:
            while self.p < len(t) and t[self.p].isspace():
                self.p += 1
            if t.startswith("//", self.p):
                e = t.find("\n", self.p)
                self.p = len(t) if e < 0 else e
            elif t.startswith("/*", self.p):
                e = t.find("*/", self.p + 2)
                self.p = len(t) if e < 0 else e + 2
            else:
                return

    def word_end(self, p):
        t = self.t
        if p < len(t) and t[p] == "@":
            p += 1
        if p < len(t) and t[p] == "$":
            p += 1
        while p < len(t) and (t[p].isalnum() or t[p] == "_" or ord(t[p]) >= 0x81):
            p += 1
        return p

    def fail(self, msg):
        ctx = self.t[self.p:self.p + 30].split("\n")[0]
        raise ParseError(f"line {self.line()}: {msg} near {ctx!r}")


# AST: ("int", n) ("str", s) ("name", w) ("call", w, [args]) ("un", op, e) ("bin", op, a, b) ("next",)
BINOPS = [("||", 0), ("&&", 1), ("==", 2), ("!=", 2), (">=", 2), ("<=", 2), (">", 2), ("<", 2),
          ("^", 3), ("|", 4), ("&", 5), ("+", 6), ("-", 6), ("*", 7), ("/", 7), ("%", 7)]


def parse_simple(s):
    s.skip_space()
    c = s.peek()
    if c in (";", ","):
        s.fail("unexpected expr end")
    if c == "(":
        s.p += 1
        e = parse_sub(s, -1)
        s.skip_space()
        if s.peek() != ")":
            s.fail("unmatched ')'")
        s.p += 1
        return e
    if c.isdigit() or (c in "-+" and s.peek(1).isdigit()):
        m = re.compile(r"[-+]?(0[xX][0-9a-fA-F]+|0[0-7]*|[1-9][0-9]*)").match(s.t, s.p)
        s.p = m.end()
        txt = m.group(0)
        sign = -1 if txt.startswith("-") else 1
        body = txt.lstrip("+-")
        if body.lower().startswith("0x"):
            v = int(body, 16)
        elif body.startswith("0") and len(body) > 1:
            v = int(body, 8)
        else:
            v = int(body)
        return ("int", sign * v)
    if c == '"':
        s.p += 1
        out = []
        while s.p < len(s.t) and s.t[s.p] != '"':
            if s.t[s.p] == "\\" and s.p + 1 < len(s.t):
                s.p += 1
            out.append(s.t[s.p])
            s.p += 1
        if s.p >= len(s.t):
            s.fail("unexpected eof in string")
        s.p += 1
        return ("str", "".join(out))
    e = s.word_end(s.p)
    if e == s.p:
        s.fail("unexpected character")
    w = s.t[s.p:e]
    s.p = e
    return ("name", w)


def parse_sub(s, limit):
    s.skip_space()
    if s.peek() == "-":
        q = s.p + 1
        save = s.p
        s.p = q
        s.skip_space()
        if s.peek() in (";", ","):
            return ("next",)
        s.p = save
    c = s.peek()
    if c in "-!~" and not (c == "!" and s.peek(1) == "="):
        s.p += 1
        e = ("un", c, parse_sub(s, 100))
    else:
        e = parse_simple(s)
    s.skip_space()
    while True:
        c = s.peek()
        if c == "(" and 8 > limit:
            s.p += 1
            args = []
            s.skip_space()
            while s.peek() and s.peek() != ")":
                args.append(parse_sub(s, -1))
                s.skip_space()
                if s.peek() == ",":
                    s.p += 1
                s.skip_space()
            if s.peek() != ")":
                s.fail("func request '(' ')'")
            s.p += 1
            if e[0] != "name":
                s.fail("call of a non-name")
            e = ("call", e[1], args)
            s.skip_space()
            continue
        for op, prec in BINOPS:
            if s.t.startswith(op, s.p):
                # '&' vs '&&', '|' vs '||' resolved by list order
                if prec > limit:
                    s.p += len(op)
                    rhs = parse_sub(s, prec)
                    e = ("bin", op, e, rhs)
                    s.skip_space()
                    break
                return e
        else:
            return e


def parse_line(s):
    """name args... ; -> ("stmt", name, [args], line)"""
    line = s.line()
    head = parse_simple(s)
    if head[0] != "name":
        s.fail("statement must start with a name")
    args = []
    s.skip_space()
    while s.p < len(s.t) and s.peek() != ";":
        if s.peek() in ")]}:":
            s.fail("unexpected char")
        args.append(parse_sub(s, -1))
        s.skip_space()
        if s.peek() == ",":
            s.p += 1
        s.skip_space()
    if s.peek() != ";":
        s.fail("need ';'")
    s.p += 1
    return ("stmt", head[1], args, line)


def parse_body(text, line0):
    s = Src(text, line0)
    s.skip_space()
    if s.peek() != "{":
        s.fail("not found '{'")
    s.p += 1
    out = []
    while s.p < len(s.t):
        s.skip_space()
        if s.peek() == "}" or s.p >= len(s.t):
            break
        if s.peek() == ";":
            s.p += 1
            continue
        e = s.word_end(s.p)
        q = e
        while q < len(s.t) and s.t[q].isspace():
            q += 1
        if e > s.p and q < len(s.t) and s.t[q] == ":":
            out.append(("label", s.t[s.p:e], s.line()))
            s.p = q + 1
            continue
        out.append(parse_line(s))
    return out


# ------------------------------------------------------------------ translator
class Npc:
    def __init__(self, file, line, header):
        self.file, self.line, self.header = file, line, header
        self.warnings = []


def ident_ok(w):
    return re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", w) is not None and w not in LUA_KEYWORDS


class Tr:
    """Translate one NPC body."""

    def __init__(self, body, npc, used_unknown):
        self.body = body
        self.npc = npc
        self.unknown = used_unknown
        self.labels = {}
        for st in body:
            if st[0] == "label":
                self.labels.setdefault(st[1], self.label_name(st[1]))
        self.uses_menu = any(st[0] == "stmt" and st[1].lower() == "menu" for st in body) or \
            any(st[0] == "stmt" and st[1].lower() == "if" and len(st[2]) > 1 and st[2][1] == ("name", "menu") for st in body)
        self.uses_l15 = "l15" in json.dumps(body)

    def label_name(self, w):
        n = re.sub(r"[^A-Za-z0-9_]", "_", w)
        if not n or n[0].isdigit() or n in LUA_KEYWORDS:
            n = "L_" + n
        return n

    def warn(self, msg):
        self.npc.warnings.append(msg)

    # ---- expressions: return (code, type) with type i/s/b
    def var(self, w):
        lw = w.lower()
        if lw in PARAMS:
            return f"player.{PARAMS[lw]}"
        if w.startswith("@"):
            tbl, key = "tmp", w[1:]
        elif w.startswith("$"):
            tbl, key = "g", w[1:]
        elif re.fullmatch(r"l\d+", w):
            tbl, key = "tmp", w
        else:
            tbl, key = "v", w
        if key.endswith("$"):
            self.warn(f"string variable {w} (not in this Athena)")
        return f"{tbl}.{key}" if ident_ok(key) else f"{tbl}[{lua_str(key)}]"

    def name_value(self, w):
        lw = w.lower()
        if lw in PARAMS:
            return self.var(w), "i"
        if lw in CONSTS and CONSTS[lw][2] == 0:
            nm, val, _ = CONSTS[lw]
            return (nm if ident_ok(nm) else str(val)), "i"
        if lw in BUILTINS:
            return self.call(w, []), BUILTINS[lw][1]
        if w in self.labels:
            self.warn(f"label {w} used as a value")
            return lua_str(w), "s"
        return self.var(w), "i"

    def as_bool(self, e):
        c, t = self.expr(e)
        if t == "b":
            return c
        if t == "s":
            return f"({c} ~= \"\")"
        return f"({c} ~= 0)"

    def as_int(self, e):
        c, t = self.expr(e)
        if t == "b":
            return f"({c} and 1 or 0)"
        return c

    def item_arg(self, e):
        if e[0] == "name" and e[1] in ITEMS and e[1].lower() not in CONSTS:
            return f"{ITEMS[e[1]]} --[[{e[1]}]]"
        return self.as_int(e) if e[0] != "str" else lua_str(e[1])

    def call(self, w, args):
        lw = w.lower()
        if lw not in BUILTINS:
            self.unknown.setdefault(lw, []).append(f"{self.npc.file}:{self.npc.line}")
            self.warn(f"unknown command {w}")
            return f"athena_unknown({lua_str(w)}{''.join(', ' + self.arg(a) for a in args)})"
        name = BUILTINS[lw][0]
        if lw in ITEM_ARG and args:
            parts = [self.item_arg(args[0])] + [self.arg(a) for a in args[1:]]
        else:
            parts = [self.arg(a) for a in args]
        if lw in ("warp", "savepoint", "save", "areawarp", "mapwarp", "monster", "areamonster", "killmonster", "mapannounce",
                  "areaannounce", "getmapusers", "getareausers", "mobcount", "setmapflagnosave", "setmapflag",
                  "removemapflag", "pvpon", "pvpoff", "gvgon", "gvgoff", "maprespawnguildid", "warpwaitingpc"):
            parts = [self.map_arg(a, p) for a, p in zip(args, parts)]
        return f"{name}({', '.join(parts)})"

    def map_arg(self, a, code):
        if a[0] != "str":
            return code
        m = a[1][:-4] if a[1].lower().endswith(".gat") else a[1]
        if m in MAP_TYPOS:
            self.warn(f"map name {m} is a typo in the source; using {MAP_TYPOS[m]}")
            m = MAP_TYPOS[m]
        return lua_str(MAP_ALIAS.get(m, m))

    def arg(self, e):
        if e[0] == "next":
            return "nil"
        c, t = self.expr(e)
        if t == "b":
            return f"({c} and 1 or 0)"
        return c

    def expr(self, e):
        k = e[0]
        if k == "int":
            return str(e[1]), "i"
        if k == "str":
            return lua_str(e[1]), "s"
        if k == "name":
            return self.name_value(e[1])
        if k == "next":
            self.warn("'-' used as a value")
            return "0", "i"
        if k == "call":
            lw = e[1].lower()
            return self.call(e[1], e[2]), (BUILTINS[lw][1] if lw in BUILTINS else "i")
        if k == "un":
            op, a = e[1], e[2]
            if op == "!":
                ac, at = self.expr(a)
                if at == "b":
                    return f"not {self.paren(ac)}", "b"
                return f"({ac} == 0)", "b"
            if op == "-":
                return f"-{self.paren(self.as_int(a))}", "i"
            return f"~{self.paren(self.as_int(a))}", "i"
        if k == "bin":
            op, a, b = e[1], e[2], e[3]
            if op in ("&&", "||"):
                lo = "and" if op == "&&" else "or"
                return f"{self.paren(self.as_bool(a))} {lo} {self.paren(self.as_bool(b))}", "b"
            if op in ("==", "!=", "<", ">", "<=", ">="):
                ac, at = self.expr(a)
                bc, bt = self.expr(b)
                if at == "b":
                    ac = f"({ac} and 1 or 0)"
                if bt == "b":
                    bc = f"({bc} and 1 or 0)"
                lop = "~=" if op == "!=" else op
                return f"{self.paren(ac)} {lop} {self.paren(bc)}", "b"
            ac, at = self.expr(a)
            bc, bt = self.expr(b)
            if op in ("|", "&") and at == "b" and bt == "b":
                lo = "or" if op == "|" else "and"
                return f"{self.paren(ac)} {lo} {self.paren(bc)}", "b"
            if op == "+" and (at == "s" or bt == "s"):
                return f"{self.paren(ac)} .. {self.paren(bc)}", "s"
            if at == "b":
                ac = f"({ac} and 1 or 0)"
            if bt == "b":
                bc = f"({bc} and 1 or 0)"
            lop = {"/": "//", "^": "~"}.get(op, op)
            return f"{self.paren(ac)} {lop} {self.paren(bc)}", "i"
        raise ValueError(e)

    @staticmethod
    def paren(c):
        """Parenthesize unless c is an atom: name, field, number, string or a call."""
        if re.fullmatch(r"-?\d+|[A-Za-z_][\w.]*(\[\"[^\"]*\"\])?|\"(?:[^\"\\\\]|\\\\.)*\"", c):
            return c
        m = re.match(r"[A-Za-z_][\w.]*\(", c)
        if m:
            depth = 0
            for i in range(m.end() - 1, len(c)):
                depth += c[i] == "("
                depth -= c[i] == ")"
                if depth == 0:
                    if i == len(c) - 1 and '"' not in c:
                        return c
                    break
        return f"({c})"

    # ---- statements -> list of Lua lines
    def goto(self, target):
        if target in self.labels:
            return [f"goto {self.labels[target]}"]
        self.warn(f"goto {target}: no such label (Athena ends the script here)")
        return [f"do return end --[[ goto {target}: no such label in the source (Athena ends the script) ]]"]

    def stmt(self, st):
        _, name, args, line = st
        lw = name.lower()
        if lw == "mes":
            if not args:
                return ['mes ""']
            c, t = self.expr(args[0])
            return [f"mes {c}" if args[0][0] == "str" else f"mes({c})"]
        if lw == "next":
            return ["next()"]
        if lw == "close":
            return ["close()", "@RETURN"]
        if lw == "end":
            return ["@RETURN"]
        if lw == "goto":
            if not args or args[0][0] != "name":
                self.warn("goto without label")
                return ["@RETURN"]
            return self.goto(args[0][1])
        if lw == "set":
            if len(args) < 2 or args[0][0] != "name":
                self.warn(f"bad set at line {line}")
                return [f"-- bad set at line {line}"]
            vc, vt = self.expr(args[1])
            if vt == "b":
                vc = f"({vc} and 1 or 0)"
            return [f"{self.var(args[0][1])} = {vc}"]
        if lw == "input":
            if not args or args[0][0] != "name":
                return ["input_number()"]
            return [f"{self.var(args[0][1])} = input_number()"]
        if lw == "menu":
            opts, targets = [], []
            for i in range(0, len(args), 2):
                opts.append(self.arg(args[i]))
                targets.append(args[i + 1] if i + 1 < len(args) else ("next",))
            out = [f"c = menu({', '.join(opts)})"]
            if self.uses_l15:
                out.append("tmp.l15 = c")
            conds = []
            for i, t in enumerate(targets, 1):
                if t[0] == "next":
                    continue
                if t[0] == "name" and t[1] in self.labels:
                    conds.append((i, f"goto {self.labels[t[1]]}"))
                else:
                    self.warn(f"menu target {t} is not a label")
                    conds.append((i, "do return end"))
            if conds:
                s = "if " + " elseif ".join(f"c == {i} then {g}" for i, g in conds) + " end"
                out.append(s)
            return out
        if lw == "else":
            # thief.txt: `if(JobLevel < 10) goto A; else(q==0) goto B;` with B on the next line;
            # a later dialect's else, which 817 stopped at. As an if, it means what was meant.
            self.warn(f"'else' at line {line} (not in 817) translated as 'if'")
            lw = "if"
        if lw == "if":
            if len(args) < 2:
                self.warn(f"bad if at line {line}")
                return [f"-- bad if at line {line}"]
            cond = self.as_bool(args[0])
            if args[1][0] != "name":
                self.warn(f"if without a command at line {line}")
                return [f"-- bad if at line {line}"]
            inner = self.stmt(("stmt", args[1][1], args[2:], line))
            inner = ["return" if x == "@RETURN" else x for x in inner]
            if inner and inner[-1] == "return" and len(inner) > 1:
                pass
            return [f"if {cond} then " + " ".join(inner) + " end"]
        if lw in BUILTINS:
            return [self.call(name, args)]
        if lw in NOOP_COMMANDS:
            return [f"-- {name} (jAthena label marker; 817 stopped here, we don't)"]
        self.unknown.setdefault(lw, []).append(f"{self.npc.file}:{line}")
        self.warn(f"unknown command {name} at line {line}")
        return [f"athena_unknown({lua_str(name)}{''.join(', ' + self.arg(a) for a in args)})"]

    def run(self):
        lines = []
        events = [w for w in self.labels if w.startswith("On")]
        for st in self.body:
            if st[0] == "label":
                lines.append(f"::{self.labels[st[1]]}::")
            else:
                for x in self.stmt(st):
                    lines.append(x)
        # resolve @RETURN: drop when it's the last statement or followed only by nothing
        out = []
        for i, x in enumerate(lines):
            if x == "@RETURN":
                rest = [y for y in lines[i + 1:]]
                if not rest:
                    continue
                out.append("do return end")
            else:
                out.append(x)
        head = []
        if self.uses_menu:
            head.append("local c")
        if events:
            head.append("if at then")
            for j, ev in enumerate(events):
                kw = "if" if j == 0 else "elseif"
                head.append(f"  {kw} at == {lua_str(ev)} then goto {self.labels[ev]}")
            head.append("  end")
            head.append("end")
        return head + out, events


# ------------------------------------------------------------------ files
HEADER = re.compile(r"^([^\t]+)\t([^\t]+)\t([^\t]*)\t(.*)$")


def split_npcs(text):
    """Yield (line_no, header_cols, body_text or None)."""
    lines = text.split("\n")
    i = 0
    while i < len(lines):
        raw = lines[i]
        line = raw.lstrip()
        if line.startswith("//") or not line.strip():
            i += 1
            continue
        m = HEADER.match(line.rstrip())
        if not m:
            i += 1
            continue
        w1, w2, w3, w4 = m.groups()
        if w2.strip().lower() != "script":
            i += 1
            continue
        start = i + 1
        buf = ""
        if "{" in raw:
            buf = raw[raw.index("{"):]
        body_line = start
        while True:
            if buf.rstrip().endswith("}"):
                break
            i += 1
            if i >= len(lines):
                break
            if not buf:
                if "{" in lines[i]:
                    buf = lines[i][lines[i].index("{"):]
                    body_line = i + 1
            else:
                buf += "\n" + lines[i]
        yield start, (w1, w2, w3, w4), buf, body_line
        i += 1


def header_info(cols):
    w1, _, w3, w4 = cols
    w4 = w4.strip()
    if w4.endswith("{"):
        w4 = w4[:-1].rstrip()
    w4 = w4.rstrip(",")
    name, exname = w3, w3
    if "::" in w3:
        name, exname = w3.split("::", 1)
    pos = w1.strip()
    if pos == "-":
        mp, x, y, d = "-", 0, 0, 0
    else:
        m = re.match(r"([^,]+?)(?:\.gat)?,(\d+),(\d+),(\d+)", pos)
        if not m:
            return None
        mp, x, y, d = m.group(1), int(m.group(2)), int(m.group(3)), int(m.group(4))
        mp = MAP_ALIAS.get(mp, mp)
    nums = [n.strip() for n in w4.split(",") if n.strip()]
    try:
        sprite = int(nums[0]) if nums else -1
    except ValueError:
        return None
    touch = None
    if len(nums) >= 3:
        touch = (int(nums[1]), int(nums[2]))
    return {"map": mp, "x": x, "y": y, "dir": d, "sprite": sprite, "name": name.strip(), "id": exname.strip(),
            "touch": touch}


_WORLD = None


def world():
    """What mod0631 already places (scripts/import/warps.lua, spawns.lua): warps as
    (map, x, y, target map), spawns as (map, monster)."""
    global _WORLD
    if _WORLD is None:
        warps, spawns = set(), set()
        f = ROOT / "scripts/import/warps.lua"
        g = ROOT / "scripts/import/spawns.lua"
        if not f.exists() or not g.exists():
            sys.exit("run tools/import/athena_import.py first: scripts/import/warps.lua and spawns.lua are needed")
        for m in re.finditer(r'warp \{ map = "([^"]+)", x = (\d+), y = (\d+), .*?to = \{ "([^"]+)"', f.read_text(encoding="utf-8")):
            warps.add((m.group(1), int(m.group(2)), int(m.group(3)), m.group(4)))
        for m in re.finditer(r'spawn \{ map = "([^"]+)", .*?mob = (\d+),', g.read_text(encoding="utf-8")):
            spawns.add((m.group(1), int(m.group(2))))
        _WORLD = (warps, spawns)
    return _WORLD


def convert_file(key, out_rel, guard, note, unknown, stats):
    src, rel = key.split(":", 1) if ":" in key else ("817", key)
    base, src_desc = SOURCES[src]
    text = read(base / rel)
    fixed = []
    if src == "817":
        lines = text.split("\n")
        for (frel, fline), (old, new, why) in SOURCE_FIXES.items():
            if frel != rel:
                continue
            if old not in lines[fline - 1]:
                sys.exit(f"{rel}:{fline}: source fix does not apply: {old!r} not on that line")
            lines[fline - 1] = lines[fline - 1].replace(old, new)
            fixed.append(f"-- source fix ({rel}:{fline}): {old} -> {new}: {why}")
        text = "\n".join(lines)
    parts = [f"-- Generated by tools/import/athena_npc.py from {src_desc} {rel}.",
             "-- Tier athena2003: Athena-family emulator script, contemporary with the client; dialogue is",
             "-- the emulator authors' (often fan-translated), not Gravity's. GPL, like its source.",
             "-- Don't edit: override in scripts/npc/ (npc{replaces=}, wrap_npc, remove_npc) and list the",
             "-- change in docs/DEVIATIONS.md."]
    if note:
        parts.append(f"-- Note: {note}")
    parts.extend(fixed)
    if guard:
        parts.append(f"if not content({lua_str(guard)}) then return end")
    parts.append("")
    n_ok = 0
    for line, cols, body, body_line in split_npcs(text):
        if f"{rel}:{line}" in SKIP_NPC:
            parts.append(f"-- line {line}: skipped: {SKIP_NPC[f'{rel}:{line}']}")
            parts.append("")
            continue
        info = header_info(cols)
        npc = Npc(rel, line, cols)
        if info is None:
            parts.append(f"-- line {line}: unparsed header {cols!r}")
            stats["bad_header"] += 1
            continue
        try:
            ast = parse_body(body, body_line)
        except ParseError as e:
            parts.append(f"-- line {line}: {info['name']}: parse error ({e}); Athena 817 would refuse to start")
            stats["parse_error"] += 1
            print(f"{rel}:{line}: parse error: {e}", file=sys.stderr)
            continue
        tr = Tr(ast, npc, unknown)
        code, events = tr.run()
        fields = [f"map = {lua_str(info['map'])}", f"x = {info['x']}", f"y = {info['y']}", f"dir = {info['dir']}",
                  f"sprite = {info['sprite']}", f"name = {lua_str(info['name'])}"]
        if info["id"] != info["name"]:
            fields.append(f"id = {lua_str(info['id'])}")
        if info["touch"]:
            fields.append(f"touch = {{ {info['touch'][0]}, {info['touch'][1]} }}")
        if events:
            fields.append("events = { " + ", ".join(lua_str(e) for e in events) + " }")
        fields.append(f"src = {lua_str(f'{src}:{rel}:{line}')}")
        for w in npc.warnings:
            parts.append(f"-- warning ({rel}:{line}): {w}")
        stats["warnings"] += len(npc.warnings)
        parts.append("npc {")
        parts.append("  " + ", ".join(fields[:6]) + ",")
        rest = fields[6:]
        if rest:
            parts.append("  " + ", ".join(rest) + ",")
        fn = "function(at)" if events else "function()"
        parts.append(f"  on_click = {fn}")
        for c in code:
            parts.append("    " + c)
        parts.append("  end,")
        parts.append("}")
        parts.append("")
        n_ok += 1
    stats["npcs"] += n_ok
    # The monsters and warps a script file places itself (the Swordman course's Chonchons and
    # its three bridges, the Priest test's Wraiths). The world's own spawns and warps are
    # mod0631's (athena_import.py): what it already places is left to it.
    extra = []
    for ln, line in enumerate(text.split("\n"), 1):
        if line.lstrip().startswith("//"):
            continue
        cols = line.rstrip().split("\t")
        if len(cols) < 4:
            continue
        kind, last = cols[1].strip(), cols[-1].strip()
        if kind == "monster":
            m = re.match(r"([\w\-]+?)(?:\.gat)?,(\d+),(\d+),(\d+),(\d+)$", cols[0].strip())
            d = [x.strip() for x in last.split(",")]
            if not m or len(d) < 2 or not d[0].isdigit():
                sys.exit(f"{rel}:{ln}: monster line not understood")
            vals = [int(x) if x.lstrip("-").isdigit() else 0 for x in d] + [0] * 4
            mp = MAP_ALIAS.get(m.group(1), m.group(1))
            if (mp, vals[0]) in world()[1]:
                extra.append(f"-- line {ln}: monster {vals[0]} on {mp}: mod0631 places it (scripts/import/spawns.lua)")
                continue
            if re.search(r"_[2-9]-\d$", mp):
                extra.append(f"-- line {ln}: monster {vals[0]} on {mp}: a parallel copy of the map (tools/import/maps.py)")
                continue
            extra.append(f"spawn {{ map = {lua_str(mp)}, x = {m.group(2)}, y = {m.group(3)}, xs = {m.group(4)}, "
                         f"ys = {m.group(5)}, mob = {vals[0]}, count = {vals[1]}, delay = {vals[2]}, "
                         f"delay2 = {vals[3]} }} -- {rel}:{ln}")
        elif kind == "warp":
            m = re.match(r"([\w\-]+?)(?:\.gat)?,(\d+),(\d+)", cols[0].strip())
            d = re.match(r"(\d+),(\d+),([\w\-]+?)(?:\.gat)?,(\d+),(\d+)$", last)
            if not m or not d:
                sys.exit(f"{rel}:{ln}: warp line not understood")
            mp, to = MAP_ALIAS.get(m.group(1), m.group(1)), MAP_ALIAS.get(d.group(3), d.group(3))
            x, y = int(m.group(2)), int(m.group(3))
            if any(w[0] == mp and w[3] == to and max(abs(w[1] - x), abs(w[2] - y)) <= 3 for w in world()[0]):
                extra.append(f"-- line {ln}: warp at {mp} {x},{y} to {to}: mod0631 has this exit within 3 cells "
                             "(scripts/import/warps.lua)")
                continue
            extra.append(f"warp {{ map = {lua_str(mp)}, x = {m.group(2)}, y = {m.group(3)}, "
                         f"span = {{ {d.group(1)}, {d.group(2)} }}, to = {{ {lua_str(to)}, {d.group(4)}, {d.group(5)} }} }}"
                         f" -- {rel}:{ln}")
    if extra:
        parts.append("-- Monsters and warps this file places.")
        parts.extend(extra)
        parts.append("")
    stats["placed"] = stats.get("placed", 0) + sum(1 for e in extra if not e.startswith("--"))
    out = ROOT / "scripts/import/npc" / out_rel
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text("\n".join(parts).rstrip() + "\n", encoding="utf-8")
    return n_ok


def write_consts():
    lines = ["-- Generated by tools/import/athena_npc.py from " + SOURCES["817"][1] + " db/const.txt (GPL).",
             "-- Athena script constants (job ids, map flags, effect ids...) for the imported NPCs.", ""]
    for lw, (nm, val, typ) in sorted(CONSTS.items(), key=lambda kv: kv[1][0].lower()):
        if typ == 0 and ident_ok(nm):
            lines.append(f"{nm} = {val}")
    (ROOT / "scripts/import/athena_const.lua").write_text("\n".join(lines) + "\n", encoding="utf-8")


def main():
    unknown = {}
    stats = {"npcs": 0, "parse_error": 0, "bad_header": 0, "warnings": 0}
    old = ROOT / "scripts/import/npc"
    if old.exists():
        for f in old.rglob("*.lua"):
            f.unlink()
    for rel, (out_rel, guard, note) in FILES.items():
        n = convert_file(rel, out_rel, guard, note, unknown, stats)
        if REPORT:
            print(f"{rel}: {n} npcs")
    write_consts()
    print(stats)
    if unknown:
        print("unknown commands:")
        for k, v in sorted(unknown.items(), key=lambda kv: -len(kv[1])):
            print(f"  {k}: {len(v)} ({', '.join(v[:3])})")


if __name__ == "__main__":
    main()
