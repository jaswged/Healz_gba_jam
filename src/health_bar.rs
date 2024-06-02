// change our imports to include what we will use
use alloc::vec::Vec;
use agb::{display::object::{Graphics, Object, OamManaged, Tag}};
use agb::display::Priority;
use agb::display::tiled::{RegularBackgroundSize, Tiled0, VRamManager};
use agb::display::object::Sprite;

use crate::game_manager::GRAPHICS;

// static HEALTH_BAR_LETTERS: &Tag = GRAPHICS.tags().get("hp_begin");
// static HEALTH_BAR_START: &Tag = GRAPHICS.tags().get("hp_start");
// static HEALTH_BAR_START_CONT: &Tag = GRAPHICS.tags().get("hp_start_cont");
// static HEALTH_CONT: &Tag = GRAPHICS.tags().get("hp_cont");
// static HEALTH_MID: &Tag = GRAPHICS.tags().get("hp_filled");
// static HEALTH_MID_EMPTY: &Tag = GRAPHICS.tags().get("hp_empty");
// static HEALTH_BAR_END: &Tag = GRAPHICS.tags().get("hp_end");

static HP_1_SPRITE: &Sprite = GRAPHICS.tags().get("hp_1").sprite(0);
static HP_2_SPRITE: &Sprite = GRAPHICS.tags().get("hp_2").sprite(0);
static HP_3_SPRITE: &Sprite = GRAPHICS.tags().get("hp_3").sprite(0);
static HP_4_SPRITE: &Sprite = GRAPHICS.tags().get("hp_4").sprite(0);
static HP_5_SPRITE: &Sprite = GRAPHICS.tags().get("hp_5").sprite(0);
static HP_6_SPRITE: &Sprite = GRAPHICS.tags().get("hp_6").sprite(0);
static HP_7_SPRITE: &Sprite = GRAPHICS.tags().get("hp_7").sprite(0);
static HP_8_SPRITE: &Sprite = GRAPHICS.tags().get("hp_8").sprite(0);


// redo tags
// static HP_LETTERS: &Tag = GRAPHICS.tags().get("hp_letters");
// static HP_LETTERS_END: &Tag = GRAPHICS.tags().get("hp_letters_end");
// static HP_FRAME: &Tag = GRAPHICS.tags().get("hp_frame");
// static HP_END: &Tag = GRAPHICS.tags().get("hp_end");
// static HP_INFILL: &Tag = GRAPHICS.tags().get("hp_infill");

// static MID_MT_SPRITE: &Sprite = HEALTH_MID_EMPTY.sprite(0);
// static MID_FILL_SPRITE: &Sprite = HEALTH_MID.sprite(0);
// todo other 7 sprites references

pub struct HealthBar<'obj> {
    health_amt: i32,
    health_mid1: Object<'obj>,
    health_mid2: Object<'obj>,
    health_mid3: Object<'obj>,
    health_mid4: Object<'obj>,
    health_end: Object<'obj>,
}

impl<'obj> HealthBar<'obj> {
    pub fn new(object: &'obj OamManaged<'_>, start_x: i32, start_y: i32) -> Self {
        let mut health_amt = 35;
        let mut health_mid1 = object.object_sprite(HP_8_SPRITE);
        let mut health_mid2 = object.object_sprite(HP_8_SPRITE);
        let mut health_mid3 = object.object_sprite(HP_8_SPRITE);
        let mut health_mid4 = object.object_sprite(HP_8_SPRITE);
        let mut health_end = object.object_sprite(HP_3_SPRITE); // todo how many do i start with here

        health_mid1.show();
        health_mid2.show();
        health_mid3.show();
        health_mid4.show();
        health_end.show();

        let mut health_bar = Self {
            health_amt,
            health_mid1,
            health_mid2,
            health_mid3,
            health_mid4,
            health_end,
        };

        health_bar.set_position(start_x, start_y);

        health_bar
    }

    fn set_position(&mut self, x: i32, y: i32) {
        // new! use of the `set_position` method. This is a helper feature using agb's vector types.
        self.health_mid1.set_position((x, y));
        self.health_mid2.set_position((x+8, y));
        self.health_mid3.set_position((x+16, y));
        self.health_mid4.set_position((x+24, y));
        self.health_end.set_position((x+32, y));
    }

    // pub fn take_damage(&mut self, damage: usize){
    //     println!("Took {} damage!", damage);
    //     // todo here jason
    //     if damage >= self.health_amt {
    //         println!("Is Dead!");
    //         self.health_amt = 0;
    //         // todo what now? How to trigger game over?
    //         return
    //     }
    //     self.health_amt -= damage;

    //     println!("Current health is: {}", self.health_amt);
    //     self.update_bar(damage);
    // }

    // fn update_bar(&mut self, damage: usize) {
    //     // currently decrement only!
    //     println!("\nUpdate health bar. Ptr at {}", self.heath_ptr);
    //     // todo here jason
    // }
}