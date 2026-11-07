# Mobs See Far

**A Horizon native mod for Minecraft 26.3 — version 1.0.0 — by Bogdan_Tokarev.**

Hostile mobs see you from far away. Not from 16 blocks: from **128**.

| Mob | The game's own sight | With Mobs See Far |
| --- | --- | --- |
| zombie, husk, drowned | 35 blocks | **128 blocks** |
| skeleton, spider, creeper, enderman, witch, slime, pillager | 16 blocks | **128 blocks** |
| warden | 24 blocks | **128 blocks** |

A zombie notices you while you are still a hundred blocks off, picks a path and comes. A skeleton
starts shooting the moment you step into its range, and a creeper is already walking at you from
across the valley.

* It changes **only hostile mobs** — the game's *monster* category. Animals, villagers, golems and the
  player keep the sight the game gave them.
* It changes nothing else about them: same speed, same damage, same drops, same behaviour, same
  spawning and same despawning. Only the distance at which they can notice and target a player.
* Turning the mod off in **Options → Mods** puts every mob's own sight back on its next tick, in the
  same world, without a restart.
* It runs on the server side (single player, and the server thread of a LAN world), so every player
  in the world is seen from as far.
* `/mobsseefar` in chat says what the mod is doing.

Its icon (`icon.png`, 16×16 pixel art, one art pixel per file pixel) is an eye with a muted red iris
and a dark pupil, by Bogdan_Tokarev.

## What "sight" means here

The game keeps the distance at which a mob can spot a player in its `follow_range` attribute — the
one `/attribute` edits, and the one a zombie has 35 of. Every targeting goal reads it
(`NearestAttackableTargetGoal`, the melee and the ranged ones), so raising it raises the whole chain:
noticing, targeting, and how long a path a mob is willing to find to reach you.

Mobs See Far writes `128` into that attribute for each hostile mob once per tick, remembering the value
the game had for that mob, and writes it back when the mod is switched off. The pathfinder's budget
grows with it the way the game grows it itself
(`PathNavigation.updatePathfinderMaxVisitedNodes`: 128 × 16 nodes instead of a zombie's 35 × 16), so a
mob that can see 128 blocks can also path them.

### Why 128 and not more

128 is **the game's own despawn distance**: `MobCategory.getDespawnDistance` discards a hostile mob
that is farther than 128 blocks from every player. So at 128 the mod needs no other rule touched — a
mob that exists is a mob inside its sight, spawning and despawning stay exactly vanilla, and the
pathfinder's budget stays within what the game itself scales to. It is also three and a half times the
35 a zombie has, which is the whole point: they notice you from the edge of the loaded world.

Nothing has to be configured for it either: Horizon's simulation distance is 12 chunks (192 blocks) by
default, and a dedicated server's is 10 chunks (160 blocks) — both beyond 128, so every hostile mob
that exists also ticks and sees you.

## Install

1. In the Horizon launcher, **Mods**, find Mobs See Far, **Install** into the instance you play.
2. Press **Build** on the *Native build of {instance}* row, read the red warning and confirm with
   **I trust these mods: build** (the launcher compiles the game with the mod; the first build
   downloads Rust and takes 20 to 30 minutes).
3. Play that instance. In the game the mod is listed under **Options → Mods**, on by default.

Or from a file: install `mobs_see_far-1.0.0.hnmod` from the Mods page (it is not sandboxed: it is Rust
compiled into your game, so install only mods whose source you trust).

## Command

| Command | What it says |
| --- | --- |
| `/mobsseefar` (or `/msf`) | The mod's version, that it is on, and the distance hostile mobs see you from — or that it is off in Options → Mods |

## How it is built

A [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle (`mixins/mobs_see_far/`) that
the Horizon launcher compiles into the game.

| File | What it is |
| --- | --- |
| `mixins/mobs_see_far/horizon-mixin.toml` | The bundle: one module, four HEAD injections |
| `mixins/mobs_see_far/runtime.rs` | The range, the remembered originals and the decisions — added to the game as `pub mod mobs_see_far` in `client_lib.rs`; also this crate's library, for its tests |
| `mixins/mobs_see_far/mob.rs` | `Mob::tick` of a hostile mob: its follow range becomes 128, or the game's own again |
| `mixins/mobs_see_far/listen.rs` | The switch in Options → Mods, read on the server's thread each tick |
| `mixins/mobs_see_far/command.rs` | `/mobsseefar` |
| `mixins/mobs_see_far/list.rs` | Its row in Options → Mods |

The decision itself is plain Rust — no game types, no dependencies — so `cargo test` runs it on its
own: 8 tests (a hostile mob gets 128; a mob that already has it is left alone; turning the mod off
puts the game's own range back; a mob the mod never touched is never touched; each mob remembers its
own range; turning it on again keeps the original; the remembered ranges stop at their limit; and a
switched-off mod leaves every mob at its own range).

Three injections are on Horizon's stable **hook points** (`native_entity_tick_listened`,
`native_command`, `native_mods` — API 146 is the newest they need). The fourth is a HEAD injection
into the game's own `Mob::tick` (`horizon_client_lib::mobs::Mob::tick`), the one function every ticking
mob goes through, which is where the follow range is applied before the mob's AI, its targeting goals
and its pathfinder read it. That one names a function of Horizon's source, so the mod builds on the
commit it was tested on, `2ebf00a` (Nightly 2026-11-07, hook points API 411).

## Build and check

```sh
cargo test                                       # the logic and its tests
cargo clippy --all-targets -- -D warnings        # no lints
python3 native/tools/native.py check mobs-see-far                 # apply on the pinned Horizon, check the game
python3 native/tools/native.py check mobs-see-far --horizon latest # the newest Nightly
python3 native/tools/native.py package mobs-see-far               # native/target/mobs_see_far-1.0.0.hnmod
```

(from a clone of [horizon-rust/horizon-mods](https://github.com/horizon-rust/horizon-mods), with this
folder as `native/mobs-see-far`).

License: MIT — see `LICENSE`. Author and maintainer: **Bogdan_Tokarev**.
