// Options > Mods keeps the mod's switch. The server's game asks its mods once
// each tick (for every family of entities) whether they hear entity ticks, and
// it does it on the thread that ticks the mobs, so the switch is read here:
// turning the mod off reaches the mobs on their next tick, not on the next
// world.
fn mobs_see_far_listen(game: &mut crate::game::Game<'_>) {
    use horizon_client_lib::mobs_see_far as m;
    m::set_on(game.native_on(m::ID));
}
