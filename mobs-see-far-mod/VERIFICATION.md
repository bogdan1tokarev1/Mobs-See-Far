# Mobs See Far 1.0.0 — how it was checked

Author of the checks: the mod's own author (Bogdan_Tokarev). Everything here is reproducible with the
commands in `README.md`; this file records what was run and what came out, including what could **not**
be run outside a real game.

## What it was built on

| | |
| --- | --- |
| Game | Minecraft **26.3** |
| Horizon | Nightly, commit **`2ebf00a0281c592f79191e76fce0e7d522c6f425`** (2026-11-07), hook points API **411** |
| horizon-mixins | **0.6.0**, commit `e8cd0aa38cffef49cb2e088ef716baa107148c02` |
| Hook points the bundle needs | **146** (the newest it hooks: `native_entity_tick_listened`) |
| Rust | 1.99.1 (`cargo 1.99.1`, `rustc 1.99.1`), edition 2024 |
| The range | **128 blocks** — the game's own despawn distance, so no other rule is touched |

## 1. The mod's own logic — `cargo test`: 8 of 8

```sh
cargo test --offline
```

```
running 8 tests
test tests::a_hostile_mob_sees_the_player_from_the_mods_range ... ok
test tests::a_mob_that_already_has_it_is_left_alone ... ok
test tests::a_mob_the_mod_never_touched_keeps_its_own_range ... ok
test tests::a_switched_off_mod_leaves_every_mob_at_its_own_range ... ok
test tests::each_mob_remembers_its_own_range ... ok
test tests::turning_it_on_again_keeps_the_original ... ok
test tests::turning_the_mod_off_puts_the_games_own_range_back ... ok
test tests::the_remembered_ranges_stop_at_the_limit ... ok

test result: ok. 8 passed; 0 failed
```

`cargo clippy --offline --all-targets -- -D warnings` — clean, no lints.

`cargo run --example range` prints what the mod does to a few mobs:

```
mob                           own     mod on    mod off
minecraft:skeleton             16        128         16
minecraft:zombie               35        128         35
minecraft:husk                 35        128         35
minecraft:drowned              35        128         35
minecraft:warden               24        128         24
minecraft:bee                  48        128         48
```

## 2. The bundle applying to Horizon's real source — `horizon-mixins apply`: ok

```sh
horizon-mixins apply --workspace <horizon> --mods <bundle> --out <stage>
```

```json
{ "injections": 4, "modules": 1, "status": "ok", "stage_changed": true }
```

It changed exactly two files of the game and added the module:

| File | Change |
| --- | --- |
| `crates/horizon-client/src/client_lib.rs` | `pub mod mobs_see_far { … }` — the mod's logic, compiled into the crate whose mob AI the game runs |
| `crates/horizon-client/src/mobs.rs` | the top of `Mob::tick`: `mobs_see_far_mob_tick(&mut *self)` before `track_section()` and `ai::tick` |
| `crates/horizon-client/src/game/native_hooks.rs` | `mobs_see_far_listen`, `mobs_see_far_command`, `mobs_see_far_listed` at the three hook points |

Checked by reading the staged source back (`grep`), not just by the tool's report. Nothing else of the
game is touched: no despawn rule, no spawn rule, no attributes other than the mob's own follow range.

## 3. The game compiling with the mod — `cargo check`, exit 0

```sh
RUSTFLAGS="-D warnings" cargo check --locked -p horizon-client -p horizon-client-lib
```

* full run on a cold target directory: **finished in 3 m 18 s, exit 0**;
* the re-run on the final files: **finished in 24.6 s, exit 0**.

No warnings (they are errors under `-D warnings`), no errors. The injected helpers are type-checked
against the real `Mob`, `MobWorld`, `Game` and `NativeMod` types of this Horizon commit, and the whole
client library compiles with them inside.

## 4. The engine's own mob tests, with the mod compiled in — 575 passed, 0 failed

The staged game's test binary was built and its mob tests run, so the injected code executed inside the
engine's own tick paths:

```sh
cargo test --locked -p horizon-client-lib --lib mobs::
```

```
test result: ok. 575 passed; 0 failed; 25 ignored; 1626 filtered out
```

Run twice: on the 200-block build with the despawn injection, and again on this 128-block build with
four injections. Nothing panicked, nothing hung, and no engine assertion broke.

## 5. The package

```sh
python3 native/tools/native.py package mobs-see-far
```

```
mobs_see_far-1.0.0.hnmod — 6640 bytes
sha256 81714b7d1ce0f31641ad1a0971fbc96531f9dde8347e8f82c967b3c543b8b5ad
```

Contents: `horizon-native.json` (schema 1, game 26.3, hooks 146, horizon `2ebf00a`, mixins `e8cd0aa`),
the module `runtime.rs`, the four helpers, `horizon-mixin.toml` and the mod's `LICENSE`.

## What could not be checked here

**The mod was not launched in the game.** The machine the checks ran on has no Horizon launcher, no
prepared Minecraft 26.3 resources and no GPU or display, so `native.py build`/`proof` and the game
itself cannot run there. For the same reason the engine's own tests that need a full resource root
(`.local/horizon-client/26.3`) skip themselves; of the 575 that passed, those that build a world were
no-ops, and the ones that run without resources are the ones that actually exercised the code.

So "no crashes in game" rests on: the whole client compiling with `-D warnings`, the injections
landing exactly at the tick point, 575 engine tests passing with the code live, and the injected code
itself having no index, no unwrap, no slicing, no casting of user data, and one `RefCell` per thread
that is never borrowed twice (each call borrows it once, briefly).

## What to check in the game

1. **Options → Mods** lists *Mobs See Far 1.0.0*, and it is on.
2. `/mobsseefar` (or `/msf`) in chat answers with the version and 128 blocks.
3. Stand in the open at night and watch the horizon: a zombie or a skeleton starts walking (or
   shooting) at you from far outside 35 blocks — up to the edge of the loaded world.
4. **Options → Mods** → turn it off, wait a second, and the mobs go back to the game's own sight: a
   skeleton stops shooting at 40 blocks, and `/mobsseefar` says it is off.
5. Turn it back on and the range returns at once, without a restart.
