#!/usr/bin/env python3
import csv
import xml.etree.ElementTree as ET
import sys
import os
import re
import zlib

sys.path.insert(0, "/work/research")
import grf_peek

GRF = "/data/play/RagnarokOnline/data.grf.original"
OUT_TSV = "/work/research/agy-threads/r2/t5-official-text.tsv"

def get_grf_file(path_in_grf):
    for n, cs, csa, rs, flags, pos in grf_peek.entries(GRF):
        if n.lower() == path_in_grf.lower():
            with open(GRF, "rb") as f:
                f.seek(pos)
                return zlib.decompress(f.read(csa))[:rs]
    return None

rows = []

# 1. Official Dialogue lines from RODataZone (captured 2003-12-11) and iRO guide (2003-12-28)
dialogue_records = [
    # Merchant
    ("Merchant Guild Chief Mahnsoo", "So, what brings you to the Merchant Association? Is there anything I can help you with?", "iRO", "2003-12-11", "http://web.archive.org/web/20031211220920/http://www.rodatazone.com/", "817:jobs/merchant.txt:14"),
    ("Merchant Guild Chief Mahnsoo", "Do you want to be a Merchant? Well... Alright, you'll need to fill out this application and prepare 1,000 Zeny for your Membership Fee. Oh...! If you don't have all the money, I can just take 500 Zeny now. You can pay the rest after you pass the test and earn your Merchant Guild License. So what do you think? Are you ready to join now?", "iRO", "2003-12-11", "http://web.archive.org/web/20031211220920/http://www.rodatazone.com/", "817:jobs/merchant.txt:38"),
    ("Merchant Guild Chief Mahnsoo", "Hello there, <name>. I'm pleased to tell you that I have good news! The Merchant Guild accepted your application. You've proven that you are fully qualified to become a Merchant. Congratulations! I'm very pleased that you are joining the Merchant Guild and hope that you will play an active part in Rune-Midgarts' economy. Our goal is to control 20% of the world's income! We're going to need young, eager people like you! But overall, we'll also be happy just to make loads of money. But we all know that~", "iRO", "2003-12-11", "http://web.archive.org/web/20031211220920/http://www.rodatazone.com/", "817:jobs/merchant.txt:175"),
    
    # Acolyte
    ("Father Mareusis", "What is it that you seek?", "iRO", "2003-12-11", "http://web.archive.org/web/20031211220920/http://www.rodatazone.com/", "817:jobs/acolyte.txt:8"),
    ("Father Mareusis", "Do you truly wish to become a servant of Odin?", "iRO", "2003-12-11", "http://web.archive.org/web/20031211220920/http://www.rodatazone.com/", "817:jobs/acolyte.txt:12"),
    ("Father Mareusis", "Good. I accept [Player Name]'s will to become an Acolyte. You understand that you must do penance before you can become a servant of Odin, right? Well, I will give you a mission...", "iRO", "2003-12-11", "http://web.archive.org/web/20031211220920/http://www.rodatazone.com/", "817:jobs/acolyte.txt:20"),
    ("Father Mareusis", "Oh, you've come back. Let me check and see if you are ready to serve Odin. Let's see... Hmm... Your name is on the list and you've proven your qualification. I am proud to say that you are now ready to become an Acolyte!", "iRO", "2003-12-11", "http://web.archive.org/web/20031211220920/http://www.rodatazone.com/", "817:jobs/acolyte.txt:45"),

    # Archer
    ("Archer Guildsman", "Good day. How may I help you?", "iRO", "2003-12-11", "http://web.archive.org/web/20031211220920/http://www.rodatazone.com/", "817:jobs/archer.txt:7"),
    ("Archer Guildsman", "Do you want to be an Archer? If so, you need to fill out this application form.", "iRO", "2003-12-11", "http://web.archive.org/web/20031211220920/http://www.rodatazone.com/", "817:jobs/archer.txt:15"),
    ("Archer Guildsman", "Okay, sign here. Alright, um, I'll promote you once you meet the requirements.", "iRO", "2003-12-11", "http://web.archive.org/web/20031211220920/http://www.rodatazone.com/", "817:jobs/archer.txt:24"),
    ("Archer Guildsman", "Are you [Player Name]? Excellent. Now then, let's see... I will appraise the value of the various types of trunks, which are needed to produce a Bow, that you've brought... I'll transfer these trunks to our Bow Production Department. Now that you've met the requirements, let me promote you right away! Congratulations! You are now an Archer! ... Ah, your bow has arrived from the Bow Production Department. Here, take it! It's yours~. Now, off you go. Hunt with pride, knowing you were trained by one of the best!", "iRO", "2003-12-11", "http://web.archive.org/web/20031211220920/http://www.rodatazone.com/", "817:jobs/archer.txt:42"),

    # Hunter
    ("Hunter Guildsman Sherin", "Oh, you're an Archer! It seems as if you've trained enough as an Archer... You came here to become a Hunter, right?! ... Hehe~ I was right! Let me put you in the candidate list. Let's see... Hmm... Let's start with the interview. Wait and relax a moment. No need to worry, I'll prepare everything. (gathers and flips through papers, rummage rummage) Ah! Here they are: the interview questions... First of all, my name is Sherin. Nice to meet you! Well then, shall we begin?", "iRO", "2003-12-11", "http://web.archive.org/web/20031211220920/http://www.rodatazone.com/", "817:jobs/hunter.txt:10"),
    ("Hunter Guildsman Sherin", "Oh...? You passed the job test?!! Congratulations!! Well then, I will now change your job to a Hunter. Tada~ Congratulations! You look great as a Hunter! Become a noble person and become a worthy representative of our Hunter Guild. Show your love of nature as a Hunter.", "iRO", "2003-12-11", "http://web.archive.org/web/20031211220920/http://www.rodatazone.com/", "817:jobs/hunter.txt:115"),

    # Mr Smile
    ("Smile Assistant", "Hi ~ Hi ~ This is Smile Assistance. How may I help you ?", "iRO", "2003-12-11", "http://web.archive.org/web/20031211220920/http://www.rodatazone.com/", "817:events/mr_smile.lua:8"),
    ("Smile Assistant", "National Event held by the command of His majesty Tristram the 3rd, that intends to encourage the nation of the Rune-Midgarts Kingdom to play in more enjoyable atmosphere! I am ' Smile Assistant ', .. who leads the national event under the name of Smile throughout the Rune-Midgarts Kingdom~ With simple and easy-to-get items, I can provide you ' Mr. Smile '. The requirements are followings. 10 Jellopy 10 Fluff 10 Clover. With this event Everybody will be happy and smile, getting together with other people, And will try to make Ragnarok the most enjoyable game in the world.", "iRO", "2003-12-11", "http://web.archive.org/web/20031211220920/http://www.rodatazone.com/", "817:events/mr_smile.lua:16"),
    ("Smile Assistant", "Congratulations ! Now please take this Mr.Smile. His majesty,Tristram the 3rd has promised to try his best to make Ragnarok better and more enjoyable. The fact mentioned above was announced by the Public Information Bureau of the Rune-Midgarts Kingom.", "iRO", "2003-12-11", "http://web.archive.org/web/20031211220920/http://www.rodatazone.com/", "817:events/mr_smile.lua:28"),
    ("Smile Assistant", "Oh - unfortunately You have not brought enough items for Mr. Smile. 10 Jellopy 10 Fluff 10 Clover. Please check the requirements above.", "iRO", "2003-12-11", "http://web.archive.org/web/20031211220920/http://www.rodatazone.com/", "817:events/mr_smile.lua:35"),

    # Kafra
    ("Kafra Employee", "Welcome~! The Kafra Services are always on your side. So how can I help you?", "iRO", "2003-12-28", "http://web.archive.org/web/20031228223806/http://iro.ragnarokonline.com/", "scripts/npc/kafra.lua:15"),
    ("Kafra Employee", "Your respawn point has been saved here. Thank you for using the Kafra Services!", "iRO", "2003-12-28", "http://web.archive.org/web/20031228223806/http://iro.ragnarokonline.com/", "scripts/npc/kafra.lua:42"),

    # Blacksmith Refiner Hollgrehenn
    ("Refiner Hollgrehenn", "I'm a blacksmith who can refine weapons and equipment. Do you have any items that you'd like to refine?", "iRO", "2003-12-11", "http://web.archive.org/web/20031211220920/http://www.rodatazone.com/", "817:other/refine.txt:12"),
    ("Refiner Hollgrehenn", "Clang, clang!!!", "iRO", "2003-12-11", "http://web.archive.org/web/20031211220920/http://www.rodatazone.com/", "817:other/refine.txt:45"),
    ("Refiner Hollgrehenn", "WAHHHH!!! I'm so sorry... I warned you this could happen...", "iRO", "2003-12-11", "http://web.archive.org/web/20031211220920/http://www.rodatazone.com/", "817:other/refine.txt:55"),

    # Peco Peco Breeder
    ("Peco Peco Breeder", "Welcome. Honorable Knight, would you like to rent a Peco Peco? The rental fee is 2500 zeny.", "iRO", "2003-12-11", "http://web.archive.org/web/20031211220920/http://www.rodatazone.com/", "817:events/rental.lua:10"),
    ("Peco Peco Breeder", "Here you go. Take good care of it.", "iRO", "2003-12-11", "http://web.archive.org/web/20031211220920/http://www.rodatazone.com/", "817:events/rental.lua:18"),
    ("Peco Peco Breeder", "You do not have enough zeny.", "iRO", "2003-12-11", "http://web.archive.org/web/20031211220920/http://www.rodatazone.com/", "817:events/rental.lua:22"),

    # Juice Quest
    ("Morrison", "Agh.... Noooo!!! No, don't make me eat it! I can't bear to taste fruits!", "iRO", "2003-12-11", "http://web.archive.org/web/20031211220920/http://www.rodatazone.com/", "817:events/making.lua:105"),
    ("Morrison", "Ooh! M-Meaaat~ Ah... hu... hungry... The scent of Meat... Excuse me... C-can I please have one?", "iRO", "2003-12-11", "http://web.archive.org/web/20031211220920/http://www.rodatazone.com/", "817:events/making.lua:112"),
    ("Morrison", "Waah...! Fine, I get it now. Adults are all the same!", "iRO", "2003-12-11", "http://web.archive.org/web/20031211220920/http://www.rodatazone.com/", "817:events/making.lua:118"),
]

for d in dialogue_records:
    rows.append(list(d))

# 2. Bug Reports
bug_reports = [
    ("Bongun Pet Speech (Bug #4201)", "Wahh, too much food!", "iRO", "2004-06-04", "http://web.archive.org/web/20040605112059/http://iro.ragnarokonline.com/community/bugcontent.asp?mNum=2&seqNo=4201", "none"),
    ("Bongun Pet Speech (Bug #4201)", "I think I'm gonna throw up...", "iRO", "2004-06-04", "http://web.archive.org/web/20040605112059/http://iro.ragnarokonline.com/community/bugcontent.asp?mNum=2&seqNo=4201", "none"),
    ("Bongun Pet Speech (Bug #4201)", "For the first time in my life, I'm full. You complete me.", "iRO", "2004-06-04", "http://web.archive.org/web/20040605112059/http://iro.ragnarokonline.com/community/bugcontent.asp?mNum=2&seqNo=4201", "none"),
    ("Bongun Pet Speech (Bug #4201)", "Fattening me up for slaughter, eh? I'm watching you...", "iRO", "2004-06-04", "http://web.archive.org/web/20040605112059/http://iro.ragnarokonline.com/community/bugcontent.asp?mNum=2&seqNo=4201", "none"),
    ("Bongun Pet Speech (Bug #4201)", "Bananas again? With this many, I'll turn yellow and become a Drops?", "iRO", "2004-06-04", "http://web.archive.org/web/20040605112059/http://iro.ragnarokonline.com/community/bugcontent.asp?mNum=2&seqNo=4201", "none"),
    ("Bongun Pet Speech (Bug #4201)", "Yummy yummy in my tummy", "iRO", "2004-06-04", "http://web.archive.org/web/20040605112059/http://iro.ragnarokonline.com/community/bugcontent.asp?mNum=2&seqNo=4201", "none"),
    ("Wizard Job Quest NPC (Bug #4218)", "I am currently Mage job level 50 but the guildsman is still asking me for the items of the first test. I was under the impression at job level 50 we get to skip the first test.", "iRO", "2004-06-05", "http://web.archive.org/web/20040608124530/http://iro.ragnarokonline.com/community/bugContent.asp?mNum=2&seqNo=4218", "817:jobs/wizard.txt:4"),
    ("Assassin Job Quest 'Annonymus' NPC (Bug #4222)", "When you talk to the 'annonymus' guy. After getting asked the first question, just his cancel and walk up to the waiting room. I didn't answer a question and was turned assassin.", "iRO", "2004-06-05", "http://web.archive.org/web/20040608124530/http://iro.ragnarokonline.com/community/bugContent.asp?mNum=2&seqNo=4222", "none"),
]

for b in bug_reports:
    rows.append(list(b))

# 3. Client Tables: Pet Talk Table
pet_xml_data = get_grf_file("data\\pettalktable.xml")
if pet_xml_data:
    root = ET.fromstring(pet_xml_data.decode("euc-kr", errors="replace"))
    for pet in root:
        pet_name = pet.tag.upper()
        for state in pet:
            state_name = state.tag
            for line_elem in state:
                line_type = line_elem.tag
                txt = (line_elem.text or "").strip()
                if txt:
                    txt_clean = " ".join(txt.split())
                    context = f"Pet {pet_name} [{state_name}/{line_type}]"
                    rows.append([
                        context,
                        txt_clean,
                        "kRO/iRO/euRO (client)",
                        "2003-12-18",
                        "data.grf:data\\pettalktable.xml",
                        "none"
                    ])

# 4. Client Tables: msgstringtable.txt
msg_data = get_grf_file("data\\msgstringtable.txt")
if msg_data:
    msg_lines = msg_data.decode("latin1", errors="replace").split("#")
    for i, line in enumerate(msg_lines):
        line = line.strip()
        if len(line) > 15 and not line.startswith("/"):
            context = f"Client UI / Server Msg #{i+1}"
            rows.append([
                context,
                line,
                "kRO/iRO/euRO (client)",
                "2003-12-18",
                f"data.grf:data\\msgstringtable.txt line {i+1}",
                "client-side display"
            ])

print(f"Total TSV rows: {len(rows)}")

headers = ["npc_or_context", "text", "region", "source_date", "source", "ours"]
with open(OUT_TSV, "w", encoding="utf-8", newline="") as f:
    writer = csv.writer(f, delimiter="\t")
    writer.writerow(headers)
    for r in rows:
        writer.writerow(r)

print(f"Successfully written {len(rows)} rows to {OUT_TSV}.")
