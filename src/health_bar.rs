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

static MID_MT_SPRITE: &Sprite = HEALTH_MID_EMPTY.sprite(0);
static MID_FILL_SPRITE: &Sprite = HEALTH_MID.sprite(0);

pub struct HealthBar<'obj> {
    health_letters: Object<'obj>,
    health_start: Object<'obj>,
    health_end: Object<'obj>,
    middle_healths: Vec<Object<'obj>>
}

impl<'obj> HealthBar<'obj> {
    pub fn new(object: &'obj OamManaged<'_>, start_x: i32, start_y: i32) -> Self {
        let mut health_letters = object.object_sprite(HEALTH_BAR_LETTERS.sprite(0));
        let mut health_start = object.object_sprite(HEALTH_BAR_START_CONT.sprite(0));
        let mut health_end = object.object_sprite(HEALTH_BAR_END.sprite(0));

        health_letters.show();
        health_start.show();
        health_end.show();

        let mut middle_healths: Vec<Object> = Vec::new();
        for _ in 0..38{
            let mut tmp = object.object_sprite(HEALTH_MID.sprite(0));
            tmp.show();
            middle_healths.push(tmp);
        }

        let mut health_bar = Self {
            health_letters,
            health_start,
            health_end,
            middle_healths
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
        let mut cnt = x + 12;
        for o in &mut self.middle_healths{
            o.set_position((cnt, y));
            cnt += 1;
        }

        // End goes at a fixed distance from start
        self.health_end.set_position((x+40, y)); // 80
    }
}