use agb::display::tiled::{MapLoan, RegularMap, TiledMap, VRamManager};
use agb::include_background_gfx;
use agb::input::{Button, ButtonController};
use crate::sfx::Sfx;

// 2ce8f4  vs 000000
include_background_gfx!(backgrounds, "2ce8f4",
        title => deduplicate "gfx/title-screen.aseprite",
        ui => deduplicate "gfx/dungeon.aseprite",
        cave_blank => deduplicate "gfx/cave_blank.aseprite",
        dungeon_blank => deduplicate "gfx/dungeon_blank.aseprite",
        field_blank =>  deduplicate "gfx/field_blank.aseprite",
        sewer_blank => deduplicate "gfx/sewer_blank.aseprite",
        ending => deduplicate "gfx/ending_page.aseprite",
        game_over => deduplicate "gfx/game_over.aseprite",
        pause => deduplicate "gfx/pause.aseprite",
        names => deduplicate "gfx/names_and_banner.aseprite",);

pub enum SplashScreen {
    Start,
    End,
    Pause,
    Over,
}

#[derive(Clone)]
pub enum Terrain {
    Cave,
    Dungeon,
    Field,
    Sewer,
}

pub fn show_background_terrain(bg: &mut MapLoan<RegularMap>, vram: &mut VRamManager, which: Terrain){
    bg.clear(vram);
    bg.commit(vram);
    let tile_data = match which {
        Terrain::Cave => &backgrounds::cave_blank,
        Terrain::Dungeon => &backgrounds::dungeon_blank,
        Terrain::Field => &backgrounds::field_blank,
        Terrain::Sewer => &backgrounds::sewer_blank,
    };
    // vram.replace_tile(tile_data, 0, tile_data, 0);
    bg.set_visible(false);
    bg.fill_with(vram, tile_data);
    bg.commit(vram);
    bg.set_visible(true);
}

pub fn show_background_names(bg: &mut MapLoan<RegularMap>, vram: &mut VRamManager){
    bg.clear(vram);
    bg.commit(vram);
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

pub fn hide_background_ui(bg: &mut MapLoan<RegularMap>) {
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
    map.commit(vram);
    map.set_scroll_pos((0i16, 0i16));
    let tile_data = match which {
        SplashScreen::Start => &backgrounds::title,
        SplashScreen::End => &backgrounds::ending,
        SplashScreen::Over => &backgrounds::game_over,
        SplashScreen::Pause => &backgrounds::pause,
    };

    let vblank = agb::interrupt::VBlank::get();

    vblank.wait_for_vblank();
    map.fill_with(vram, tile_data);
    map.commit(vram);
    vram.set_background_palettes(backgrounds::PALETTES);
    map.set_visible(true);

    loop {
        input.update();
        if matches!(which, SplashScreen::Start) && input.is_just_pressed(Button::SELECT) {
            // show help Pause screen
            show_splash_screen(input, vram, SplashScreen::Pause, sfx, map);
        }

        if input.is_just_pressed(
            Button::A
                | Button::B
                | Button::START
                | Button::SELECT,
        ) {
            break;
        }

        sfx.frame();
        vblank.wait_for_vblank();
    }

    map.set_visible(false);
    map.clear(vram);
    map.commit(vram);
}
