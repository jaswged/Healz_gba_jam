use agb::display::object::{Object, OamManaged, Sprite};
use agb::println;

use crate::game_manager::GRAPHICS;

static HP_1_SPRITE: &Sprite = GRAPHICS.tags().get("hp_1").sprite(0);
static HP_2_SPRITE: &Sprite = GRAPHICS.tags().get("hp_2").sprite(0);
static HP_3_SPRITE: &Sprite = GRAPHICS.tags().get("hp_3").sprite(0);
static HP_4_SPRITE: &Sprite = GRAPHICS.tags().get("hp_4").sprite(0);
static HP_5_SPRITE: &Sprite = GRAPHICS.tags().get("hp_5").sprite(0);
static HP_6_SPRITE: &Sprite = GRAPHICS.tags().get("hp_6").sprite(0);
static HP_7_SPRITE: &Sprite = GRAPHICS.tags().get("hp_7").sprite(0);
static HP_8_SPRITE: &Sprite = GRAPHICS.tags().get("hp_8").sprite(0);
static HP_SPRITE_ARR: [&Sprite; 8] = [
    HP_8_SPRITE,
    HP_7_SPRITE,
    HP_6_SPRITE,
    HP_5_SPRITE,
    HP_4_SPRITE,
    HP_3_SPRITE,
    HP_2_SPRITE,
    HP_1_SPRITE,
];


pub struct HealthBar<'obj> {
    pub health_amt: usize,
    health_mid1: Object<'obj>,
    health_mid2: Object<'obj>,
    health_mid3: Object<'obj>,
    health_mid4: Object<'obj>,
    health_end: Object<'obj>,
    object: &'obj OamManaged<'obj>,
    pub health_max: usize,
}

impl<'obj> HealthBar<'obj> {
    pub fn new(object: &'obj OamManaged<'obj>, start_x: i32, start_y: i32) -> Self {
        let health_amt = 35; // 32 for easy math. set back to 35 if figure out a solution
        let filled = HP_SPRITE_ARR[0];
        let health_mid1 = object.object_sprite(filled);
        let health_mid2 = object.object_sprite(filled);
        let health_mid3 = object.object_sprite(filled);
        let health_mid4 = object.object_sprite(filled);
        let health_end = object.object_sprite(HP_3_SPRITE);

        let mut health_bar = Self {
            health_amt,
            health_mid1,
            health_mid2,
            health_mid3,
            health_mid4,
            health_end,
            object,
            health_max: health_amt,
        };

        health_bar.show_all();

        health_bar.set_position(start_x, start_y);

        health_bar
    }

    fn set_position(&mut self, x: i32, y: i32) {
        // new! use of the `set_position` method. This is a helper feature using agb's vector types.
        self.health_mid1.set_position((x, y));
        self.health_mid2.set_position((x + 8, y));
        self.health_mid3.set_position((x + 16, y));
        self.health_mid4.set_position((x + 24, y));
        self.health_end.set_position((x + 32, y));
    }

    pub fn update_bar(&mut self, new_health: usize) {
        // Match on ranges
        // first = 0..=8;
        // second = 9..=16;
        // third = 17..=24;
        // fourth = 25..=32;
        // last 33..=35
        match (self.health_amt, new_health) {
            // Both are first sprite
            (0..=8, 0..=8) => {
                let new_sprite = HP_SPRITE_ARR[8 - new_health];
                self.health_mid1.set_sprite(self.object.sprite(new_sprite));
            }
            // Old is 1st, New is 2nd
            (0..=8, 9..=16) => {
                // show full for old.
                self.health_mid1.set_sprite(self.object.sprite(HP_SPRITE_ARR[0]));
                // Update new
                self.health_mid2.show();
                let new_sprite = HP_SPRITE_ARR[16 - new_health];
                self.health_mid2.set_sprite(self.object.sprite(new_sprite));
            }
            // Old is 2nd, New is 1st,
            (9..=16, 0..=8) => {
                self.health_mid2.hide();
                let new_sprite = HP_SPRITE_ARR[8 - new_health];
                self.health_mid1.set_sprite(self.object.sprite(new_sprite));
            }
            // Both are second sprite
            (9..=16, 9..=16) => {
                let new_sprite = HP_SPRITE_ARR[16 - new_health];
                self.health_mid2.set_sprite(self.object.sprite(new_sprite));
            }
            // Old is 3rd, New is 2nd
            (17..=24, 9..=16) => {
                self.health_mid3.hide();
                let new_sprite = HP_SPRITE_ARR[16 - new_health];
                self.health_mid2.set_sprite(self.object.sprite(new_sprite));
            }
            // Old is 2nd, New is 3rd
            (9..=16, 17..=24) => {
                self.health_mid2.set_sprite(self.object.sprite(HP_SPRITE_ARR[0]));

                self.health_mid3.show();
                let new_sprite = HP_SPRITE_ARR[24 - new_health];
                self.health_mid3.set_sprite(self.object.sprite(new_sprite));
            }
            // Both are 3rd sprite
            (17..=24, 17..=24) => {
                let new_sprite = HP_SPRITE_ARR[24 - new_health];
                self.health_mid3.set_sprite(self.object.sprite(new_sprite));
            }
            // Old is 4th, New is 3rd
            (25..=32, 17..=24) => {
                self.health_mid4.hide();
                let new_sprite = HP_SPRITE_ARR[24 - new_health];
                self.health_mid3.set_sprite(self.object.sprite(new_sprite));
            }
            // Old is 3rd, New is 4th
            (17..=24, 25..=32) => {
                self.health_mid3.set_sprite(self.object.sprite(HP_SPRITE_ARR[0]));

                self.health_mid4.show();
                let new_sprite = HP_SPRITE_ARR[32 - new_health];
                self.health_mid4.set_sprite(self.object.sprite(new_sprite));
            }
            // Both are 4th sprite
            (25..=32, 25..=32) => {
                let new_sprite = HP_SPRITE_ARR[32 - new_health];
                self.health_mid4.set_sprite(self.object.sprite(new_sprite));
            }
            // Both are last sprite
            (33.., 33..) => {
                let new_sprite = HP_SPRITE_ARR[40 - new_health];
                self.health_end.set_sprite(self.object.sprite(new_sprite));
            }
            // Old is last, new is 4th
            (33.., 25..=32) => {
                self.health_end.hide();
                let new_sprite = HP_SPRITE_ARR[32 - new_health];
                self.health_mid4.set_sprite(self.object.sprite(new_sprite));
            }
            // old is 4th, new is last
            (25..=32, 33..) => {
                self.health_mid4.set_sprite(self.object.sprite(HP_SPRITE_ARR[0]));

                self.health_end.show();
                let new_sprite = HP_SPRITE_ARR[40 - new_health];
                self.health_end.set_sprite(self.object.sprite(new_sprite));
            }
            _ => {
                println!("TODO: Implement the cases where the start and end blocks arent the same");
            }
        };

        self.health_amt = new_health;
    }

    pub fn hide_mid1(&mut self) {
        self.health_mid1.hide();
    }

    pub fn hide_all(&mut self) {
        self.health_mid1.hide();
        self.health_mid2.hide();
        self.health_mid3.hide();
        self.health_mid4.hide();
        self.health_end.hide();
    }
    pub fn show_all(&mut self) {
        self.health_mid1.show();
        self.health_mid2.show();
        self.health_mid3.show();
        self.health_mid4.show();
        self.health_end.show();
    }
}