-- The three doomed swords: rumours in four towns, and the smith who hides
-- east of Payon and makes Executioner, Grimtooth and Mysteltain (item names
-- as this client shows them).
--
-- Source: mod0631 conf/npc_event_ma_sword.txt (archived 2003-12-18; loaded
-- by its map_athena.conf). Tier athena2003, jRO lineage. 817 has three of
-- the rumour-tellers as passers-by with the same lines in English
-- (cities/izlude.txt:388, morroc.txt:373, prontera.txt:104) and neither the
-- man in Prontera's tavern nor the smith, so in 817 nothing came of the
-- rumour. The ingredients are monster drops in mod0631's own database.
--
-- What is whose:
--   * steps, places and recipes: mod0631;
--   * the English of Nain, Cetsu and Veeyop is 817's (it translates the same
--     script), typos mended and pages cut to the dialogue window; Gatu and
--     the smith are our retelling of mod0631's Japanese, much shortened
--     (the smith tells the whole of Baldr's death). None of it is Gravity's
--     English (research/THREADS.md #2);
--   * names: Nain, Cetsu, Veeyop are 817's; "Gatu" is our reading of the
--     katakana; the smith has no name ("gloomy man");
--   * Nain stands on 817's cell (izlude_in 173,88); mod0631 has her at
--     160,129. The others are on the same cell in both.
-- Where we leave mod0631 (docs/DEVIATIONS.md):
--   * a rumour heard twice does not set the quest back (mod0631 writes 1
--     over whatever was there, so hearing of a second sword sent the player
--     back to Gatu, and a recipe already given was forgotten);
--   * the smith takes what he asks for. mod0631 asks for five Cursed Rubies
--     and ten Blades Lost in Darkness, tests for one ruby and ten blades and
--     takes one of each; and for Mysteltain it takes a hundred Young Twigs
--     where it asked for and tested one;
--   * the smith's list shows the swords the player has heard of. mod0631's
--     chain of tests shows all three to anyone who has heard of the first
--     two.

local EMPERIUM = 714
local SWORDS = {
  {
    var = "ex_sword", label = "Executioner", item = 1169,
    needs = { { 7017, 2 }, { 1008, 3 }, { 7024, 10 }, { 609, 50 }, { EMPERIUM, 1 } },
  },
  {
    var = "orga_sword", label = "Grimtooth", item = 1237,
    needs = { { 724, 5 }, { 7023, 10 }, { 7002, 100 }, { 7022, 1 }, { EMPERIUM, 1 } },
  },
  {
    var = "mys_sword", label = "Mysteltain", item = 1138,
    needs = { { 7018, 1 }, { 7019, 1 }, { 7020, 1 }, { 7021, 1 }, { EMPERIUM, 1 } },
  },
}

-- a rumour-teller: the first hearing starts the sword's thread and, if
-- nothing has started yet, the search for the smith
local function heard(var)
  if v[var] < 1 then v[var] = 1 end
  if v.Ma_sword < 1 then v.Ma_sword = 1 end
end

-- ------------------------------------------------------------ the rumours

npc {
  map = "izlude_in", x = 173, y = 88, dir = 2, sprite = 102, name = "Nain",
  replaces = "817:cities/izlude.txt:388", src = "m631:conf/npc_event_ma_sword.txt:359",
  on_click = function()
    if v.ex_sword >= 1 then
      mes "[Nain]"
      mes "No matter how powerful a sword it is, isn't it spooky to think of the ghosts the sword possesses within..."
      next()
      mes "[Nain]"
      mes "For me, I would throw it away into the garbage can without a regret."
      return close()
    end
    mes "[Nain]"
    mes "Long ago, when the worst criminals were still put to death, there was a sword made for it."
    mes "The doomed sword they call ^0000FFExecutioner^000000."
    next()
    mes "[Nain]"
    mes "The grudges of the condemned gathered in it and tormented whoever held it."
    next()
    mes "[Nain]"
    mes "In the end the executioner himself could bear it no longer and had someone destroy it..."
    next()
    mes "[Nain]"
    mes "The blacksmith who broke it is said to have lived somewhere in Prontera."
    mes "They say he has vanished since."
    heard("ex_sword")
    close()
  end,
}

npc {
  map = "morocc", x = 281, y = 178, dir = 8, sprite = 97, name = "Cetsu",
  replaces = "817:cities/morroc.txt:373", src = "m631:conf/npc_event_ma_sword.txt:387",
  on_click = function()
    if v.orga_sword >= 1 then
      mes "[Cetsu]"
      mes "I wish I knew the secret to forging the Grimtooth... well.. Whatever, I'm not a blacksmith."
      return close()
    end
    mes "[Cetsu]"
    mes "People talk of a sword made out of ogre tooth, which is rumored to be much stronger than steel."
    mes "This sword has come to be known as ^0000FFGrimtooth^000000."
    next()
    mes "[Cetsu]"
    mes "A sword made of ogre tooth, however, cannot become the Grimtooth by mere hardening and sharpening."
    next()
    mes "[Cetsu]"
    mes "The sword must be enchanted with special magic to attain its full potential."
    next()
    mes "[Cetsu]"
    mes "Unfortunately, only one blacksmith, who once lived in Prontera, knows how to do this."
    next()
    mes "[Cetsu]"
    mes "He left Prontera long ago and has not been seen since..."
    heard("orga_sword")
    close()
  end,
}

npc {
  map = "prontera", x = 197, y = 188, dir = 4, sprite = 50, name = "Veeyop",
  replaces = "817:cities/prontera.txt:104", src = "m631:conf/npc_event_ma_sword.txt:419",
  on_click = function()
    if v.mys_sword >= 1 then
      mes "[Veeyop]"
      mes "Balder is the son of Odin, the god we worship."
      mes "That is why this sword Mysteltain frightens me..."
      return close()
    end
    mes "[Veeyop]"
    mes "Have you ever heard of a doomed sword, ^0000FFMysteltain^000000?"
    mes "It's named after a legendary Young Twig which had been used to murder Balder the God of Light..."
    next()
    mes "[Veeyop]"
    mes "According to the legend, it was a very little weak twig. I have no idea about it any more than that."
    next()
    mes "[Veeyop]"
    mes "Whatever the legend, people say there is a way to construct the legendary sword."
    next()
    mes "[Veeyop]"
    mes "Only a blacksmith who used to live in Prontera knew the method. Unfortunately he's gone to nowhere."
    heard("mys_sword")
    close()
  end,
}

-- ------------------------------------------------------------ Prontera's tavern: Gatu

npc {
  map = "prt_in", x = 163, y = 12, dir = 4, sprite = 86, name = "Gatu",
  src = "m631:conf/npc_event_ma_sword.txt:320",
  on_click = function()
    if v.Ma_sword >= 2 then
      mes "[Gatu]"
      mes "Well, if anyone can make a weapon fit to be called a doomed sword, it is that smith in Payon..."
      next()
      mes "[Gatu]"
      mes "If you do meet him, be careful. They say he is a dangerous man."
      return close()
    end
    if v.Ma_sword < 1 then
      mes "..."
      return close()
    end
    mes "[Gatu]"
    mes "Yes... I heard the rumour. Not only ordinary weapons: he could make swords with some strange power in them."
    next()
    mes "[Gatu]"
    mes "For some reason he has shut himself away in the hills ^0000FFnear Payon^000000 and will not come out."
    next()
    mes "[Gatu]"
    mes "He was that good, so the great and the rich brought him all kinds of shady work."
    next()
    mes "[Gatu]"
    mes "He used to be cheerful and well liked. The things he got mixed up in made him silent and gloomy."
    next()
    mes "[Gatu]"
    mes "People still seek him out to ask for something, so his skill must be the real thing."
    mes "Poor fellow. Perhaps it was that skill that made him hide from the world."
    v.Ma_sword = 2
    close()
  end,
}

-- ------------------------------------------------------------ east of Payon: the smith

local SMITH = "[Blacksmith of Doomed Swords]"

local function smith(...)
  mes(SMITH)
  for _, line in ipairs({ ... }) do mes(line) end
end

-- what he says the first time each sword is named; the last page is the list
local TALES = {
  ex_sword = function()
    smith("You want that sword? Ha ha ha... you amuse me.", "Do you know what kind of sword it is?")
    next()
    smith("An executioner took the heads of the condemned with it.", "One by one it drank their blood, and their grudge stayed in the blade.")
    next()
    smith("Even he, who killed without a thought, came to fear it and asked me to destroy it.", "And you want it brought back?")
    next()
    smith("Then bring 2 ^0000FFExecutioner's Mittens^000000, 3 ^0000FFNecklaces of Oblivion^000000, 10 ^0000FFBloody Edges^000000,",
      "and 50 ^0000FFAmulets^000000 to bind the grudge into the blade.")
    next()
    smith("And one test of my own: I think I have the right.", "Bring 1 ^0000FFEmperium^000000. Then I will know you, and make the sword.")
  end,
  orga_sword = function()
    smith("Yes, Grimtooth. A dagger everyone wants.", "Do you know how so small a blade comes by such power?")
    next()
    smith("No ordinary forging makes it.", "You borrow an evil power and lay a curse on the blade. Heh heh... that is what a doomed sword is.")
    next()
    smith("Now and then someone comes who wants the power and does not fear the curse.", "Are you one of them?")
    next()
    smith("So am I. I know what it does to me, and I cannot stop making them.", "I never heard a good word of anyone who walked off with one.")
    next()
    smith("Steel? It melts the moment the curse is laid.", "An ogre's tooth is hard, and it makes the curse its own strength.")
    next()
    smith("Bring 5 ^0000FFCursed Rubies^000000, 10 ^0000FFBlades Lost in Darkness^000000, 100 ^0000FFOgre Teeth^000000 and 1 ^0000FFOld Hilt^000000.",
      "Be quick. It hurts me even to move.")
    next()
    smith("One thing more: prove to me you are fit to swing it.", "Bring 1 ^0000FFEmperium^000000 and I will grant that you are.")
  end,
  mys_sword = function()
    smith("That name... do you know what sword it is?", "It is the doomed sword that killed Balder, son of Odin and of Frigg.")
    next()
    smith("Frigg dreamed of her son's death, and made every thing in the world swear to do him no harm.",
      "The gods threw whatever they liked at him and laughed.")
    next()
    smith("Loki envied him, and found the one plant too weak to have been asked: a small soft twig, Mysteltain.")
    next()
    smith("He put it in the hand of blind Hodur and said: throw.", "The twig became a blade and went through Balder's breast.")
    next()
    smith("To make it or to bear it is forbidden. I knew that.", "I wanted a stronger sword, and I dirtied my hands.")
    next()
    smith("When it came out I lost my wife for it.", "I made it! Why was the guilt hers? Damn them!")
    next()
    smith("...I talk too much. You only want the sword.", "You will have guessed what goes into it.")
    next()
    smith("1 ^0000FFYoung Twig^000000, 1 ^0000FFLoki's Whispers^000000, 1 ^0000FFMother's Nightmare^000000, 1 ^0000FFFoolishness of the Blind^000000.")
    next()
    smith("No thanks. Having come this far I must do it.", "But I must see whether you can master it: bring 1 ^0000FFEmperium^000000 with the rest.")
  end,
}

npc {
  map = "pay_fild08", x = 218, y = 283, dir = 4, sprite = 89, name = "Gloomy Man",
  src = "m631:conf/npc_event_ma_sword.txt:1",
  on_click = function()
    mes "..."
    if v.Ma_sword < 2 then return close() end
    next()
    local known, labels = {}, {}
    for _, s in ipairs(SWORDS) do
      if v[s.var] >= 1 then
        known[#known + 1] = s
        labels[#labels + 1] = s.label
      end
    end
    if #known == 0 then return close() end
    local s = known[menu(table.unpack(labels))]
    if not s then return close() end
    if v[s.var] < 2 then
      TALES[s.var]()
      v[s.var] = 2
      return close()
    end
    smith("So. It is " .. s.label .. " you want.", "Then you have everything I named? Show me.")
    next()
    for _, it in ipairs(s.needs) do
      if count_item(it[1]) < it[2] then
        smith("You do not have it all!")
        return close()
      end
    end
    smith("Good. It is all here.", "Wait a little.")
    next()
    for _, it in ipairs(s.needs) do take_item(it[1], it[2]) end
    give_item(s.item, 1)
    smith("Take it.")
    close()
  end,
}
