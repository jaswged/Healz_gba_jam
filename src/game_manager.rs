use agb::display::object::Graphics;
use agb::include_aseprite;

pub static GRAPHICS: &Graphics = include_aseprite!(
    "gfx/corners.aseprite",  // Frame around characters
    "gfx/buttons.aseprite",  // Buttons on Gba ui
    "gfx/boss_hp.aseprite", // boss health bar size 16x16
    "gfx/bars.aseprite" // character health bars size 8x8
    ,"gfx/dungeon.aseprite" // background tiles
    ,"gfx/characters.aseprite" // Character sprites
    ,"gfx/bosses.aseprite" // Character sprites
);

pub struct GameManager{
    pub currently_selected_char: i32
}
