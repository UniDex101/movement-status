// Pure logic + settings. Also the library the game injects.

pub const ID: &str = "movement_status";
pub const NAME: &str = "Movement Status";
pub const VERSION: &str = "0.1.0";
pub const DESCRIPTION: &str = "Sprint / sneak / flying status lines. Reads current player state and physical key holds. Optional background box.";

pub const BACKGROUND: u32 = 0x7800_0000;
pub const INK: u32 = 0xFF_FFFF;

/// Background box on unless the stored value is "false".
pub fn read_background(stored: Option<String>) -> bool {
    stored.is_none_or(|v| v.trim() != "false")
}

pub fn background_label(on: bool) -> String {
    format!("Background: {}", if on { "ON" } else { "OFF" })
}

thread_local! {
    static BACKGROUND: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
}

pub fn background(get: impl FnOnce() -> Option<String>) -> bool {
    BACKGROUND.with(|c| {
        let v = c.get().unwrap_or_else(|| read_background(get()));
        c.set(Some(v));
        v
    })
}

pub fn set_background(on: bool) {
    BACKGROUND.with(|c| c.set(Some(on)));
}

/// Build the lines to show. Empty when nothing is active.
pub fn status_lines(
    sprinting: bool,
    crouching: bool,
    flying: bool,
    sprint_held: bool,
    sneak_held: bool,
) -> Vec<String> {
    let mut lines = Vec::new();

    if sprinting {
        let mode = if sprint_held { "[Held]" } else { "[Toggled]" };
        lines.push(format!("Sprint {mode}"));
    }

    if crouching {
        let mode = if sneak_held { "[Held]" } else { "[Toggled]" };
        lines.push(format!("Sneak {mode}"));
    }

    if flying {
        lines.push("Flying".to_string());
    }

    lines
}
