// change our imports to include what we will use
use alloc::vec::Vec;
use agb::{display::object::{Graphics, Object, OamManaged, Tag}};
use agb::display::Priority;
use agb::display::tiled::{RegularBackgroundSize, Tiled0, VRamManager};
use agb::display::object::Sprite;

use crate::game_manager::GRAPHICS;

static HEALTH_BAR_LETTERS: &Tag = GRAPHICS.tags().get("hp_begin");
static HEALTH_BAR_START: &Tag = GRAPHICS.tags().get("hp_start");
static HEALTH_BAR_START_CONT: &Tag = GRAPHICS.tags().get("hp_start_cont");
static HEALTH_CONT: &Tag = GRAPHICS.tags().get("hp_cont");
static HEALTH_MID: &Tag = GRAPHICS.tags().get("hp_filled");
static HEALTH_MID_EMPTY: &Tag = GRAPHICS.tags().get("hp_empty");
static HEALTH_BAR_END: &Tag = GRAPHICS.tags().get("hp_end");

// redo tags
static HP_LETTERS: &Tag = GRAPHICS.tags().get("hp_letters");
static HP_LETTERS_END: &Tag = GRAPHICS.tags().get("hp_letters_end");
static HP_FRAME: &Tag = GRAPHICS.tags().get("hp_frame");
static HP_END: &Tag = GRAPHICS.tags().get("hp_end");
static HP_INFILL: &Tag = GRAPHICS.tags().get("hp_infill");

static MID_MT_SPRITE: &Sprite = HEALTH_MID_EMPTY.sprite(0);
static MID_FILL_SPRITE: &Sprite = HEALTH_MID.sprite(0);
// todo other 7 sprites references

pub struct HealthBar<'obj> {
    health_letters: Object<'obj>,
    health_start: Object<'obj>,
    health_mid1: Object<'obj>,
    health_mid2: Object<'obj>,
    health_mid3: Object<'obj>,
    health_end: Object<'obj>,
}

impl<'obj> HealthBar<'obj> {
    pub fn new(object: &'obj OamManaged<'_>, start_x: i32, start_y: i32) -> Self {
        let mut health_letters = object.object_sprite(HEALTH_BAR_LETTERS.sprite(0));
        let mut health_start = object.object_sprite(HEALTH_BAR_START_CONT.sprite(0));
        let mut health_mid1 = object.object_sprite(HEALTH_CONT.sprite(0));
        let mut health_mid2 = object.object_sprite(HEALTH_CONT.sprite(0));
        let mut health_mid3 = object.object_sprite(HEALTH_CONT.sprite(0));
        let mut health_end = object.object_sprite(HEALTH_BAR_END.sprite(0));

        health_letters.show();
        health_start.show();
        health_mid1.show();
        health_mid2.show();
        health_mid3.show();
        health_end.show();

        let mut health_bar = Self {
            health_letters,
            health_start,
            health_mid1,
            health_mid2,
            health_mid3,
            health_end,
        };

        health_bar.set_position(start_x, start_y);

        health_bar
    }

    fn set_position(&mut self, x: i32, y: i32) {
        // new! use of the `set_position` method. This is a helper feature using agb's vector types.
        self.health_letters.set_position((x, y));
        // Start is 4px wide
        self.health_start.set_position((x+8, y));

        self.health_mid1.set_position((x+12, y));
        self.health_mid2.set_position((x+20, y));
        self.health_mid3.set_position((x+28, y));

        // End goes at a fixed distance from start
        self.health_end.set_position((x+36, y));
    }
}