// Mobs See Far 1.0.0, a Horizon native mod by Bogdan_Tokarev: hostile mobs see the
// player from 128 blocks away instead of the 16 to 35 the game gives them. A
// zombie, a skeleton, a creeper or a spider notices you while you are still a
// hundred blocks off, and comes for you across the loaded world.
//
// The game reads that distance from the mob's `follow_range` attribute
// (`Mob.attrs.follow_range`, what `/attribute` sets, `Attributes.FOLLOW_RANGE`
// in vanilla), so the mod writes it there, once per mob per tick, and puts the
// game's own value back the moment the player turns the mod off.
//
// This file is both the mod's Cargo library (its tests, `cargo test`) and the
// module the bundle adds to the game as `crate::mobs_see_far`.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;

/// The mod's id (its row, its settings folder), name, version and author.
pub const ID: &str = "mobs_see_far";
pub const NAME: &str = "Mobs See Far";
pub const VERSION: &str = "1.0.0";
pub const AUTHOR: &str = "Bogdan_Tokarev";
pub const DESCRIPTION: &str =
    "Hostile mobs see the player from 128 blocks away instead of the 16 to 35 the game gives them.";

/// How far a hostile mob sees the player, in blocks, while the mod is on.
///
/// 128 is the game's own despawn distance (`MobCategory.getDespawnDistance`:
/// a hostile mob farther than 128 blocks from every player is discarded), so
/// this range needs nothing else changed in the game: a mob that exists is a
/// mob inside its sight, and the mod touches no other rule.
pub const RANGE: f64 = 128.0;

/// The follow ranges the game gives its hostile mobs: 16 for most, 35 for a
/// zombie, a husk or a drowned (the widest). The command says them in chat,
/// and the README compares them with the mod's range.
pub const VANILLA_NARROWEST: f64 = 16.0;
pub const VANILLA_WIDEST: f64 = 35.0;

/// At most this many mobs have their own range remembered at once. Past it the
/// count starts over: a world with more hostile mobs than this is a horde, and
/// a mob whose own range was forgotten keeps the mod's until it despawns.
pub const REMEMBERED_MAX: usize = 8192;

thread_local! {
    /// The mod's switch, read from Options > Mods each tick on the server
    /// thread. On until a hook says otherwise, so a world opened on the mod
    /// gets its range from the first tick.
    static ON: Cell<bool> = const { Cell::new(true) };
    /// Each mob's own follow range, as the game had it before the mod widened
    /// it, so turning the mod off puts every one of them back.
    static REMEMBERED: RefCell<HashMap<u32, f64>> = RefCell::new(HashMap::new());
}

/// The mod's switch, as Options > Mods has it.
pub fn set_on(on: bool) {
    ON.with(|cell| cell.set(on));
}

pub fn on() -> bool {
    ON.with(Cell::get)
}

/// Forgets every remembered range (a new world, or a test).
pub fn forget_all() {
    REMEMBERED.with(|remembered| remembered.borrow_mut().clear());
}

/// How many mobs have their own range remembered on this thread.
pub fn remembered() -> usize {
    REMEMBERED.with(|remembered| remembered.borrow().len())
}

/// The follow range the mod wants a mob to have: `RANGE` while it is on, the
/// range the game gave that mob while it is off. The first call for a mob keeps
/// the game's own value, so turning the mod off puts exactly that back.
pub fn plan(id: u32, current: f64) -> f64 {
    if on() {
        REMEMBERED.with(|remembered| {
            let mut remembered = remembered.borrow_mut();
            if remembered.len() >= REMEMBERED_MAX {
                remembered.clear();
            }
            remembered.entry(id).or_insert(current);
        });
        RANGE
    } else {
        REMEMBERED
            .with(|remembered| remembered.borrow().get(&id).copied())
            .unwrap_or(current)
    }
}

/// What one tick does to a hostile mob's range: `Some` when it changes, `None`
/// when the mob already has it (which keeps the tick cheap for a whole horde).
pub fn step(id: u32, current: f64) -> Option<f64> {
    let wanted = plan(id, current);
    (wanted != current).then_some(wanted)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each test runs on its own thread, so each starts with its own state:
    /// this only spells that out.
    fn fresh() {
        set_on(true);
        forget_all();
    }

    #[test]
    fn a_hostile_mob_sees_the_player_from_the_mods_range() {
        fresh();
        // A zombie's own 35 becomes 128.
        assert_eq!(step(7, 35.0), Some(128.0));
        assert_eq!(plan(7, 35.0), 128.0);
    }

    #[test]
    fn a_mob_that_already_has_it_is_left_alone() {
        fresh();
        step(7, 35.0);
        // The next tick finds 128 in place: nothing to write.
        assert_eq!(step(7, RANGE), None);
    }

    #[test]
    fn turning_the_mod_off_puts_the_games_own_range_back() {
        fresh();
        assert_eq!(step(7, 35.0), Some(128.0));
        set_on(false);
        assert_eq!(step(7, RANGE), Some(35.0));
        // And it stays there.
        assert_eq!(step(7, 35.0), None);
    }

    #[test]
    fn a_mob_the_mod_never_touched_keeps_its_own_range() {
        fresh();
        // Nothing remembered for this one (it spawned while the mod was off).
        set_on(false);
        assert_eq!(step(11, 16.0), None);
    }

    #[test]
    fn each_mob_remembers_its_own_range() {
        fresh();
        for (id, own) in [(1, 35.0), (2, 16.0), (3, 24.0)] {
            assert_eq!(step(id, own), Some(128.0));
        }
        assert_eq!(remembered(), 3);
        set_on(false);
        assert_eq!(step(1, RANGE), Some(35.0));
        assert_eq!(step(2, RANGE), Some(16.0));
        assert_eq!(step(3, RANGE), Some(24.0));
    }

    #[test]
    fn turning_it_on_again_keeps_the_original() {
        fresh();
        step(1, 35.0);
        set_on(false);
        assert_eq!(step(1, RANGE), Some(35.0));
        set_on(true);
        // The remembered value is the game's, not the mod's own 128.
        assert_eq!(step(1, 35.0), Some(128.0));
        set_on(false);
        assert_eq!(step(1, RANGE), Some(35.0));
    }

    #[test]
    fn the_remembered_ranges_stop_at_the_limit() {
        fresh();
        for id in 0..(REMEMBERED_MAX as u32) {
            plan(id, 35.0);
        }
        assert_eq!(remembered(), REMEMBERED_MAX);
        // One more mob starts the count over instead of growing without end.
        plan(REMEMBERED_MAX as u32, 16.0);
        assert_eq!(remembered(), 1);
    }

    #[test]
    fn a_switched_off_mod_leaves_every_mob_at_its_own_range() {
        fresh();
        set_on(false);
        for own in [35.0, 16.0, 24.0, 48.0] {
            assert_eq!(step(5, own), None);
        }
    }
}
