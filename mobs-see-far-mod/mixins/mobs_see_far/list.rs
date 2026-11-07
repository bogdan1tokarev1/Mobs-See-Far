// The mod's row in Options > Mods: its name, its version and the switch.
fn mobs_see_far_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use horizon_client_lib::mobs_see_far as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: format!("{} By {}.", m::DESCRIPTION, m::AUTHOR),
        keys: Vec::new(),
    });
}
