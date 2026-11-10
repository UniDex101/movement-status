# Movement Status

A lightweight native HUD for Horizon that shows your current movement state in the corner of the screen.

It reads the live player state (sprinting, sneaking, flying, elytra gliding) and the physical vs. logical key state so it can correctly distinguish between a key that is being held and a key that is toggled via vanilla’s Toggle Sprint / Toggle Crouch options.

## Features

- Live movement status in the bottom-left corner:
  - `Sprint [Held]` / `Sprint [Toggled]`
  - `Sneak [Held]` / `Sneak [Toggled]`
  - `Flying [Creative]` / `Flying [Elytra]`
- Multiple states can be shown at once (sprint + sneak, etc.)
- Optional semi-transparent background box
- Respects vanilla Toggle Sprint and Toggle Crouch settings
- Uses the real physical key state so the Held/Toggled labels stay accurate

## Requirements

- Horizon (Minecraft: Java Edition 26.3)
- Native mod support enabled in the launcher

## Installation

1. Install the mod from the launcher’s Mods page (or drop the `.hnmod` into `horizon-native-mods/`).
2. Click **Build** and confirm the native-code warning.
3. Once built, turn the mod on under Options → Mods.

## Settings

A single toggle controls the background box. Open Options → Mods → Movement Status → Settings to turn it on or off.

## Notes

This is a status HUD only. It does not change movement, keybinds, or gameplay. It simply reports what the game is already doing.

The physical key checks currently use the vanilla defaults (Left Control for sprint, Left Shift for sneak). Custom keybinds may show the wrong Held/Toggled label until a future update maps the bound key.

## License

MIT
