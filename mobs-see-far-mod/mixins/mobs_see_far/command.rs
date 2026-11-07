// `/mobsseefar` (or the short `/msf`) says in chat what the mod is doing, so a
// player can check it without going back to Options > Mods.
fn mobs_see_far_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use horizon_client_lib::mobs_see_far as m;
    let Some(args) = ["mobsseefar", "msf"]
        .into_iter()
        .find_map(|name| command.strip_prefix(name))
    else {
        return;
    };
    // `/mobsseefar` and `/msf anything`, never `/mobsseefarx`.
    if !(args.is_empty() || args.starts_with(' ')) {
        return;
    }
    *ran = true;
    if game.native_on(m::ID) {
        game.native_chat(&format!(
            "{} {}: hostile mobs see you from {:.0} blocks (the game gives them {:.0} to {:.0}).",
            m::NAME,
            m::VERSION,
            m::RANGE,
            m::VANILLA_NARROWEST,
            m::VANILLA_WIDEST
        ));
    } else {
        game.native_chat(&format!(
            "{} {}: off, in Options > Mods.",
            m::NAME,
            m::VERSION
        ));
    }
}
