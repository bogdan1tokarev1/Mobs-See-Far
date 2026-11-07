// Every mob that ticks in the server's level: a hostile one sees the player
// from the mod's range while it is on, and goes back to the game's own range
// when the player turns the mod off in Options > Mods.
fn mobs_see_far_mob_tick(mob: &mut crate::mobs::Mob) {
    use crate::mobs_see_far as m;
    // Only the hostile ones (the monster category, as the game files them):
    // animals, villagers and golems keep the sight the game gave them.
    if !mob.hostile() {
        return;
    }
    let Some(range) = m::step(mob.id, mob.attrs.follow_range) else {
        return;
    };
    mob.attrs.follow_range = range;
    // `PathNavigation.updatePathfinderMaxVisitedNodes`: the game asks the
    // pathfinder for a longer path budget whenever the follow range changes,
    // so a mob that can see 128 blocks can also walk them.
    mob.update_max_visited_nodes();
}
