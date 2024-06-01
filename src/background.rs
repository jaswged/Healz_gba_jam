use agb::display::Priority;
use agb::display::tiled::{RegularBackgroundSize, RegularMap, Tiled0, TiledMap, TileFormat, VRamManager};
use agb::include_background_gfx;
use agb::input::{Button, ButtonController};

include_background_gfx!(backgrounds, "000000",
        // level => deduplicate "gfx/dungeon_floor.png",
        title => deduplicate "gfx/title-screen.aseprite",
        dungeon => deduplicate "gfx/dungeon.aseprite");

pub fn show_dungeon_background(vram: &mut VRamManager, tiled: &Tiled0) {
    let mut bg = tiled.background(Priority::P2,
                                  RegularBackgroundSize::Background32x32,
                                  TileFormat::FourBpp);
    bg.set_scroll_pos((0i16, 0));
    vram.set_background_palettes(backgrounds::PALETTES);
    bg.set_visible(false);
    bg.fill_with(vram, &backgrounds::dungeon);
    bg.commit(vram);
    // sfx.frame(); = // self.mixer.frame();

    bg.set_visible(true);
}

pub fn show_splash_screen(input: &mut ButtonController, vram: &mut VRamManager, tiled: &Tiled0) {
    let mut background = tiled.background(
        Priority::P1,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp,
    );

    background.set_scroll_pos((0i16, 0));
    vram.set_background_palettes(backgrounds::PALETTES);

    background.set_visible(false);

    background.fill_with(vram, &backgrounds::title);
    background.commit(vram);
    // sfx.frame(); = // self.mixer.frame();

    background.set_visible(true);

    loop {
        input.update();
        if input.is_just_pressed(
            Button::A
                | Button::B
                | Button::START
                | Button::SELECT,
        ) {
            break;
        }
        agb::display::busy_wait_for_vblank();
    }
    background.set_visible(false);
    background.clear(vram);
    background.commit( vram);
}
