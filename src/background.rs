use agb::display::tiled::{
    MapLoan, RegularBackgroundSize, RegularMap, TileFormat, Tiled0, TiledMap, VRamManager
};
use agb::display::Priority;
use agb::include_background_gfx;
use agb::input::{Button, ButtonController};
use crate::sfx::Sfx;

include_background_gfx!(backgrounds, "000000",
        title => deduplicate "gfx/title-screen.aseprite",
        dungeon => deduplicate "gfx/dungeon.aseprite",
        dungeon_blank => deduplicate "gfx/dungeon_blank.aseprite",
        ending => deduplicate "gfx/ending_page.aseprite");

pub fn show_dungeon_screen<'obj>(vram: &mut VRamManager, tiled: &'obj Tiled0<'obj>, is_blank: bool) -> MapLoan<'obj, RegularMap> {
    let mut bg: MapLoan<RegularMap> = tiled.background(Priority::P2,
                                  RegularBackgroundSize::Background32x32,
                                  TileFormat::FourBpp);
    bg.set_scroll_pos((0i16, 0));
    vram.set_background_palettes(backgrounds::PALETTES);
    bg.set_visible(false);
    if is_blank {
        bg.fill_with(vram, &backgrounds::dungeon_blank);
    } else {
        bg.fill_with(vram, &backgrounds::dungeon);
    }
    bg.commit(vram);
    // sfx.frame(); = // self.mixer.frame();

    bg.set_visible(true);
    bg
}

pub fn tear_down_dungeon_screen(mut bg: &mut MapLoan<RegularMap>, vram: &mut VRamManager) {
    bg.set_visible(false);
    bg.clear(vram);
    bg.commit(vram);
}

pub fn show_splash_screen(input: &mut ButtonController, vram: &mut VRamManager, tiled: &Tiled0, sfx: &mut Sfx) {
    let mut background: MapLoan<RegularMap> = tiled.background(
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
        sfx.frame();
        if input.is_just_pressed(Button::A | Button::B | Button::START | Button::SELECT) {
            // todo add a help background page if Select/B is picked
            break;
        }
        agb::display::busy_wait_for_vblank();
    }
    background.set_visible(false);
    background.clear(vram);
    background.commit(vram);
}

pub fn show_game_over_screen(input: &mut ButtonController, vram: &mut VRamManager, tiled: &Tiled0, sfx: &mut Sfx,) {
    let mut ending_bg = tiled.background(
        Priority::P1,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp,
    );
    vram.set_background_palettes(backgrounds::PALETTES);
    ending_bg.set_visible(false);

    ending_bg.fill_with(vram, &backgrounds::ending);
    ending_bg.commit(vram);
    ending_bg.set_visible(true);

    loop {
        input.update();
        sfx.frame();
        if input.is_just_pressed(Button::A | Button::B | Button::START | Button::SELECT) {
            break;
        }
        agb::display::busy_wait_for_vblank();
    }
    ending_bg.set_visible(false);
    ending_bg.clear(vram);
    ending_bg.commit(vram);
}
