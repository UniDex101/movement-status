fn movement_status_hud( game: &mut crate::game::Game<'_>, batch: &mut horizon_client::hud::HudBatch, corners: &mut horizon_client::extras_hud::Corners, ) {
    use crate::game::mod_movement_status as m;
    use horizon_client::extras::Corner;
    use winit::keyboard::KeyCode;

    if !game.native_on(m::ID) || !game.native_corner_shown(Corner::BottomLeft) { return; }

    let Some(p) = game.player.as_ref() else { return; };

    // Logical held state (stays true under vanilla toggle)
    let sprint_logical = game
        .native_mapping("key.sprint")
        .map(|(held, _)| held)
        .unwrap_or(false);
    let sneak_logical = game
        .native_mapping("key.sneak")
        .map(|(held, _)| held)
        .unwrap_or(false);

    // Physical key state (actual key currently pressed)
    // Defaults are Left Control / Left Shift; rebinds will need a follow-up
    // that maps the bound key string → KeyCode if you want full rebind support.
    let sprint_phys = game.physical_keys.contains(&KeyCode::ControlLeft);
    let sneak_phys = game.physical_keys.contains(&KeyCode::ShiftLeft);

    let toggle_sprint = game.settings.toggle_sprint;
    let toggle_sneak = game.settings.toggle_crouch;

    // Prefer physical for the "Held" decision; fall back to logical + setting
    let sprint_held = sprint_phys || (sprint_logical && !toggle_sprint);
    let sneak_held = sneak_phys || (sneak_logical && !toggle_sneak);

    let lines = m::status_lines(
        p.sprinting,
        p.local_crouching,
        p.flying,
        p.fall_flying,
        sprint_held,
        sneak_held,
        toggle_sprint,
        toggle_sneak,
    );

    if lines.is_empty() { return; }

    let w = lines
        .iter()
        .map(|l| game.native_text_width(l))
        .fold(0.0, f32::max)
        + 8.0;
    let h = 10.0 * lines.len() as f32 + 4.0;

    let (x, y) = corners.place(Corner::BottomLeft, w, h);

    if m::background(|| game.native_setting(m::ID, "background")) { game.native_fill(batch, [x, y, w, h], m::BACKGROUND_COLOR); }

    for (i, line) in lines.iter().enumerate() {
        game.native_text(
            batch,
            line,
            [x + 4.0, y + 2.0 + 10.0 * i as f32],
            m::INK,
            true,
        );
    }
}
