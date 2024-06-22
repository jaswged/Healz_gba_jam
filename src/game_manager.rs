use agb::display::object::Graphics;
use agb::include_aseprite;

pub static GRAPHICS: &Graphics = include_aseprite!(
    "gfx/corners.aseprite",       // Frame around characters
    "gfx/buttons.aseprite",       // Buttons on Gba ui
    "gfx/boss_hp.aseprite",       // boss health bar size 16x16
    "gfx/characters.aseprite",    // Character sprites
    "gfx/bosses.aseprite",        // Boss sprites and names
    "gfx/banner.aseprite",        // Bottom Banner
    "gfx/health.aseprite",        // health bars
    "gfx/spell_effects.aseprite"  // Spell effects
);

pub struct GameManager {
    pub currently_selected_char: usize,
    // pub chars: [Character<'obj>; 1]
}

impl GameManager {
    pub fn new() -> Self {
        GameManager {
            currently_selected_char: 0,
        }
    }
}
