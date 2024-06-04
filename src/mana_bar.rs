use agb::display::object::{Object, OamManaged, Sprite};
use agb::println;

use crate::game_manager::GRAPHICS;

static MN_1_SPRITE: &Sprite = GRAPHICS.tags().get("mn_1").sprite(0);
static MN_2_SPRITE: &Sprite = GRAPHICS.tags().get("mn_2").sprite(0);
static MN_3_SPRITE: &Sprite = GRAPHICS.tags().get("mn_3").sprite(0);
static MN_4_SPRITE: &Sprite = GRAPHICS.tags().get("mn_4").sprite(0);
static MN_5_SPRITE: &Sprite = GRAPHICS.tags().get("mn_5").sprite(0);
static MN_6_SPRITE: &Sprite = GRAPHICS.tags().get("mn_6").sprite(0);
static MN_7_SPRITE: &Sprite = GRAPHICS.tags().get("mn_7").sprite(0);
static MN_8_SPRITE: &Sprite = GRAPHICS.tags().get("mn_8").sprite(0);
static MN_SPRITE_ARR: [&Sprite; 8] = [
    MN_8_SPRITE,
    MN_7_SPRITE,
    MN_6_SPRITE,
    MN_5_SPRITE,
    MN_4_SPRITE,
    MN_3_SPRITE,
    MN_2_SPRITE,
    MN_1_SPRITE,
];

pub struct ManaBar<'obj> {
    pub mana_amt: usize,
    mana_mid1: Object<'obj>,
    mana_mid2: Object<'obj>,
    mana_mid3: Object<'obj>,
    mana_mid4: Object<'obj>,
    mana_end: Object<'obj>,
    object: &'obj OamManaged<'obj>,
    pub mana_max: usize,
}

impl<'obj> ManaBar<'obj> {
    pub fn new(object: &'obj OamManaged<'obj>, start_x: i32, start_y: i32) -> Self {
        let mana_amt = 35;
        let filled = MN_SPRITE_ARR[0];
        let mana_mid1 = object.object_sprite(filled);
        let mana_mid2 = object.object_sprite(filled);
        let mana_mid3 = object.object_sprite(filled);
        let mana_mid4 = object.object_sprite(filled);
        let mana_end = object.object_sprite(MN_3_SPRITE);

        let mut mana_bar = Self {
            mana_amt,
            mana_mid1,
            mana_mid2,
            mana_mid3,
            mana_mid4,
            mana_end,
            object,
            mana_max: mana_amt,
        };

        mana_bar.show_all();

        mana_bar.set_position(start_x, start_y);

        mana_bar
    }

    fn set_position(&mut self, x: i32, y: i32) {
        // new! use of the `set_position` method. This is a helper feature using agb's vector types.
        self.mana_mid1.set_position((x, y));
        self.mana_mid2.set_position((x + 8, y));
        self.mana_mid3.set_position((x + 16, y));
        self.mana_mid4.set_position((x + 24, y));
        self.mana_end.set_position((x + 32, y));
    }

    pub fn update_bar(&mut self, new_mana: usize) {
        // Match on ranges
        // first = 0..=8;
        // second = 9..=16;
        // third = 17..=24;
        // fourth = 25..=32;
        // last 33..=35
        match (self.mana_amt, new_mana) {
            // Both are first sprite
            (0..=8, 0..=8) => {
                let new_sprite = MN_SPRITE_ARR[8 - new_mana];
                self.mana_mid1.set_sprite(self.object.sprite(new_sprite));
            }
            // Old is 1st, New is 2nd
            (0..=8, 9..=16) => {
                // show full for old.
                self.mana_mid1.set_sprite(self.object.sprite(MN_SPRITE_ARR[0]));
                // Update new
                self.mana_mid2.show();
                let new_sprite = MN_SPRITE_ARR[16 - new_mana];
                self.mana_mid2.set_sprite(self.object.sprite(new_sprite));
            }
            // Old is 2nd, New is 1st,
            (9..=16, 0..=8) => {
                self.mana_mid2.hide();
                let new_sprite = MN_SPRITE_ARR[8 - new_mana];
                self.mana_mid1.set_sprite(self.object.sprite(new_sprite));
            }
            // Both are second sprite
            (9..=16, 9..=16) => {
                let new_sprite = MN_SPRITE_ARR[16 - new_mana];
                self.mana_mid2.set_sprite(self.object.sprite(new_sprite));
            }
            // Old is 3rd, New is 2nd
            (17..=24, 9..=16) => {
                self.mana_mid3.hide();
                let new_sprite = MN_SPRITE_ARR[16 - new_mana];
                self.mana_mid2.set_sprite(self.object.sprite(new_sprite));
            }
            // Old is 2nd, New is 3rd
            (9..=16, 17..=24) => {
                self.mana_mid2.set_sprite(self.object.sprite(MN_SPRITE_ARR[0]));

                self.mana_mid3.show();
                let new_sprite = MN_SPRITE_ARR[24 - new_mana];
                self.mana_mid3.set_sprite(self.object.sprite(new_sprite));
            }
            // Both are 3rd sprite
            (17..=24, 17..=24) => {
                let new_sprite = MN_SPRITE_ARR[24 - new_mana];
                self.mana_mid3.set_sprite(self.object.sprite(new_sprite));
            }
            // Old is 4th, New is 3rd
            (25..=32, 17..=24) => {
                self.mana_mid4.hide();
                let new_sprite = MN_SPRITE_ARR[24 - new_mana];
                self.mana_mid3.set_sprite(self.object.sprite(new_sprite));
            }
            // Old is 3rd, New is 4th
            (17..=24, 25..=32) => {
                self.mana_mid3.set_sprite(self.object.sprite(MN_SPRITE_ARR[0]));

                self.mana_mid4.show();
                let new_sprite = MN_SPRITE_ARR[32 - new_mana];
                self.mana_mid4.set_sprite(self.object.sprite(new_sprite));
            }
            // Both are 4th sprite
            (25..=32, 25..=32) => {
                let new_sprite = MN_SPRITE_ARR[32 - new_mana];
                self.mana_mid4.set_sprite(self.object.sprite(new_sprite));
            }
            // Both are last sprite
            (33.., 33..) => {
                let new_sprite = MN_SPRITE_ARR[40 - new_mana];
                self.mana_end.set_sprite(self.object.sprite(new_sprite));
            }
            // Old is last, new is 4th
            (33.., 25..=32) => {
                self.mana_end.hide();
                let new_sprite = MN_SPRITE_ARR[32 - new_mana];
                self.mana_mid4.set_sprite(self.object.sprite(new_sprite));
            }
            // old is 4th, new is last
            (25..=32, 33..) => {
                self.mana_mid4.set_sprite(self.object.sprite(MN_SPRITE_ARR[0]));

                self.mana_end.show();
                let new_sprite = MN_SPRITE_ARR[40 - new_mana];
                self.mana_end.set_sprite(self.object.sprite(new_sprite));
            }
            _ => {
                println!("TODO: Implement the cases where the start and end blocks arent the same");
            }
        };

        self.mana_amt = new_mana;
    }

    pub fn spend_mana(&mut self, spent: usize){
        println!("Spent {} mana!", spent);
        let new_mana = self.mana_amt - spent;

        self.update_bar(new_mana);
    }

    pub fn recover_mana(&mut self, gained: usize){
        println!("Gained {} mana!", gained);
        let new_mana = self.mana_amt + gained;

        let mut new_mana = self.mana_amt + gained;
        if new_mana >= self.mana_max {
            println!("Is full mana!");
            self.mana_amt = self.mana_max;
            new_mana = self.mana_max;
        }

        self.update_bar(new_mana);
    }

    pub fn hide_mana_mid1(&mut self) {
        self.mana_mid1.hide();
    }

    pub fn hide_all(&mut self) {
        self.mana_mid1.hide();
        self.mana_mid2.hide();
        self.mana_mid3.hide();
        self.mana_mid4.hide();
        self.mana_end.hide();
    }
    pub fn show_all(&mut self) {
        self.mana_mid1.show();
        self.mana_mid2.show();
        self.mana_mid3.show();
        self.mana_mid4.show();
        self.mana_end.show();
    }
}