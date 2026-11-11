-- Alberta's child who loves dolls: give her one and she gives a sweet back.
--
-- Source: mod0631 conf/npc_event_doll.txt (archived 2003-12-18; loaded by
-- its map_athena.conf). Tier athena2003, jRO lineage. 817 has her as a
-- passer-by ("Kid", cities/alberta.txt:427) with the first two pages only,
-- so there the wish for a monkey doll led nowhere.
--
-- What is whose: the trades (which doll for what) are mod0631's; her first
-- two pages are 817's English, mended; the rest is our translation of
-- mod0631's Japanese, shortened to fit the dialogue window. She stands on
-- 817's cell (117,135; mod0631: 117,134). mod0631 tells a girl and a boy
-- apart only by "big sister" and "big brother"; 817's opening is its line to
-- a boy, so a girl gets ours. The dolls are named as this client names them.

local DOLLS = {
  {
    label = "Poring Doll", doll = 741, gift = 529, n = 1,
    { "Oh! I have a Poring Doll too, but if you give me one I will take good care of it!", "Poring Doll, so fluffy~" },
    { "I'll give you a present back!", "Here, a candy I was hiding." },
    { "Hehe. Mom scolds me when I eat too many, so you have one too.", "Thanks for the doll!" },
  },
  {
    label = "Chonchon Doll", doll = 742, gift = 530, n = 1,
    { "Eh... a Chonchon... doll..." },
    { "Uuh... you went to the trouble of giving it to me, so I have to take care of it...", "Here. You can have this." },
    { "You get cavities if you eat a lot, so eat it a little at a time.", "Thank you..." },
    { "(Chonchon... mutter mutter)" },
  },
  {
    label = "Puppet", doll = 740, gift = 530, n = 1,
    { "Wow! A bunny!", "It's so cute! Thank you!" },
    { "So soft~ I know! I'll give you a present too!", "Um... there! Here you are!" },
    { "This is a candy Santa gave me~ Try it.", "Thank you for the bunny! I'll sleep with it every night~" },
  },
  {
    label = "Rocker Doll", doll = 752, gift = 532, n = 7,
    { "Ah, a grasshopper doll. It's a Rocker~", "I don't like them when they hop, but the doll is cute." },
    { "Thank you~ I'll give you something too.", "I think there was something Grandpa gave me..." },
    { "We have lots at home, so you get a share~", "Thanks for the doll!" },
  },
  {
    label = "Spore Doll", doll = 743, gift = 538, n = 5,
    { "Eww... a mushroom...", "Mom always scolds me to eat my mushrooms..." },
    { "But a doll is cute, so I'll take it. I still won't eat mushrooms...", "I'll give you a present too." },
    { "My mom made these. Have some!", "Thanks for the doll~" },
  },
  {
    label = "Osiris Doll", doll = 751, gift = 522, n = 2,
    { "Scary!", "I've never seen a doll like this..." },
    { "It's so rare, I want to show Daddy!", "So you can have these!" },
    { "I went into the forest without telling Mom and found them.", "There are two, so share with your sweetheart! Ehehe..." },
  },
  {
    label = "Baphomet Doll", doll = 750, gift = 526, n = 2,
    { "Ooh! I've never seen one of these!", "It's cute, but a bit scary." },
    { "You gave me such a pretty doll, so I'll give you something too~" },
    { "Daddy gave me this when I was sick.", "I'm well now, so you can have it~ Thank you for the doll!" },
  },
  {
    label = "Racoon Doll", doll = 754, gift = 539, n = 3,
    { "Ah, a raccoon doll~", "I don't like Smokies, but the doll is cute!" },
    { "So cute~ Thank you!" },
    { "Have some of Grandma's cake! Don't you like sweet things?", "It's good, try it~ Thanks for the cute doll!" },
  },
  {
    label = "Yoyo Doll", doll = 753, gift = 608, n = 1,
    { "Waah!!! A monkey! So cute~", "Thank you so much! This is the one I wanted~" },
    { "So cute~ I know! I want to thank you." },
    { "I picked this up on the way to another village with Daddy. It looks like a seed.",
      "I planted it in our garden and nothing came up, so you can have it!" },
    { "Thanks for the doll!" },
  },
}

local function kid(lines)
  mes "[Kid]"
  for _, line in ipairs(lines) do mes(line) end
end

npc {
  map = "alberta", x = 117, y = 135, dir = 7, sprite = 96, name = "Kid",
  replaces = "817:cities/alberta.txt:427", src = "m631:conf/npc_event_doll.txt:2",
  on_click = function()
    if player.sex == 1 then
      kid { "Heh? Is this your first time in Alberta?", "Heh... you don't like dolls, do you? Boys don't like girlish stuff.",
        "I like dolls a looooooooot!" }
    else
      kid { "Hello! Hey, hey, do you like dolls?", "I like dolls a looooooooot!" }
    end
    next()
    kid { "I mean~ I want a new doll~", "I wish my dad will buy me a doll for my birthday present." }
    next()
    if menu("That's nice", "Shall I give you a doll?") == 1 then
      kid { "I am wondering what kind of doll he will buy...", "I hope it will be a yoyo the monkey doll. Heh~ Cute yoyo doll. So cute." }
      return close()
    end
    kid { "Eh? You'll give me one? A doll?", "Yay! What kind of doll is it!?" }
    next()
    local labels = {}
    for i, d in ipairs(DOLLS) do labels[i] = d.label end
    local d = DOLLS[menu(table.unpack(labels))]
    if not d then return close() end
    if count_item(d.doll) < 1 then
      kid { "Liar! You don't have that doll!", "(Hmph!)" }
      return close()
    end
    -- the doll and her present change hands before her last page
    for i, page in ipairs(d) do
      if i == #d then
        take_item(d.doll, 1)
        give_item(d.gift, d.n)
      end
      kid(page)
      if i < #d then next() end
    end
    close()
  end,
}
