-- Turtle Island: the old man in Alberta's inn, the sailor who takes you
-- there, the scholar and the logbook on the quay, the four things the
-- logbook leads to on the island, and the sailor who brings you back.
--
-- Source: mod0631 conf/npc_turtle.txt (archived 2003-12-18; its header says
-- "Turtle Quest! From jRO"). Tier athena2003, jRO lineage. 817 has none of
-- these people: its Alberta sailor sold a passage for 800z to the mouth of
-- the island's cave and nobody brought you back (scripts/npc/
-- alberta_sailor.lua). iRO announced the island for 2003-12-16 "with new
-- quests" (research/notes/round4-review.md), which is all we know of the
-- international servers: research/THREADS.md #24.
--
-- What is whose:
--   * who stands where, the order of the steps, the two variables, the
--     10000z fare, the cells the ship lands on, the rewards (five Bananas,
--     or a Red Gemstone): mod0631;
--   * the words are ours: a short retelling of what each one says, not a
--     translation of mod0631's Japanese and not Gravity's English
--     (research/THREADS.md #2). The names are our reading of its katakana;
--   * one mended line: mod0631's rock tests `if (kame_quest == 2) LStart2;`
--     without the goto, so its Bananas could never be had. Ours jumps.
-- Kept as mod0631 has it: reading the logbook again after an errand starts
-- another, so both rewards can be had as often as one sails.

local BANANA, RED_GEMSTONE = 513, 716
local FARE = 10000

-- ------------------------------------------------------------ Alberta

npc {
  map = "alberta", x = 91, y = 70, dir = 4, sprite = 89, name = "Candy Boy",
  src = "m631:conf/npc_turtle.txt:1",
  on_click = function()
    mes "^3355ff- something bubbles in a ladle -^000000"
    next()
    mes "[Sodaman]"
    mes "Don't talk to me now! Sugar and soda, over the fire, and it must not burn..."
    next()
    mes "[Sodaman]"
    mes "You want a story instead? There is an old man in the inn here who nearly conquered Turtle Island."
    mes "He drinks, but he talks sense. Go and ask him."
    close()
  end,
}

npc {
  map = "alberta_in", x = 23, y = 104, dir = 4, sprite = 120, name = "Old Man",
  src = "m631:conf/npc_turtle.txt:47",
  on_click = function()
    mes "[Turtle Grandpa]"
    mes "Whew... the trading capital of the world, and not one decent tavern in it."
    next()
    local pick = menu("About Turtle Island", "How to get to Turtle Island", "End the conversation")
    if pick == 1 then
      mes "[Turtle Grandpa]"
      mes "More treasure than a lifetime could count, they say, and a medicine that lengthens life. Nobody is even sure the island exists."
      mes "I am."
      next()
      mes "[Turtle Grandpa]"
      mes "For a true man of the sea with an honest heart I might have a word or two."
      next()
      if menu("(look him in the eye)", "Tell me! Tell me!") == 1 then
        mes "[Turtle Grandpa]"
        mes "Good eyes. Listen, then."
        mes "Some ten years ago the adventurer ^3355ffZonadan Niliriya^000000 and ten companions set out to find the island."
        next()
        mes "[Turtle Grandpa]"
        mes "They had only rumours to go by, and gathered them one at a time. Their leader was something else: a kick of his was worth a master's Bash, they say."
        next()
        mes "[Turtle Grandpa]"
        mes "They found it. And something happened there that kept most of them on the island for good. I do not know the rest."
        next()
        mes "[Turtle Grandpa]"
        mes "Ask the scholar at the east end of the harbour. And look near him for the ^3355fflogbook^000000 he always keeps."
      else
        mes "[Turtle Grandpa]"
        mes "I have nothing to teach a greedy soul like that. Off with you!"
      end
    elseif pick == 2 then
      if v.turtle_npc ~= 1 then
        mes "[Turtle Grandpa]"
        mes "Hard to find, harder to reach, and few come back. You still want to go?"
        mes "Fool. It will leave nothing of you but bones."
        next()
      end
      mes "[Turtle Grandpa]"
      mes "Somewhere in the harbour there is an old steamboat man called ^3355ffGotanblue^000000."
      mes "Tell him the old turtle sent you, and he may show you the way. Good luck. Kah kah kah!"
      v.turtle_npc = 1
    else
      mes "[Turtle Grandpa]"
      mes "Hic... one more, and then no more."
    end
    close()
  end,
}

npc {
  map = "alberta", x = 247, y = 122, dir = 4, sprite = 709, name = "Sailor",
  src = "m631:conf/npc_turtle.txt:236",
  on_click = function()
    if v.turtle_npc ~= 1 then
      mes "[Gotanblue]"
      mes "Be good to your friends while you can. You never know when the parting comes."
      next()
      mes "[Gotanblue]"
      mes "And the sea is wider than the piece of it you see from the shore."
      return close()
    end
    mes "[Gotanblue]"
    mes "Hm? That look says you have heard it all. So you met the old turtle. To business, then."
    next()
    local pick = menu("About Turtle Island", "How to get to Turtle Island", "End the conversation")
    if pick == 1 then
      mes "[Gotanblue]"
      mes "The island that took my companions. We sailed from here, more than ten years ago, with ^3355ffZonadan Niliriya^000000 leading and the best navigator he could hire."
      next()
      mes "[Gotanblue]"
      mes "Three years of searching, then a fog so thick we struck rock after rock. When the ship was nearly gone, there it lay."
      next()
      mes "[Gotanblue]"
      mes "We were not the first. A swordsman called ^3355ffWon^000000 had found it alone, long before, and left word of a carved jewel worth more than any other."
      mes "We searched for months and never saw it."
      next()
      mes "[Gotanblue]"
      mes "Leaving was worse. A month in the fog, land at last, and it was the far shore of the same island. Again, and again."
      mes "One by one we lost them. Only the captain and I came home."
      next()
      mes "[Gotanblue]"
      mes "That is all I know. The scholar by the harbour still studies the place: ask him the rest."
    elseif pick == 2 then
      mes "[Gotanblue]"
      mes("After a tale like that you still want to go? Hmph. Then my fee as a guide is " .. FARE .. " Zeny.")
      next()
      if menu("Turtle Island -> " .. FARE .. " z", "Never mind") ~= 1 then
        mes "[Gotanblue]"
        mes "Wise. It is a cursed island, and I am spared the feeling of rowing a man to the gallows."
        return close()
      end
      if player.zeny < FARE then
        mes "[Gotanblue]"
        mes("It is not cheap, but cheaper than your life. You do not have " .. FARE .. " Zeny.")
        return close()
      end
      mes "[Gotanblue]"
      mes "Your mind is made up, I see. Then we sail!"
      next()
      mes "^3355ff- you board Gotanblue's steamboat -"
      mes "- and it heads for Turtle Island -^000000"
      next()
      -- the island's maps come with a patch the client may not have merged
      if not pcall(warp, "tur_dun01", 157, 39) then
        mes "[Gotanblue]"
        mes "The fog is too thick today. Keep your money."
        return close()
      end
      player.zeny = player.zeny - FARE
      return
    else
      mes "[Gotanblue]"
      mes "Hard to find, harder to reach, and few come back alive. The wise do not set out at all."
    end
    close()
  end,
}

npc {
  map = "alberta", x = 248, y = 89, dir = 4, sprite = 121, name = "Turtle Island Scholar",
  src = "m631:conf/npc_turtle.txt:520",
  on_click = function()
    mes "[Zonadan Niliriya]"
    mes "It lies under that same sky, and yet you cannot measure the way there by eye."
    next()
    local pick = menu("About Turtle Island", "Aldebaran and Alberta sound alike", "End the conversation")
    if pick == 1 then
      mes "[Zonadan Niliriya]"
      mes "Why the name? Nothing grand: from the sea the island looks like a turtle putting out its head."
      next()
      mes "[Zonadan Niliriya]"
      mes "And the fog is spray, from a great waterfall inside its caves. Truth is that plain; we dig for it so as not to fear what we cannot name."
      next()
      mes "[Zonadan Niliriya]"
      mes "I feared the island once. Now only one thing there is unexplained, a carved jewel, and until it is I shall go on studying."
    elseif pick == 2 then
      mes "[Zonadan Niliriya]"
      mes "Hah...! Now that you say it... Hahahaha!"
    else
      mes "[Zonadan Niliriya]"
      mes "Work at the truth and one day the answer comes."
      next()
      mes "[Zonadan Niliriya]"
      mes "That reminds me, I am hungry. Why does nobody in Alberta sell meat buns?"
    end
    close()
  end,
}

npc {
  map = "alberta", x = 248, y = 94, dir = 4, sprite = 111, name = "Logbook",
  src = "m631:conf/npc_turtle.txt:597",
  on_click = function()
    if v.kame_quest ~= 1 then
      mes "[Logbook]"
      mes "= 03:20 ="
      mes "No moon, and nothing to be seen a hundred paces off. The men cannot sleep."
      next()
      mes "[Logbook]"
      mes "= 04:10 ="
      mes "A blow from nowhere brought down a mast and killed the cook. The hull is holed."
      next()
      mes "[Logbook]"
      mes "= 04:45 ="
      mes "The boatswain and two hands, mending the hull, were killed by what came in through the hole. A third of the food is spoiled."
      next()
      mes "[Logbook]"
      mes "= 05:23 ="
      mes "More rocks, less water between them. Shall we reach land?"
      mes "- Captain Zonadan Niliriya -"
      v.kame_quest = 1
      return close()
    end
    mes "^3355ff- a logbook, torn in places, -"
    mes "- stained with weed and mould -^000000"
    next()
    if rand(0, 1) == 0 then
      mes "^3355ff- the page where a banana leaf -"
      mes "- has been pressed -^000000"
      next()
      mes "[Logbook]"
      mes "On the island we found a long yellow fruit among broad leaves. It may not be a banana, but it tastes like one, and we stored all we could."
      next()
      mes "[Logbook]"
      mes "Then the sickness: the biggest eaters first, then a man dead. The island's animals will not touch the fruit."
      next()
      mes "[Logbook]"
      mes "We have sealed it away. Peeled, it does not rot. It may yet be a medicine; we will take it home."
      next()
      mes "[Logbook]"
      mes "^ff5533- tur_dun01 -"
      mes "- X : 160 . Y : 81 -^000000"
      mes "^3355ff- a thin key with a small skull on it is tucked in here -"
      mes "- you remember the Skull Key -^000000"
      v.kame_quest = 2
    else
      mes "^3355ff- the page marked with a bird's -"
      mes "- footprint in dark red juice -^000000"
      next()
      mes "[Logbook]"
      mes "Someone lived on this island before us. We looked for what he left, day after day, and found nothing certain."
      next()
      mes "[Logbook]"
      mes "Now we know why: he was one man, a swordsman called Won, and he wrote on hides. The island has four levels, he says, full of traps, and some power at work."
      next()
      mes "[Logbook]"
      mes "We hid his record on the second level."
      mes "^3355ff- tur_dun02 -"
      mes "- X : 132 . Y : 251 -"
      mes "- under the tree root -^000000"
      next()
      mes "^3355ff- a thin key with a root drawn on it is tucked in here -"
      mes "- you remember the Root Key -^000000"
      v.kame_quest = 3
    end
    close()
  end,
}

-- ------------------------------------------------------------ the island

npc {
  map = "tur_dun01", x = 160, y = 81, dir = 0, sprite = 111, name = "Skull Grave Rock",
  src = "m631:conf/npc_turtle.txt:870",
  on_click = function()
    if v.kame_quest ~= 2 then
      mes "^3355ff- a grave rock with a skull set on it; -"
      mes "- the sight makes you shiver -^000000"
      return close()
    end
    mes "^3355ff- low on the rock is a keyhole -"
    mes "- with a small skull drawn by it -"
    mes "- you turn the Skull Key in it -^000000"
    next()
    mes "^3355ff- click! -"
    mes "- the top of the rock opens -^000000"
    give_item(BANANA, 5)
    v.kame_quest = 6
    next()
    mes "^3355ff- you take five Bananas -^000000"
    close()
  end,
}

npc {
  map = "tur_dun02", x = 132, y = 251, dir = 0, sprite = 111, name = "Tree Root",
  src = "m631:conf/npc_turtle.txt:895",
  on_click = function()
    if v.kame_quest ~= 3 then
      mes "^3355ff- thick old roots, twisted -"
      mes "- together, deep in the ground -^000000"
      return close()
    end
    mes "^3355ff- under the roots is a keyhole -"
    mes "- you turn the Root Key in it -^000000"
    next()
    mes "^3355ff- the roots part like a door -"
    mes "- and a scroll falls out -^000000"
    next()
    mes "[Scroll]"
    mes "^ff0000I came here to master the sword. Where I was born they say: who cuts the turtle's shell holds the world.^000000"
    mes "^ff0000- Won -^000000"
    next()
    mes "[Scroll]"
    mes "^ff0000= go on =^000000"
    mes "^ff0000tur_dun02  X : 45 . Y : 125^000000"
    mes "^ff0000the Turtle Crystal Mound^000000"
    next()
    mes "^3355ff- a key with a crystal drawn on it -"
    mes "- is rolled into the scroll -"
    mes "- you take the Crystal Key -^000000"
    v.kame_quest = 4
    close()
  end,
}

npc {
  map = "tur_dun02", x = 46, y = 125, dir = 0, sprite = 111, name = "Turtle Crystal Mound",
  src = "m631:conf/npc_turtle.txt:950",
  on_click = function()
    if v.kame_quest ~= 4 then
      mes "^3355ff- a heap of the crystals that -"
      mes "- are found only on this island -^000000"
      return close()
    end
    mes "^3355ff- in the heap is a keyhole -"
    mes "- shaped like a crystal -"
    mes "- you turn the Crystal Key in it -^000000"
    next()
    mes "^3355ff- a gap opens, and in it -"
    mes "- lies another scroll -^000000"
    next()
    mes "[Scroll]"
    mes "^5533ffI can split the trees here in two. Next, the crystal: its grain runs every way, and only a true cut will mark it. I shall make that cut.^000000"
    next()
    mes "[Scroll]"
    mes "^5533ffFive paces east, thirty south, five more: the Turtle Pillar. The last of the art sleeps there.^000000"
    next()
    mes "^3355ff- a long key hangs from the scroll -"
    mes "- you take the Pillar Key -^000000"
    v.kame_quest = 5
    close()
  end,
}

npc {
  map = "tur_dun02", x = 49, y = 156, dir = 0, sprite = 111, name = "Turtle Pillar",
  src = "m631:conf/npc_turtle.txt:1004",
  on_click = function()
    if v.kame_quest ~= 5 then
      mes "^3355ff- one of the many pillars -"
      mes "- that hold the island up -^000000"
      return close()
    end
    mes "^3355ff- low on the pillar is a keyhole -"
    mes "- with a turtle carved by it -"
    mes "- you turn the Pillar Key in it -^000000"
    next()
    mes "^3355ff- a door grinds open and -"
    mes "- a written stone rolls out -^000000"
    next()
    mes "[Turtle Stone]"
    mes "^00ff00No sword cuts this pillar, and if one did the cave would fall. The art is not in cutting what can be cut. It is in seeing what should be.^000000"
    next()
    mes "[Turtle Stone]"
    mes "^00ff00For all I gave up to learn it, I leave this here.^000000"
    mes "^00ff00- Won -^000000"
    give_item(RED_GEMSTONE, 1)
    v.kame_quest = 6
    next()
    mes "^3355ff- you take a Red Gemstone -^000000"
    close()
  end,
}

npc {
  map = "tur_dun01", x = 165, y = 29, dir = 4, sprite = 709, name = "Sailor",
  src = "m631:conf/npc_turtle.txt:1064",
  on_click = function()
    mes "[Gotanblue]"
    mes "Back to Alberta?"
    next()
    if menu("Return to Alberta", "End the conversation") ~= 1 then return close() end
    mes "^3355ff- you board Gotanblue's steamboat -"
    mes "- and it heads for Alberta -^000000"
    next()
    warp("alberta", 241, 115)
  end,
}
