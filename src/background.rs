use agb::display::tiled::{MapLoan, RegularMap, TiledMap, VRamManager};
use agb::include_background_gfx;
use agb::input::ButtonController;
use crate::sfx::Sfx;

// 2ce8f4  vs 000000
include_background_gfx!(backgrounds, "2ce8f4",
        title => deduplicate "gfx/title-screen.aseprite",
        ui => deduplicate "gfx/dungeon.aseprite",
        dungeon_blank => deduplicate "gfx/dungeon_blank.aseprite",
        ending => deduplicate "gfx/ending_page.aseprite",
        game_over => deduplicate "gfx/game_over.aseprite",
        // help => deduplicate "gfx/help-text.aseprite",
        names => deduplicate "gfx/names_and_banner.aseprite",);

pub enum SplashScreen {
    Start,
    End,
    Over,
}

pub enum Terrain {
    Dungeon,
}

pub fn show_background_terrain(mut bg: MapLoan<RegularMap>, vram: &mut VRamManager, which: Terrain){
    bg.clear(vram);
    let tile_data = match which {
        Terrain::Dungeon => &backgrounds::dungeon_blank,
    };

    bg.set_visible(false);
    bg.fill_with(vram, tile_data);
    bg.commit(vram);
    bg.set_visible(true);
}

pub fn show_background_names(mut bg: MapLoan<RegularMap>, vram: &mut VRamManager){
    bg.clear(vram);
    bg.set_visible(false);
    bg.fill_with(vram, &backgrounds::names);
    bg.commit(vram);
    bg.set_visible(true);
}

pub fn show_background_ui(bg: &mut MapLoan<RegularMap>, vram: &mut VRamManager){
    bg.set_visible(false);
    bg.fill_with(vram, &backgrounds::ui);
    bg.commit(vram);
    bg.set_visible(true);
}

pub fn hide_background_ui(bg: &mut MapLoan<RegularMap>, vram: &mut VRamManager) {
    bg.set_visible(false);
}

pub fn show_splash_screen(
    input: &mut ButtonController,
    vram: &mut VRamManager,
    which: SplashScreen,
    sfx: &mut Sfx,
    map: &mut RegularMap,
) {
    map.clear(vram);
    map.set_scroll_pos((0i16, 0i16));
    let tile_data = match which {
        SplashScreen::Start => &backgrounds::title,
        SplashScreen::End => &backgrounds::ending,
        SplashScreen::Over => &backgrounds::game_over,
    };

    let vblank = agb::interrupt::VBlank::get();

    vblank.wait_for_vblank();
    map.fill_with(vram, tile_data);
    map.commit(vram);
    vram.set_background_palettes(backgrounds::PALETTES);
    map.set_visible(true);

    loop {
        input.update();
        if input.is_just_pressed(
            agb::input::Button::A
                | agb::input::Button::B
                | agb::input::Button::START
                | agb::input::Button::SELECT,
        ) {
            break;
        }

        sfx.frame();
        vblank.wait_for_vblank();
    }

    map.set_visible(false);
    map.clear(vram);
}
