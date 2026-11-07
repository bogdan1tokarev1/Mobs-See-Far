// A table of what the mod does to a few hostile mobs' sight, and what happens to
// them when the player turns it off and on again:
//
//     cargo run --example range
//
// The decisions are the mod's own code (this crate's library), so this is what
// the game's mobs get: their own follow range widened to 128, and back.
use horizon_mod_mobs_see_far::{RANGE, VANILLA_NARROWEST, VANILLA_WIDEST, VERSION, forget_all, plan, set_on};

fn main() {
    // The follow ranges the game gives them: a skeleton's 16, a zombie's,
    // husk's and drowned's 35, a warden's 24, an enderman's 16 (and 48 for a
    // mob given a range by its own code, such as a bee, to show it goes back).
    let mobs = [
        ("minecraft:skeleton", 16.0),
        ("minecraft:zombie", 35.0),
        ("minecraft:husk", 35.0),
        ("minecraft:drowned", 35.0),
        ("minecraft:warden", 24.0),
        ("minecraft:bee", 48.0),
    ];
    println!("Mobs See Far {VERSION}: hostile mobs see the player from {RANGE:.0} blocks");
    println!("(the game gives them {VANILLA_NARROWEST:.0} to {VANILLA_WIDEST:.0})\n");
    println!("{:<22} {:>10} {:>10} {:>10}", "mob", "own", "mod on", "mod off");
    forget_all();
    set_on(true);
    // Each mob is its own entity id: the mod remembers a range for each of them.
    let on: Vec<f64> = mobs
        .iter()
        .enumerate()
        .map(|(i, (_, own))| plan(i as u32 + 1, *own))
        .collect();
    set_on(false);
    let off: Vec<f64> = on
        .iter()
        .enumerate()
        .map(|(i, widened)| plan(i as u32 + 1, *widened))
        .collect();
    for (i, (name, own)) in mobs.iter().enumerate() {
        println!(
            "{:<22} {:>10.0} {:>10.0} {:>10.0}",
            name, own, on[i], off[i]
        );
    }
    set_on(true);
}
