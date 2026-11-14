-- Dyes: the rumour in Alberta, the chemist in Geffen, the dye maker in
-- Morroc, and the two dyers in Prontera.
--
-- Source: mod0631 conf/npc_event_dye.txt (archived 2003-12-18; loaded by its
-- map_athena.conf:52). Tier athena2003, jRO lineage. 817 has four of these
-- people as passers-by with one or two lines each (cities/alberta.txt:322,
-- geffen.txt:190 and :248, prontera.txt:55) and no dye maker at all, though
-- its own Merchant quest sends a parcel to "JavaDullihan, the Dyermaker in
-- Morroc": nothing in 817 makes a dye, and its hat quests ask for them.
--
-- What is whose:
--   * steps, places, recipes and prices: mod0631;
--   * the English lines 817 has for the same NPCs (it translates the same
--     script) are 817's, typos mended; the rest is our translation of
--     mod0631's Japanese, shortened to fit the dialogue window. None of it is
--     Gravity's English (research/THREADS.md #2);
--   * names: Louitz, Aure Dupon, Morgenshtein, Dairenne and JavaDullihan are
--     817's; "Jovovich" and "Ginedin Rephere" are our reading of mod0631's
--     katakana;
--   * hair dyeing itself is ours: mod0631's hairdresser lists the terms (one
--     dye of the colour you want, 5000z) and then says "not implemented".
--     Which colour number is which is the client's: its nine hair palettes
--     per hairstyle are red, yellow, violet, orange, green, blue, white,
--     black, red again (data\palette, all 38 sets; tier client);
--   * clothes are not dyed: mod0631's dyer is "too busy", and so is ours
--     (research/THREADS.md #13).
-- docs/DEVIATIONS.md lists the two places where we leave mod0631.

local ALCOHOL, DETRIMINDEXTA, KARVODAILNIROL, COUNTERAGENT, MIXTURE, EMPTY_BOTTLE = 970, 971, 972, 973, 974, 713
local RED_HERB, YELLOW_HERB, WHITE_HERB, BLUE_HERB, GREEN_HERB = 507, 508, 509, 510, 511

local function has(list)
  for _, it in ipairs(list) do
    if count_item(it[1]) < it[2] then return false end
  end
  return true
end

local function take(list)
  for _, it in ipairs(list) do take_item(it[1], it[2]) end
end

-- ------------------------------------------------------------ Alberta: Louitz

npc {
  map = "alberta_in", x = 133, y = 55, dir = 3, sprite = 84, name = "Louitz",
  replaces = "817:cities/alberta.txt:322", src = "m631:conf/npc_event_dye.txt:5",
  on_click = function()
    if v.MOLGENSTAIN == 2 then
      mes "[Merchant Louitz]"
      mes "Oh... it's you."
      mes "Well? Have you been to see that mad scientist in Geffen?"
      next()
      if menu("Talk about Morgenshtein", "Cancel") == 1 then
        mes "[Merchant Louitz]"
        mes "You asked around Geffen where he lives and still haven't gone?"
        mes "Hmm... what are you waiting for? He may make something amazing out of those solutions..."
      else
        mes "[Merchant Louitz]"
        mes "If you have something to say, say it. Well, if you don't feel like it, it can't be helped..."
      end
      return close()
    end
    mes "[Merchant Louitz]"
    mes "What's the matter?"
    next()
    local det, kar = count_item(DETRIMINDEXTA) > 0, count_item(KARVODAILNIROL) > 0
    local what = (det and kar) and "the solutions" or kar and "Karvodailnirol" or det and "Detrimindexta" or nil
    local pick
    if what then
      pick = menu("Talk", "Talk about " .. what, "Cancel")
    else
      pick = menu("Talk", "Cancel")
      if pick == 2 then pick = 3 end
    end
    if pick == 1 then
      mes "[Merchant Louitz]"
      mes "I'd stayed in Geffen to find Magic Tools for a while. In Geffen, I heard the Rumor about the Mad Scientist."
      mes "With Great Interest, I tried to meet him several times and finally I did. But it was no use... he was too immersed in his work to take a look at me at all."
      next()
      mes "[Merchant Louitz]"
      mes "I still remember what I heard him murmur at that moment."
      mes "'Karvodailnirol... Detrimindexta...'"
      mes "At first I thought it just useless murmur, but now I realise those things are new Solutions, totally different from Water."
    elseif pick == 2 then
      -- mod0631 sets 1 here whatever it was, which would send someone the
      -- chemist already knows round by Geffen's gossip again
      if v.MOLGENSTAIN < 1 then v.MOLGENSTAIN = 1 end
      mes "[Merchant Louitz]"
      mes("Oh... so this is " .. (what == "the solutions" and "that solution" or what) .. "!")
      mes "I had only heard of it and never seen it. How did you get hold of such a rare thing?"
      mes "But what to do with it, I'm sorry to say, I don't know. Only he would know..."
      next()
      mes "[Merchant Louitz]"
      mes "For showing me something so rare, I'll tell you where to find the chemist."
      mes "It was long ago and I don't remember the exact place, but he lives in Geffen."
      mes "His name was ^0000ffMorgenshtein^000000."
      next()
      mes "[Merchant Louitz]"
      mes "Ask around Geffen and you will find out more."
      mes "Good luck."
    else
      mes "[Louitz]"
      mes "Um... Now I've seen everything."
    end
    close()
  end,
}

-- ------------------------------------------------------------ Geffen: Aure Dupon

npc {
  -- 817's cell; mod0631 has him three cells east (184,114)
  map = "geffen", x = 181, y = 114, dir = 4, sprite = 82, name = "Aure Dupon",
  replaces = "817:cities/geffen.txt:248", src = "m631:conf/npc_event_dye.txt:92",
  on_click = function()
    local where = function()
      mes "He lives upstairs from the smithy."
      mes "One thing, briefly, because it matters: be careful! Better not to get too close to him..."
    end
    local nothing = function()
      mes "[Aure Dupon]"
      mes "I can see your strong will on your look. But you won't tell me anything."
      mes "I feel sorry but I can't help it..."
      mes "Human being is one part of Nature... The Human will is the same as ... Mother Nature."
    end
    local strange = function()
      mes "[Aure Dupon]"
      mes "I don't think I am a normal person because I am bewitched by power of magic. However beside me it seems so many strange people are out there ..."
      mes "If I were you, would you stake your life on one thing? Neglecting everything more important in your life... as if you'd gone insane..."
    end
    if v.MOLGENSTAIN == 2 then
      mes "[Aure Dupon]"
      mes "Fate has led us together once more. I was waiting for you."
      mes "So... have you met the man you are looking for?"
      next()
      if menu("Talk about Morgenshtein", "Cancel") == 1 then
        mes "[Aure Dupon]"
        mes "You have not met him yet."
        where()
      else
        mes "[Aure Dupon]"
        mes "I can see your strong will on your look. But you won't tell me anything."
        mes "I thought we were friends by now... was that only my own illusion?"
      end
      return close()
    end
    mes "[Aure Dupon]"
    mes "Time never awaits you. Even Magic can't make it slow. Now I can feel it still passing by while I am talking."
    mes "Anyway, why did you call me to stop?"
    next()
    if v.MOLGENSTAIN <= 0 then
      if menu("Talk", "Cancel") == 1 then strange() else nothing() end
      return close()
    end
    local pick = menu("Talk", "Ask about Morgenshtein", "Cancel")
    if pick == 1 then
      strange()
    elseif pick == 2 then
      v.MOLGENSTAIN = 2
      mes "[Aure Dupon]"
      mes "If you know of that madman, you can't be far from mad yourself. You seem to want to know a great deal about him..."
      mes "To explain him is beyond me. I'm sorry... The most I can do is tell you where he lives."
      next()
      mes "[Aure Dupon]"
      where()
    else
      nothing()
    end
    close()
  end,
}

-- ------------------------------------------------------------ Geffen: Morgenshtein

-- { product, fee, ingredients }
local SOLVENTS = {
  { COUNTERAGENT, 3000, "a Counteragent", "Alcohol.... and..... Detrimindexta....",
    { { ALCOHOL, 1 }, { DETRIMINDEXTA, 1 }, { EMPTY_BOTTLE, 1 } } },
  { MIXTURE, 4000, "a Mixture", "Alcohol.... and..... Karvodailnirol....",
    { { ALCOHOL, 1 }, { KARVODAILNIROL, 1 }, { EMPTY_BOTTLE, 1 } } },
}

npc {
  map = "geffen_in", x = 141, y = 140, dir = 3, sprite = 121, name = "Morgenshtein",
  replaces = "817:cities/geffen.txt:190", src = "m631:conf/npc_event_dye.txt:146",
  on_click = function()
    local who = "[Chemist Morgenshtein]"
    local research = function()
      mes(who)
      mes "Heheheh... so you have heard of my research somewhere..."
      mes "Well, you know it all anyway, so I won't hide it... no... there is nothing to hide... Heheheheh..."
      next()
      mes(who)
      mes "In truth... my research is as good as finished...."
      mes "Nothing is beyond this genius brain of mine... Heheh.... it is only a matter of time... time.... Hehehe...."
      next()
      mes(who)
      mes "It is a very simple thing... a way to dissolve what water will not dissolve."
      mes "That hard rock, that lump of iron, melted into a feeble liquid!!!"
      mes "Everything turned to slime~~~ Hehe!!!...Heheheh!!...slime!!!"
      next()
      mes(who)
      mes "Hah... hah.... a...anyway, I can't dissolve everything yet, but I have found out a few things."
      mes "With them far more new things can be made than you can imagine... if only you wish!!"
    end
    if v.MOLGENSTAIN == 3 then
      mes(who)
      mes "Heheheheh.... welcome. Heheheh.... Hehehehehe...."
      next()
      local pick = menu("Ask about his research", "Ask him to make something", "Cancel")
      if pick == 1 then
        research()
        return close()
      elseif pick ~= 2 then
        mes(who)
        mes "Hehe....heheheh...."
        return close()
      end
      mes(who)
      mes "Hehehehe.... so you are interested after all."
      mes "Yes... yes... who in the world could fail to be..."
      mes "Now... what do you want made??.. Hm? Heheheh"
      next()
      local s = SOLVENTS[menu("Counteragent", "Mixture")]
      if not s then return close() end
      mes(who)
      mes("Oh... " .. s[3] .. ", you say... Hu hu hu hu... then I shall tell you what this game needs.")
      mes "... let me see... oh... oh...... ahh....."
      next()
      mes(who)
      mes("Oh..... I remember... yes... " .. s[4] .. " and an Empty Bottle to keep it in...")
      mes "And for the pleasure of this game of genius, a small token of thanks... Heheheh"
      mes("Listen well... " .. s[2] .. " Zeny will do...heheheh")
      next()
      mes(who)
      mes "Now... shall we begin?...."
      next()
      if menu("Make it", "Cancel") ~= 1 then
        mes(who)
        mes "Hu... huhuhuhu...."
        return close()
      end
      if not has(s[5]) or player.zeny < s[2] then
        mes(who)
        mes "Huhuhu....not enough...hah..hah....quick.. quick...go and gather the materials!!"
        mes "The game waits till then... Huhuhuhu"
        return close()
      end
      mes(who)
      mes "Ooooh...... Hehehehehe...... yes... like this...... yes... go on... more........"
      mes ".... Ahh.... aha... ha.... oh.... oh!..... OOOOH!!!...!!!!!!"
      next()
      take(s[5])
      player.zeny = player.zeny - s[2]
      give_item(s[1], 1)
      mes(who)
      mes ".... hah......"
      return close()
    end
    if v.MOLGENSTAIN == 2 then
      v.MOLGENSTAIN = 3
      mes(who)
      mes "Heheheheh.... welcome. Heheheh.... Hehehehehe...."
      next()
      mes(who)
      mes "Ah... forgive me if I was rude... Heheheh... it is so long... since I saw a living person."
      mes "So... what do you want?"
      next()
      local pick = menu("Ask about his research", "Ask him to make something", "Cancel")
      if pick == 1 then
        research()
      elseif pick == 2 then
        mes(who)
        mes "Hehe....heheheh...."
      end
      return close()
    end
    mes(who)
    mes "Heheheheheh... Sniff Sniff... I can smell something appetizing here."
    mes "It is a Human Being... a living one... umm yummy... Heheheheheh"
    close()
  end,
}

-- ------------------------------------------------------------ Morroc: the dye maker

-- { dye, fee, colour name, what he says it needs, ingredients }
local DYES = {
  { 975, 3000, "Scarlet", "30 Red Herbs, a Counteragent and an Empty Bottle",
    { { RED_HERB, 30 }, { COUNTERAGENT, 1 }, { EMPTY_BOTTLE, 1 } } },
  { 976, 3000, "Lemon", "30 Yellow Herbs, a Counteragent and an Empty Bottle",
    { { YELLOW_HERB, 30 }, { COUNTERAGENT, 1 }, { EMPTY_BOTTLE, 1 } } },
  { 978, 3500, "Cobalt Blue", "20 Blue Herbs, a Counteragent and an Empty Bottle",
    { { BLUE_HERB, 20 }, { COUNTERAGENT, 1 }, { EMPTY_BOTTLE, 1 } } },
  { 979, 5000, "Dark Green", "5 Blue Herbs, 20 Green Herbs, 20 Yellow Herbs, a Counteragent, a Mixture and an Empty Bottle",
    { { BLUE_HERB, 5 }, { COUNTERAGENT, 1 }, { EMPTY_BOTTLE, 1 }, { GREEN_HERB, 20 }, { YELLOW_HERB, 20 }, { MIXTURE, 1 } } },
  { 980, 5000, "Orange", "20 Red Herbs, 20 Yellow Herbs, a Counteragent, a Mixture and an Empty Bottle",
    { { RED_HERB, 20 }, { COUNTERAGENT, 1 }, { EMPTY_BOTTLE, 1 }, { MIXTURE, 1 }, { YELLOW_HERB, 20 } } },
  -- 30 Red Herbs: mod0631's own line (npc_event_dye.txt:519), Ragu Memo's jRO list (2004-02)
  -- and RODataZone's iRO one (2004-05); mod0631's code tests and takes 20 (:524, :536)
  { 981, 5000, "Violet", "30 Red Herbs, 10 Blue Herbs, a Counteragent, a Mixture and an Empty Bottle",
    { { RED_HERB, 30 }, { COUNTERAGENT, 1 }, { EMPTY_BOTTLE, 1 }, { MIXTURE, 1 }, { BLUE_HERB, 10 } } },
  { 982, 3000, "White", "30 White Herbs, a Counteragent and an Empty Bottle",
    { { WHITE_HERB, 30 }, { COUNTERAGENT, 1 }, { EMPTY_BOTTLE, 1 } } },
  { 983, 7000, "Black", "30 Red Herbs, 30 Yellow Herbs, 30 Green Herbs, 5 Blue Herbs, a Counteragent, a Mixture and an Empty Bottle",
    { { RED_HERB, 30 }, { COUNTERAGENT, 1 }, { EMPTY_BOTTLE, 1 }, { BLUE_HERB, 5 }, { YELLOW_HERB, 30 }, { MIXTURE, 1 }, { GREEN_HERB, 30 } } },
}

npc {
  map = "morocc_in", x = 146, y = 99, dir = 3, sprite = 58, name = "JavaDullihan",
  src = "m631:conf/npc_event_dye.txt:329",
  on_click = function()
    local who = "[Dyemaker JavaDullihan]"
    mes(who)
    mes "Ah... what a hot day. Just the weather for making dyes."
    next()
    local pick = menu("Talk", "Make a dye", "Cancel")
    if pick == 1 then
      mes(who)
      mes "I have nothing in particular to tell you..."
      mes "But I can tell you about my past."
      mes "It may be long and dull. Will you hear it?"
      next()
      if menu("Listen", "Don't listen") ~= 1 then
        mes(who)
        mes "Ahaha.. oh well.. who would sit and listen to a dye maker like me. Hahahaha...."
        return close()
      end
      mes(who)
      mes "Since before I can remember... my father made dyes."
      mes "For longer than I can count he went on making dyes."
      mes "Yes... even when my mother lay dying of illness... he did not stop."
      mes "I hated him for it then..."
      next()
      mes(who)
      mes "So I came to hate dye making too, left home young and tried my hand at everything."
      mes "But... here I am, back home, carrying on the family trade. Blood will out, it seems... it is 15 years now."
      next()
      mes(who)
      mes "What I feel these days is... that I understand my father now."
      mes "Why he gave up everything and lost himself in making dyes..."
      next()
      mes(who)
      mes "It must have been for that colour, built up over work repeated again and again... a colour seen in dreams..."
      next()
      mes(who)
      mes "How do I know?... Because I believe it myself now. Since the moment, some years ago, when I began to hear what the colours say......"
      mes "You'll see. I will bring out the colour my father never could."
      return close()
    elseif pick ~= 2 then
      mes(who)
      mes "I don't mean to boast, but I have the skill of making dyestuffs."
      mes "If you ever need some, ask me. I'll make it at a good price."
      return close()
    end
    mes(who)
    mes "Good! If you like, I'll make one. I promise to bring out the colour you want."
    mes "Which colour shall it be? The fee depends on the colour: some are easy to make and some are not."
    mes "And don't think the fee high. My soul goes into these colours."
    next()
    local list = {}
    for i, d in ipairs(DYES) do list[i] = d[3] .. " Dyestuffs" end
    list[#list + 1] = "Cancel"
    local d = DYES[menu(list)]
    local changed = function()
      mes(who)
      mes "What?"
      mes "You've changed your mind now? Well..."
      close()
    end
    if not d then return changed() end
    mes(who)
    mes("Hmm... for " .. d[3] .. " Dyestuffs I need " .. d[4] .. ".")
    mes("For the making I take " .. d[2] .. " Zeny.")
    mes "Well! I expect you have it all ready... shall I start?"
    next()
    if menu("Make the dye", "Cancel") ~= 1 then return changed() end
    if not has(d[5]) or player.zeny < d[2] then
      mes(who)
      mes "Hmm, not enough..."
      mes "With that I can't bring out a colour worth having. How about gathering some more?"
      return close()
    end
    take(d[5])
    player.zeny = player.zeny - d[2]
    give_item(d[1], 1)
    mes(who)
    mes "Hmm... well, that came out quite well. A very deep colour."
    mes "Of course I will keep striving for one more bewitching still..."
    mes "Well... come back whenever you need a dye."
    close()
  end,
}

-- ------------------------------------------------------------ Prontera: the dyers

npc {
  map = "prt_in", x = 284, y = 168, dir = 3, sprite = 55, name = "Ginedin Rephere",
  src = "m631:conf/npc_event_dye.txt:621",
  on_click = function()
    local who = "[Dyer Ginedin Rephere]"
    mes(who)
    mes "Eleven... twelve... hmm... good. I should just make it in time for tonight's party."
    mes "Ah! ... a customer. Now, what can I do for you?"
    next()
    local pick = menu("Talk", "Dye clothes", "Cancel")
    if pick == 1 then
      mes(who)
      mes "The world seems a little easier these days... to judge by the orders for dresses pouring in, young ladies of some means have fallen on very good times."
      mes "... not that I call it extravagance... People like that are why the beauty we pursue can advance."
      next()
      mes(who)
      mes "We have neither fur nor scales. Is it not through clothes that a person shows what is their own?"
      mes "Seen that way, my work seems to me a noble one, and well worth doing."
      next()
      mes(who)
      mes "It is hard and exacting work, but the joy of the finished dress repays all of it."
    elseif pick == 2 then
      mes(who)
      mes "Ah... you have come to have your clothes dyed too."
      mes "I am sorry, I have no time to spare for yours now. It is all I can do to keep up with the orders already waiting..."
      mes "Would you be so kind as to call again another time?"
    else
      mes(who)
      mes "Please make yourself at home. Though I am too busy to look after you."
    end
    close()
  end,
}

-- The client's hair palettes: 0 and 8 red, 1 yellow, 2 violet, 3 orange,
-- 4 green, 5 blue, 6 white, 7 black. { dye, palette, menu name }
local HAIR = {
  { 975, 8, "Red" }, { 976, 1, "Yellow" }, { 981, 2, "Violet" }, { 980, 3, "Orange" },
  { 979, 4, "Green" }, { 978, 5, "Blue" }, { 982, 6, "White" }, { 983, 7, "Black" },
}

npc {
  map = "prt_in", x = 243, y = 168, dir = 4, sprite = 91, name = "Jovovich",
  src = "m631:conf/npc_event_dye.txt:659",
  on_click = function()
    local who = "[Hairdresser Jovovich]"
    mes(who)
    if player.sex == 1 then
      mes "Welcome."
      mes "Oh dear... what damaged hair..."
      mes "It needs proper care."
      mes "Do sit down."
    elseif rand(2) == 0 then
      mes "Welcome~."
      mes "Oh! Miss..."
      mes "Your hair is quite damaged~."
      mes "Leave it to me."
    else
      mes "Oh! Miss,"
      mes "how about having your hair dyed?"
      mes "You could be prettier still!"
      mes "Won't you make a bold change?"
    end
    next()
    local pick = menu("Dye my hair", "About hair dyeing", "Cancel")
    if pick == 2 then
      mes(who)
      mes "When your spirits are low,"
      mes "when a sweetheart has left you,"
      mes "when there is someone to impress,"
      mes "when you feel like dressing up:"
      mes "why not change your look and feel new?"
      next()
      mes(who)
      mes "Bring me the dye and I will make lovely hair lovelier."
      mes "Never mind the price. What you gain is worth more."
      next()
      mes(who)
      mes "* Materials and price *"
      mes "- ^FF30801 dye of the colour you want^000000"
      mes "- ^FF3080Dyeing fee 5000z^000000"
      return close()
    elseif pick ~= 1 then
      mes(who)
      mes "Man or woman... everyone has the right and the duty to be beautiful."
      return close()
    end
    mes(who)
    mes "Well, that took courage~."
    mes "Now then! Let's be bold and transform you!"
    mes "Which colour shall it be?"
    next()
    local list = {}
    for i, h in ipairs(HAIR) do list[i] = h[3] end
    list[#list + 1] = "Cancel"
    local h = HAIR[menu(list)]
    mes(who)
    if not h then
      mes "Man or woman... everyone has the right and the duty to be beautiful."
    elseif count_item(h[1]) < 1 or player.zeny < 5000 then
      mes "For that I need ^FF30801 dye of the colour^000000 and the ^FF30805000z^000000 fee."
    elseif player.hair_color == h[2] or (h[2] == 8 and player.hair_color == 0) then
      mes "But your hair is that colour already!"
    else
      take_item(h[1], 1)
      player.zeny = player.zeny - 5000
      set_look(6, h[2])
      mes "There~. It suits you beautifully!"
    end
    close()
  end,
}
