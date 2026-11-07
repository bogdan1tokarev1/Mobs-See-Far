# Mobs See Far

**Hostile mobs see you from 128 blocks.**

Horizon's mobs notice a player from the distance the game gives them: 35 blocks for a zombie, a husk
or a drowned, 16 for a skeleton, a spider, a creeper or an enderman. Mobs See Far raises that to 128 for
every hostile mob, so the monster you meet is one that has been walking at you for a hundred blocks.

| Mob | The game's own sight | With Mobs See Far |
| --- | --- | --- |
| zombie, husk, drowned | 35 blocks | **128 blocks** |
| skeleton, spider, creeper, enderman, witch, slime, pillager | 16 blocks | **128 blocks** |
| warden | 24 blocks | **128 blocks** |

## What it does, exactly

The sight distance is the mob's `follow_range` attribute — what `/attribute` edits. Every targeting
goal in the game reads it, so raising it raises the whole chain: noticing the player, targeting them,
and how long a path the mob is willing to find to get to them.

- **Only hostile mobs.** The game's *monster* category. Animals, villagers, golems and the player keep
  their own sight.
- **Nothing else changes.** Same speed, damage, drops, spawning, despawning and behaviour — only the
  distance at which a mob can notice and target a player. 128 is deliberately the game's own despawn
  distance (`MobCategory.getDespawnDistance`), so no other rule of the game has to be touched: a mob
  that exists is a mob inside its sight.
- **Off means off.** Switching the mod off in Options > Mods puts each mob's own sight back on its next
  tick, in the same world, without a restart: every widened mob's original value is remembered.
- **Server-side.** It runs in the server's tick (single player, and the server thread of a LAN world),
  so every player in the world is seen from as far.
- **`/mobsseefar`** in chat says what the mod is doing.

It adds no blocks, items, textures or recipes, and it keeps no settings files: one switch in
Options > Mods, one chat command. Nothing else has to be configured — Horizon's simulation distance
(12 chunks, 192 blocks, by default) already covers the whole range.

## Under the hood

A horizon-mixins bundle for Minecraft 26.3 (Horizon Nightly 2026-11-07, commit `2ebf00a`) compiled
into the game by the launcher. One module of plain Rust holds the range, the remembered original
values and the decision (8 `cargo test` tests). Four injections wire it up: three on Horizon's stable
hook points — the mod switch read on the server's thread each tick, `/mobsseefar`, and the mod's row in
Options > Mods — and one HEAD injection into the game's own `Mob::tick`, the one function every
ticking mob goes through, where the follow range is set before the mob's AI and pathfinder read it.

It needs hook points API 146 (Horizon 26.3 Nightly has 411).

By **Bogdan_Tokarev**. MIT. Source in the repository this file sits in.
