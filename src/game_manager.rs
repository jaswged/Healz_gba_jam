use agb::display::object::Graphics;
use agb::include_aseprite;

pub static GRAPHICS: &Graphics = include_aseprite!(
    "gfx/corners.aseprite",  // Frame around characters
    "gfx/buttons.aseprite",  // Buttons on Gba ui
    "gfx/boss_hp.aseprite" // 16x16 test
);

// Too large to load
// "gfx/health.aseprite"   // Health ui bars 128 x32
// "gfx/wiz.aseprite"       // Wizard character 48x48
// "gfx/bars.aseprite"     // Hp and mana bars for player  12 x 8
// "gfx/boss_hp.aseprite"   // Hp bar for boss  20 x 16
