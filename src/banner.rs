use agb::display::object::{Object, OamManaged, Tag};

use crate::game_manager::GRAPHICS;

// UI sprites
static BANNER_L_SPRITE: &Tag = GRAPHICS.tags().get("banner_l");
static BANNER_M_SPRITE: &Tag = GRAPHICS.tags().get("banner_mid");

pub struct Banner<'obj> {
    ban_l: Object<'obj>,
    ban_mid: Object<'obj>,
    ban_r: Object<'obj>,
}

impl<'obj> Banner<'obj> {
    pub fn new(object: &'obj OamManaged<'_>) -> Self {
        let mut ban_l = object.object_sprite(BANNER_L_SPRITE.sprite(0));
        ban_l.set_x(0).set_y(128).show();
        let mut ban_mid = object.object_sprite(BANNER_M_SPRITE.sprite(0));
        ban_mid.set_x(32).set_y(128).show();
        let mut ban_mid = object.object_sprite(BANNER_M_SPRITE.sprite(0));
        ban_mid.set_x(64).set_y(128).set_hflip(true).show();
        let mut ban_mid = object.object_sprite(BANNER_M_SPRITE.sprite(0));
        ban_mid.set_x(96).set_y(128).set_hflip(true).show();
        // going to need a short mid, or overlap
        let mut ban_mid = object.object_sprite(BANNER_M_SPRITE.sprite(0));
        ban_mid.set_x(112).set_y(128).set_hflip(true).show();
        let mut ban_mid = object.object_sprite(BANNER_M_SPRITE.sprite(0));
        ban_mid.set_x(144).set_y(128).show();
        let mut ban_mid = object.object_sprite(BANNER_M_SPRITE.sprite(0));
        ban_mid.set_x(176).set_y(128).set_hflip(true).show();
        let mut ban_r = object.object_sprite(BANNER_L_SPRITE.sprite(0));
        ban_r.set_x(208).set_y(128).set_hflip(true).show();

        let banner = Self {
            ban_l,
            ban_mid,
            ban_r,
        };

        banner
    }

    pub fn show(&mut self) {
        self.ban_l.show();
        self.ban_mid.show();
        self.ban_r.show();
    }
    pub fn hide(&mut self) {
        self.ban_l.hide();
        self.ban_mid.hide();
        self.ban_r.hide();
    }
}