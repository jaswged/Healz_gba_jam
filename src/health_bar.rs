// change our imports to include what we will use
use agb::{
    display::object::{Graphics, Object, OamManaged, Tag}
};

use crate::game_manager::GRAPHICS;

static HEALTH_BAR_LETTERS: &Tag = GRAPHICS.tags().get("hp_begin");
static HEALTH_BAR_START: &Tag = GRAPHICS.tags().get("hp_start");
static HEALTH_BAR_START_CONT: &Tag = GRAPHICS.tags().get("hp_start_cont");
static HEALTH_CONT: &Tag = GRAPHICS.tags().get("hp_cont");
static HEALTH_MID: &Tag = GRAPHICS.tags().get("hp_filled");
static HEALTH_MID_EMPTY: &Tag = GRAPHICS.tags().get("hp_empty");
static HEALTH_BAR_END: &Tag = GRAPHICS.tags().get("hp_end");

pub struct HealthBar<'obj> {
    health_letters: Object<'obj>,
    health_start: Object<'obj>,
    health_mid1: Object<'obj>,
    health_mid2: Object<'obj>,
    health_mid3: Object<'obj>,
    // health_mt: Object<'obj>,
    health_end: Object<'obj>,
}

impl<'obj> HealthBar<'obj> {
    pub fn new(object: &'obj OamManaged<'_>, start_x: i32, start_y: i32) -> Self {
        let mut health_letters = object.object_sprite(HEALTH_BAR_LETTERS.sprite(0));
        let mut health_start = object.object_sprite(HEALTH_BAR_START_CONT.sprite(0));
        let mut health_mid1 = object.object_sprite(HEALTH_CONT.sprite(0));
        let mut health_mid2 = object.object_sprite(HEALTH_CONT.sprite(0));
        let mut health_mid3 = object.object_sprite(HEALTH_CONT.sprite(0));
        // let mut health_mid = object.object_sprite(HEALTH_MID.sprite(0));
        // let mut health_mt = object.object_sprite(HEALTH_MID_EMPTY.sprite(0));
        let mut health_end = object.object_sprite(HEALTH_BAR_END.sprite(0));

        health_letters.show();
        health_start.show();
        health_mid1.show();
        health_mid2.show();
        health_mid3.show();
        // health_mt.show();
        health_end.show();

        let mut health_bar = Self {
            health_letters,
            health_start,
            health_mid1,
            health_mid2,
            health_mid3,
            // health_mt,
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

        // Each mid section is 1px
        /* todo the spacing between and duplication logic
        let mut to_return = "#".repeat(self.health as usize);
        let remaining = self.max_health - self.health;
        if remaining != 0{
            let a = "_".repeat(remaining as usize);
            to_return.push_str(&a);
        }
         */
        self.health_mid1.set_position((x+16, y));
        self.health_mid2.set_position((x+24, y));
        self.health_mid3.set_position((x+32, y));
        // self.health_mt.set_position((x+14, y));

        // End goes at a fixed distance from start
        self.health_end.set_position((x+40, y)); // 80
    }
}