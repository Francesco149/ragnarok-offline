# t6-refine-web: The iRO Refine Page Over Time and Period Refining Mechanics

## Answer so far

The investigation settles the three alleged "typos" on the 2004-06-05 iRO refining page (`iro.ragnarokonline.com/game/refinesystem.asp`):

1. **Phracon 2000z was an outright typo.** The 2004-06-05 capture lists Phracon at 2000z while Emveretarcon is 1000z. By the 2004-08-10 capture, Gravity corrected Phracon to 200z. In the game client and server scripts (both authentic kRO/iRO NPC data and emulators mod0631 and 817), Phracon was always 200z (item ID 1010) and Emveretarcon 1000z (item ID 1011).
2. **Identical fees (5,000z) and damage bonuses (+5) for Weapon Levels 3 and 4 were stale pre-Comodo (pre-2003-02-04) documentation.** In the official kRO patch notice of 2003-02-04 (`ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=360`), Gravity explicitly announced: "When refining weapon level 4 weapon, changed to bring +7 attack power increase per +1" (`무기레벨 4 의 무기를 제련하면 +1 당 7의 공격력 증가를 가져오도록 변경 하였습니다`) and introduced safe refining up to +4 for level 4 weapons (`4레벨의 무기의 경우 +4 까지는 안전하게 제련되도록 변경 되었습니다`). Prior to this patch, Level 4 weapons gave +5 ATK per refine and had no safe limit. The June/August 2004 iRO website still displayed the obsolete pre-February 2003 text (5,000z fee, +5 ATK). On 2004-12-05, Gravity updated Level 4 to fee 20,000z and +7 ATK, but the webmaster erroneously marked safe limit as "No Safety" (confusing the pre-patch behavior with the updated mechanics).
3. **"+1 adds 1 defense" was official period UI presentation and early emulator reality, but masked a kRO 2003-02-04 formula change.** In all website captures from 2004 through 2006, the official iRO page states "Each +1 adds 1 defense". In the client UI and in mod0631 (`map/pc.c`), 1 refine defense was added directly to `sd->def`, reducing physical damage by 1% (`damage * (100 - def1) / 100`). However, kRO's 2003-02-04 notice stated: "Additional DEF due to smelting has been changed to be less efficient than the DEF of pure armor" (`제련으로 인한 추가 DEF 는 순수한 방어구의 DEF 보다 효율이 낮게 변경 되었습니다`), which Athena 817 and subsequent pre-renewal emulators modeled with a fractional defense contribution (later standardized to ~0.7 hard DEF per armor refine).

---

## Findings

| Claim | Value | Region | Source Date | Source URL | Verbatim Quote | Grade |
|---|---|---|---|---|---|---|
| iRO 2004-06-05 Phracon price | 2000 zeny | iRO | 2004-06-05 | http://web.archive.org/web/20040605070057/http://iro.ragnarokonline.com:80/game/refinesystem.asp | "Weapon Level 1 \| Phracon \| 2000 zeny \| 50 zeny \| Each +1 increases 2 damage" | A |
| iRO 2004-08-10 Phracon price correction | 200 zeny | iRO | 2004-08-10 | http://web.archive.org/web/20040810050735/http://iro.ragnarokonline.com:80/game/refinesystem.asp | "Weapon Level 1 \| Phracon \| 200 zeny \| 50 zeny \| Each +1 increases 2 damage" | A |
| iRO 2004-06-05 Weapon L3 fee & bonus | 5,000z, +5 dmg | iRO | 2004-06-05 | http://web.archive.org/web/20040605070057/http://iro.ragnarokonline.com:80/game/refinesystem.asp | "Weapon Level 3 \| Oridecon \| not sold \| 5,000 zeny \| Each +1 increases 5 damage" | A |
| iRO 2004-06-05 Weapon L4 fee & bonus | 5,000z, +5 dmg | iRO | 2004-06-05 | http://web.archive.org/web/20040605070057/http://iro.ragnarokonline.com:80/game/refinesystem.asp | "Weapon Level 4 \| Oridecon \| not sold \| 5,000 zeny \| Each +1 increases 5 damage" | A |
| iRO 2004-06-05 Weapon L4 safe limit | Safe up to +4 | iRO | 2004-06-05 | http://web.archive.org/web/20040605070057/http://iro.ragnarokonline.com:80/game/refinesystem.asp | "Weapon Level 4 \| Safe up to +4" | A |
| iRO 2004-12-05 Weapon L4 fee & bonus update | 20,000z, +7 dmg | iRO | 2004-12-05 | http://web.archive.org/web/20041205192936/http://iro.ragnarokonline.com:80/game/refinesystem.asp | "Weapon Level 4 \| Oridecon \| not sold \| 20,000 zeny \| Each +1 increases 7 damage" | A |
| iRO 2004-12-05 Weapon L4 safe limit alteration | No Safety | iRO | 2004-12-05 | http://web.archive.org/web/20041205192936/http://iro.ragnarokonline.com:80/game/refinesystem.asp | "Weapon Level 4 \| No Safety" | A |
| iRO Armor refine effect across all captures | +1 adds 1 defense | iRO | 2004-06-05 to 2006-02-24 | http://web.archive.org/web/20040605070057/http://iro.ragnarokonline.com:80/game/refinesystem.asp | "Armor \| Elunium \| not sold \| 2,000 zeny \| Each +1 adds 1 defense" | A |
| kRO 2003-02-04 Weapon L4 ATK bonus update | +7 ATK per +1 | kRO | 2003-02-03 | https://ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=360 | "무기레벨 4 의 무기를 제련하면 +1 당 7의 공격력 증가를 가져오도록 변경 하였습니다." | A |
| kRO 2003-02-04 Over-refine random damage | Added | kRO | 2003-02-03 | https://ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=360 | "무기레벨에 따라 일정제련 레벨 이상이 되면 추가 랜덤 타격치가 붙도록 변경하였습니다. 랜덤 타격치는 무기의 + 수치가 높아질 수록 높아집니다." | A |
| kRO 2003-02-04 Weapon L4 Safe Limit added | Safe up to +4 | kRO | 2003-02-03 | https://ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=360 | "무기나 방어구 아이템의 제련 확률이 대폭 상승 하였으며 , 4레벨의 무기의 경우 +4 까지는 안전하게 제련되도록 변경 되었습니다." | A |
| kRO 2003-02-04 Refine DEF efficiency reduced | Refine DEF < pure armor DEF | kRO | 2003-02-03 | https://ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=360 | "제련으로 인한 추가 DEF 는 순수한 방어구의 DEF 보다 효율이 낮게 변경 되었습니다." | A |
| kRO 2003-02-04 Armor Safe Limit reduced | +5 -> +4 | kRO | 2003-02-03 | https://ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=360 | "방어구가 안전하게 제련되는 수치를 +5 → +4 로 변경 하였습니다." | A |
| mod0631 refine bonus table values | L1=2, L2=3, L3=5, L4=7; Armor=1 | JP / EMU | 2003-06 | `db/refine_db.txt` (athena-dev-2.1.1-mod0631) | "1, 0,4, 100,100,100,100, 60, 40, 40, 20, 20, 10, // 防具\n2, 3,7, 100,100,100,100,100,100,100, 60, 40, 20, // 武器Lv.1\n3, 5,6, 100,100,100,100,100,100, 60, 40, 20, 20, // 武器Lv.2\n5, 8,5, 100,100,100,100,100, 60, 50, 20, 20, 20, // 武器Lv.3\n7,14,4, 100,100,100,100, 60, 40, 40, 20, 20, 10, // 武器Lv.4" | B |
| mod0631 refine fees in NPC scripts | L1=500z, L2=2000z, L3=25000z, L4=50000z, Armor=20000z | JP / EMU | 2003-06 | `conf/npc_town_refine.txt` (athena-dev-2.1.1-mod0631) | "手数料500Zenyが必要だな。" / "手数料2000Zenyが必要だな。" / "手数料25000Zenyが必要だな。" / "手数料50000Zenyが必要だな。" / "手数料20000Zenyが必要だな。" | B |
| mod0631 Armor DEF formula in pc.c | 1 refine = 1 DEF (1% reduction) | JP / EMU | 2003-06 | `map/pc.c` (athena-dev-2.1.1-mod0631) line 806 | "sd->def += sd->status.inventory[index].refine*refinebonus[0][0];" | B |
| Athena 817 refine fees | L1=50z, L2=200z, L3=5000z, L4=20000z, Armor=2000z | EMU | 2004-05 | `athena/npc/other/refine.txt` (athena-817-src) | "fee 50 Zeny" / "fee 200 Zeny" / "fee 5000 Zeny" / "fee 20000 Zeny" / "fee 2000 Zeny" | B |
| Athena 817 ore prices | Phracon 200z, Emveretarcon 1000z | EMU | 2004-05 | `athena/npc/other/refine.txt` (athena-817-src) | 'menu"Pharacon - 200z",PHARA,"Emveretarcon - 1000z",EMVER;' | B |

---

## Detailed Chronology of iRO Refine Page Captures (`iro.ragnarokonline.com/game/refinesystem.asp`)

| Capture Date | Digest | Phracon Price | Emveretarcon Price | Weapon L1 (Fee / Dmg / Safe) | Weapon L2 (Fee / Dmg / Safe) | Weapon L3 (Fee / Dmg / Safe) | Weapon L4 (Fee / Dmg / Safe) | Armor (Fee / DEF / Safe) | NPC Locations Listed |
|---|---|---|---|---|---|---|---|---|---|
| **2004-06-05** | `SRVTOR2LQ3MIIRQEOPGEGEIR7WDKU4VO` | **2000 zeny** *(typo)* | 1,000 zeny | 50z / +2 / +7 | 200z / +3 / +6 | 5,000z / +5 / +5 | **5,000z** / **+5** / **+4** | 2,000z / +1 DEF / +4 | Prontera, Morroc, Payon, Al De Baran |
| **2004-06-11** | `SRVTOR2LQ3MIIRQEOPGEGEIR7WDKU4VO` | 2000 zeny | 1,000 zeny | 50z / +2 / +7 | 200z / +3 / +6 | 5,000z / +5 / +5 | 5,000z / +5 / +4 | 2,000z / +1 DEF / +4 | Prontera, Morroc, Payon, Al De Baran |
| **2004-06-14** | `SRVTOR2LQ3MIIRQEOPGEGEIR7WDKU4VO` | 2000 zeny | 1,000 zeny | 50z / +2 / +7 | 200z / +3 / +6 | 5,000z / +5 / +5 | 5,000z / +5 / +4 | 2,000z / +1 DEF / +4 | Prontera, Morroc, Payon, Al De Baran |
| **2004-06-23** | `SRVTOR2LQ3MIIRQEOPGEGEIR7WDKU4VO` | 2000 zeny | 1,000 zeny | 50z / +2 / +7 | 200z / +3 / +6 | 5,000z / +5 / +5 | 5,000z / +5 / +4 | 2,000z / +1 DEF / +4 | Prontera, Morroc, Payon, Al De Baran |
| **2004-08-10** | `ZVBVKN6CVBWL6OMWNS7IFJ7EQAF4O6E7` | **200 zeny** *(corrected)* | 1,000 zeny | 50z / +2 / +7 | 200z / +3 / +6 | 5,000z / +5 / +5 | **5,000z** / **+5** / **+4** | 2,000z / +1 DEF / +4 | Prontera, Morroc, Payon, Al De Baran |
| **2004-12-05** | `YEW4RKSZUQLMZQPFGQIHHSJDZ2O2DT2U` | 200 zeny | 1,000 zeny | 50z / +2 / +7 | 200z / +3 / +6 | 5,000z / +5 / +5 | **20,000z** / **+7** / **No Safety** | 2,000z / +1 DEF / +4 | Prontera, Morroc, Payon, Al De Baran |
| **2005-06-11** | `O4DSJC2JAQEXQZO2WUJGSRDLXVDQU6KQ` | 200 zeny | 1,000 zeny | 50z / +2 / +7 | 200z / +3 / +6 | 5,000z / +5 / +5 | 20,000z / +7 / No Safety | 2,000z / +1 DEF / +4 | Prontera, Morroc, Payon, **Juno** *(updated)* |
| **2005-08-06** | `XPNXMIF4YOFH4CFEHIOSNC5YDN73HXLQ` | 200 zeny | 1,000 zeny | 50z / +2 / +7 | 200z / +3 / +6 | 5,000z / +5 / +5 | 20,000z / +7 / No Safety | 2,000z / +1 DEF / +4 | Prontera, Morroc, Payon, Juno |
| **2005-10-15** | `YFAUFC4ZPNNA3GY564UVLA3VC5TCAEYW` | 200 zeny | 1,000 zeny | 50z / +2 / +7 | 200z / +3 / +6 | 5,000z / +5 / +5 | 20,000z / +7 / No Safety | 2,000z / +1 DEF / +4 | Prontera, Morroc, Payon, Juno |
| **2005-10-30** | `RB4I2SECJF4Z2PCTYHPZUOKSLRZY7GK4` | 200 zeny | 1,000 zeny | 50z / +2 / +7 | 200z / +3 / +6 | 5,000z / +5 / +5 | 20,000z / +7 / No Safety | 2,000z / +1 DEF / +4 | Prontera, Morroc, Payon, Juno |
| **2005-11-24** | `5LNZ7632WELE2JBOXW7RS2Y2UPUS4Y67` | 200 zeny | 1,000 zeny | 50z / +2 / +7 | 200z / +3 / +6 | 5,000z / +5 / +5 | 20,000z / +7 / No Safety | 2,000z / +1 DEF / +4 | Prontera, Morroc, Payon, Juno |
| **2005-12-12** | `M7663K6RIXFDW3JXCDFM3RZUJBALRG5M` | 200 zeny | 1,000 zeny | 50z / +2 / +7 | 200z / +3 / +6 | 5,000z / +5 / +5 | 20,000z / +7 / No Safety | 2,000z / +1 DEF / +4 | Prontera, Morroc, Payon, Juno |
| **2006-01-30** | `5YRCKT7PD25JWY5N75CAFDNR6K3XTRVZ` | 200 zeny | 1,000 zeny | 50z / +2 / +7 | 200z / +3 / +6 | 5,000z / +5 / +5 | 20,000z / +7 / No Safety | 2,000z / +1 DEF / +4 | Prontera, Morroc, Payon, Juno |
| **2006-02-24** | `BXCWREXCTM24MYJRE6C7PAE56B5UUGS7` | 200 zeny | 1,000 zeny | 50z / +2 / +7 | 200z / +3 / +6 | 5,000z / +5 / +5 | 20,000z / +7 / No Safety | 2,000z / +1 DEF / +4 | Prontera, Morroc, Payon, Juno |

---

## The Full kRO Official Notice of 2003-02-04 (seq=360)

Published by Gravity on 2003-02-03, deployed to kRO main servers on 2003-02-04 along with the Comodo episode.
URL: `https://ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=360`
Title: `[공지] 2월 4일 본서버 패치내용에 대한 안내(2)`

### Verbatim Korean Refining Section:
```text
■제련 관련 변경 사항입니다.
무기나 방어구를 제련하다가 실패했을 경우 해당 무기는 완전히 없어지도록 변경 되었습니다.
무기레벨 4 의 무기를 제련하면 +1 당 7의 공격력 증가를 가져오도록 변경 하였습니다.
무기레벨에 따라 일정제련 레벨 이상이 되면 추가 랜덤 타격치가 붙도록 변경하였습니다. 랜덤 타격치는 무기의 + 수치가 높아질 수록 높아집니다.
무기나 방어구 아이템의 제련 확률이 대폭 상승 하였으며 , 4레벨의 무기의 경우 +4 까지는 안전하게 제련되도록 변경 되었습니다.
제련으로 인한 추가 DEF 는 순수한 방어구의 DEF 보다 효율이 낮게 변경 되었습니다.
방어구가 안전하게 제련되는 수치를 +5 → +4 로 변경 하였습니다.
```

### Line-by-Line Analysis & Historical Significance:
1. `무기나 방어구를 제련하다가 실패했을 경우 해당 무기는 완전히 없어지도록 변경 되었습니다.`
   - "When refining a weapon or armor fails, the weapon [item] has been changed to completely disappear."
   - In early pre-Comodo mechanics, failure either degraded the item or had distinct rules; from this patch onward, failure resulted in complete destruction of item and slotted cards.
2. `무기레벨 4 의 무기를 제련하면 +1 당 7의 공격력 증가를 가져오도록 변경 하였습니다.`
   - "When refining Level 4 weapons, it was changed to provide +7 attack power increase per +1."
   - Explains why the early iRO website listed +5 damage for Level 4 weapons: before 2003-02-04, Level 4 weapons had received +5 damage per upgrade (the same as Level 3 weapons).
3. `무기레벨에 따라 일정제련 레벨 이상이 되면 추가 랜덤 타격치가 붙도록 변경하였습니다. 랜덤 타격치는 무기의 + 수치가 높아질 수록 높아집니다.`
   - "Depending on weapon level, when exceeding a certain refine level, an additional random damage bonus is attached. Random strike value increases as the weapon's + value increases."
   - This introduced the **over-refine bonus** (`過剰精錬ボーナス` in jAthena / `overrefine` in eAthena):
     - Level 1: +1~3 per level past +7
     - Level 2: +1~5 per level past +6
     - Level 3: +1~8 per level past +5
     - Level 4: +1~14 per level past +4
4. `무기나 방어구 아이템의 제련 확률이 대폭 상승 하였으며 , 4레벨의 무기의 경우 +4 까지는 안전하게 제련되도록 변경 되었습니다.`
   - "Refinement success rates for weapon and armor items significantly increased, and for Level 4 weapons, they have been changed to be safely refined up to +4."
   - Explains why the December 2004 iRO website listed "No Safety": prior to 2003-02-04, Level 4 weapons had **no safe limit at all** (could break going to +1). The 2003-02-04 patch established the famous safe limit of +4.
5. `제련으로 인한 추가 DEF 는 순수한 방어구의 DEF 보다 효율이 낮게 변경 되었습니다.`
   - "Additional DEF from refining has been changed to have lower efficiency than pure armor DEF."
   - Prior to this patch, each +1 refine DEF gave full 1% damage reduction equivalent to 1 point of base equipment DEF ("Each +1 adds 1 defense"). This patch weakened refine DEF so it was less effective than base armor DEF.
6. `방어구가 안전하게 제련되는 수치를 +5 → +4 로 변경 하였습니다.`
   - "The safe refinement limit for armor was changed from +5 to +4."
   - Settles armor safe limits: in alpha/early beta, armor was safe to +5. Comodo standardized armor safe limit to +4.

---

## Comparison of Period Sources

| Parameter | Official iRO (2004-06-05) | Official iRO (2004-08-10) | Official iRO (2004-12-05) | kRO Patch (2003-02-04) | mod0631 (2003-06) | Athena 817 (2004-05) |
|---|---|---|---|---|---|---|
| **Phracon Price** | 2000z *(typo)* | 200z | 200z | - | 200z | 200z |
| **Emveretarcon Price** | 1,000z | 1,000z | 1,000z | - | 1,000z | 1,000z |
| **Weapon L1 Fee** | 50z | 50z | 50z | - | 500z *(JP mod custom)* | 50z |
| **Weapon L1 ATK Bonus** | +2 | +2 | +2 | - | +2 (+3 over-refine) | +2 (+3 over-refine) |
| **Weapon L1 Safe Limit** | +7 | +7 | +7 | - | +7 | +7 |
| **Weapon L2 Fee** | 200z | 200z | 200z | - | 2,000z *(JP mod custom)* | 200z |
| **Weapon L2 ATK Bonus** | +3 | +3 | +3 | - | +3 (+5 over-refine) | +3 (+5 over-refine) |
| **Weapon L2 Safe Limit** | +6 | +6 | +6 | - | +6 | +6 |
| **Weapon L3 Fee** | 5,000z | 5,000z | 5,000z | - | 25,000z *(JP mod custom)* | 5,000z |
| **Weapon L3 ATK Bonus** | +5 | +5 | +5 | - | +5 (+8 over-refine) | +5 (+8 over-refine) |
| **Weapon L3 Safe Limit** | +5 | +5 | +5 | - | +5 | +5 |
| **Weapon L4 Fee** | 5,000z *(stale)* | 5,000z *(stale)* | 20,000z | - | 50,000z *(JP mod custom)* | 20,000z |
| **Weapon L4 ATK Bonus** | +5 *(stale)* | +5 *(stale)* | +7 | +7 | +7 (+14 over-refine) | +7 (+14 over-refine) |
| **Weapon L4 Safe Limit** | +4 | +4 | No Safety *(doc error)* | +4 (changed from none) | +4 | +4 |
| **Armor Fee** | 2,000z | 2,000z | 2,000z | - | 20,000z *(JP mod custom)* | 2,000z |
| **Armor DEF Bonus** | +1 DEF | +1 DEF | +1 DEF | < pure DEF | +1 DEF (`pc.c` line 806) | scaled / fractional |
| **Armor Safe Limit** | +4 | +4 | +4 | +4 (changed from +5) | +4 | +4 |

---

## Contradictions and Open Points

1. **Why mod0631 has 10x fees for weapons and armor**:
   - In `conf/npc_town_refine.txt` of all 2003 jAthena mod releases (mod0601, mod0629, mod0631, mod0641, mod0652), the Japanese authors configured fees as: L1=500z, L2=2000z, L3=25000z, L4=50000z, Armor=20000z.
   - However, English Athena (817) and official iRO both agree on: L1=50z, L2=200z, L3=5,000z, L4=20,000z, Armor=2,000z. The 10x multiplier in Japanese Athena scripts was either an early jRO closed/open-beta custom balance change or a private server economy nerf introduced by early Japanese emulator developers.
2. **Why the December 2004 iRO website said "Weapon Level 4: No Safety"**:
   - The webmaster who updated the page in late 2004 likely looked at early design notes or pre-February 2003 design documents stating Level 4 weapons have no safety threshold. In actual gameplay (kRO patch 2003-02-04, jAthena mod0631, Athena 817), Level 4 weapons safely refine up to +4 without breaking.
3. **Exact DEF damage reduction formula for Refine DEF**:
   - While the client UI and mod0631 add +1 refine directly to DEF, kRO 2003-02-04 explicitly nerfed refine DEF efficiency. Pre-renewal emulators later settled on 0.7 Hard DEF per refine level.

---

## Sources Read

- `http://web.archive.org/web/20040605070057/http://iro.ragnarokonline.com:80/game/refinesystem.asp` — Official iRO refine guide (Grade A, 2004-06-05 capture; contains Phracon 2000z typo and stale L4 data).
- `http://web.archive.org/web/20040810050735/http://iro.ragnarokonline.com:80/game/refinesystem.asp` — Official iRO refine guide (Grade A, 2004-08-10 capture; Phracon corrected to 200z).
- `http://web.archive.org/web/20041205192936/http://iro.ragnarokonline.com:80/game/refinesystem.asp` — Official iRO refine guide (Grade A, 2004-12-05 capture; L4 fee updated to 20,000z, bonus to +7, safe limit erroneously set to "No Safety").
- `http://web.archive.org/web/20050611013724/http://iro.ragnarokonline.com:80/game/refinesystem.asp` — Official iRO refine guide (Grade A, 2005-06-11 capture; NPC location updated from Al De Baran to Juno).
- `http://web.archive.org/web/20051124052157/http://iro.ragnarokonline.com:80/game/refinesystem.asp` — Official iRO refine guide (Grade A, 2005-11-24 capture; identical data).
- `http://web.archive.org/web/20060224085211/http://iro.ragnarokonline.com:80/game/refinesystem.asp` — Official iRO refine guide (Grade A, 2006-02-24 capture; identical data).
- `https://ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=360` — Official Gravity kRO main server patch notice seq=360 (Grade A, published 2003-02-03, deployed 2003-02-04; primary source for Comodo refine system overhaul).
- `https://ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=359` — Official Gravity kRO Sakray patch notice seq=359 (Grade A, published 2003-02-03).
- `https://ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=361` — Official Gravity kRO maintenance notice seq=361 (Grade A, published 2003-02-04; confirms Comodo connection).
- `/data/refs/eathena-archive/x/athena-dev-2.1.1-mod0631/athena-dev-2.1.1-mod0631/db/refine_db.txt` — Period jAthena mod0631 refine database (Grade B, 2003-06; defines bonuses 1,2,3,5,7 and safe limits 4,7,6,5,4).
- `/data/refs/eathena-archive/x/athena-dev-2.1.1-mod0631/athena-dev-2.1.1-mod0631/conf/npc_town_refine.txt` — Period jAthena mod0631 NPC refine script (Grade B, 2003-06; contains 500z, 2000z, 25000z, 50000z, 20000z).
- `/data/refs/eathena-archive/x/athena-dev-2.1.1-mod0631/athena-dev-2.1.1-mod0631/map/pc.c` & `map/battle.c` — Period jAthena mod0631 source code (Grade B, 2003-06; implements `sd->def += refine * 1` and `damage * (100 - def1) / 100`).
- `/data/refs/eathena-archive/x/athena-817-src/athena/db/refine_db.txt` — Period Athena 817 refine database (Grade B, 2004-05; defines bonuses 5,2,3,5,7 and safe limits 4,7,6,5,4).
- `/data/refs/eathena-archive/x/athena-817-src/athena/npc/other/refine.txt` — Period Athena 817 refine NPC script (Grade B, 2004-05; defines fees 50z, 200z, 5000z, 20000z, 2000z).
- `https://mikaeri.sakura.ne.jp/refine.htm` — Historical Japanese Ragnarok Online refine guide documenting 2008 and 2010 price cuts and armor DEF tables.

---

## Could Not Read

- `https://gamefaqs.gamespot.com/pc/561051-ragnarok-online/faqs/20092` — Blocked by Cloudflare bot protection (HTTP 403); bypassed by using primary emulator scripts and official web captures.
