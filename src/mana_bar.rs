use agb::display::object::{Object, OamManaged, Sprite};
use agb::println;

use crate::game_manager::GRAPHICS;

// region Sprites
static MT_SPRITE: &Sprite = GRAPHICS.tags().get("mt").sprite(0);

static MN_1_SPRITE: &Sprite = GRAPHICS.tags().get("mn_1").sprite(0);
static MN_2_SPRITE: &Sprite = GRAPHICS.tags().get("mn_2").sprite(0);
static MN_3_SPRITE: &Sprite = GRAPHICS.tags().get("mn_3").sprite(0);
static MN_4_SPRITE: &Sprite = GRAPHICS.tags().get("mn_4").sprite(0);
static MN_5_SPRITE: &Sprite = GRAPHICS.tags().get("mn_5").sprite(0);
static MN_6_SPRITE: &Sprite = GRAPHICS.tags().get("mn_6").sprite(0);
static MN_7_SPRITE: &Sprite = GRAPHICS.tags().get("mn_7").sprite(0);
static MN_8_SPRITE: &Sprite = GRAPHICS.tags().get("mn_8").sprite(0);
static MN_SPRITE_ARR: [&Sprite; 9] = [
    MN_8_SPRITE,
    MN_7_SPRITE,
    MN_6_SPRITE,
    MN_5_SPRITE,
    MN_4_SPRITE,
    MN_3_SPRITE,
    MN_2_SPRITE,
    MN_1_SPRITE,
    MT_SPRITE,
];

static HP_1_SPRITE: &Sprite = GRAPHICS.tags().get("hp_1").sprite(0);
static HP_2_SPRITE: &Sprite = GRAPHICS.tags().get("hp_2").sprite(0);
static HP_3_SPRITE: &Sprite = GRAPHICS.tags().get("hp_3").sprite(0);
static HP_4_SPRITE: &Sprite = GRAPHICS.tags().get("hp_4").sprite(0);
static HP_5_SPRITE: &Sprite = GRAPHICS.tags().get("hp_5").sprite(0);
static HP_6_SPRITE: &Sprite = GRAPHICS.tags().get("hp_6").sprite(0);
static HP_7_SPRITE: &Sprite = GRAPHICS.tags().get("hp_7").sprite(0);
static HP_8_SPRITE: &Sprite = GRAPHICS.tags().get("hp_8").sprite(0);
static HP_SPRITE_ARR: [&Sprite; 9] = [
    HP_8_SPRITE,
    HP_7_SPRITE,
    HP_6_SPRITE,
    HP_5_SPRITE,
    HP_4_SPRITE,
    HP_3_SPRITE,
    HP_2_SPRITE,
    HP_1_SPRITE,
    MT_SPRITE,
];

static CD_1_SPRITE: &Sprite = GRAPHICS.tags().get("cd_1").sprite(0);
static CD_2_SPRITE: &Sprite = GRAPHICS.tags().get("cd_2").sprite(0);
static CD_3_SPRITE: &Sprite = GRAPHICS.tags().get("cd_3").sprite(0);
static CD_4_SPRITE: &Sprite = GRAPHICS.tags().get("cd_4").sprite(0);
static CD_5_SPRITE: &Sprite = GRAPHICS.tags().get("cd_5").sprite(0);
static CD_6_SPRITE: &Sprite = GRAPHICS.tags().get("cd_6").sprite(0);
static CD_7_SPRITE: &Sprite = GRAPHICS.tags().get("cd_7").sprite(0);
static CD_8_SPRITE: &Sprite = GRAPHICS.tags().get("cd_8").sprite(0);
static CD_SPRITE_ARR: [&Sprite; 9] = [
    CD_8_SPRITE,
    CD_7_SPRITE,
    CD_6_SPRITE,
    CD_5_SPRITE,
    CD_4_SPRITE,
    CD_3_SPRITE,
    CD_2_SPRITE,
    CD_1_SPRITE,
    MT_SPRITE,
];
// endregion

pub enum BarType{
    Health,
    Mana,
    Cooldown
}
pub struct Bar<'obj> {
    bar_type: BarType,
    pub bar_amt: usize,
    mid1: Object<'obj>,
    mid2: Object<'obj>,
    mid3: Object<'obj>,
    mid4: Object<'obj>,
    end: Object<'obj>,
    object: &'obj OamManaged<'obj>,
    pub bar_max: usize,
}

impl<'obj> Bar<'obj> {
    pub fn new(object: &'obj OamManaged<'obj>, bar_type: BarType, start_x: i32, start_y: i32) -> Self {
        let bar_amt = 35;
        let arr = match bar_type {
            BarType::Mana => {MN_SPRITE_ARR}
            BarType::Cooldown => {CD_SPRITE_ARR}
            BarType::Health => {HP_SPRITE_ARR}
        };
        let filled = arr[0];
        let mid1 = object.object_sprite(filled);
        let mid2 = object.object_sprite(filled);
        let mid3 = object.object_sprite(filled);
        let mid4 = object.object_sprite(filled);
        let end = object.object_sprite(arr[5]);

        let mut mana_bar = Self {
            bar_type,
            bar_amt,
            mid1,
            mid2,
            mid3,
            mid4,
            end,
            object,
            bar_max: bar_amt,
        };

        mana_bar.show_all();

        mana_bar.set_position(start_x, start_y);

        mana_bar
    }

    fn set_position(&mut self, x: i32, y: i32) {
        // new! use of the `set_position` method. This is a helper feature using agb's vector types.
        self.mid1.set_position((x, y));
        self.mid2.set_position((x + 8, y));
        self.mid3.set_position((x + 16, y));
        self.mid4.set_position((x + 24, y));
        self.end.set_position((x + 32, y));
    }

    pub fn update_bar(&mut self, new_amount: usize) {
        let arr = match self.bar_type {
            BarType::Mana => {MN_SPRITE_ARR}
            BarType::Cooldown => {CD_SPRITE_ARR}
            BarType::Health => {HP_SPRITE_ARR}
        };
        // Match on ranges
        // first = 0..=8;
        // second = 9..=16;
        // third = 17..=24;
        // fourth = 25..=32;
        // last 33..=35
        match (self.bar_amt, new_amount) {
            // Both are first sprite
            (0..=8, 0..=8) => {
                let new_sprite = arr[8 - new_amount];
                self.mid1.set_sprite(self.object.sprite(new_sprite));
            }
            // Old is 1st, New is 2nd
            (0..=8, 9..=16) => {
                // show full for old.
                self.mid1.set_sprite(self.object.sprite(arr[0]));
                // Update new
                self.mid2.show();
                let new_sprite = arr[16 - new_amount];
                self.mid2.set_sprite(self.object.sprite(new_sprite));
            }
            // Old is 2nd, New is 1st,
            (9..=16, 0..=8) => {
                self.mid2.hide();
                let new_sprite = arr[8 - new_amount];
                self.mid1.set_sprite(self.object.sprite(new_sprite));
            }
            // Both are second sprite
            (9..=16, 9..=16) => {
                let new_sprite = arr[16 - new_amount];
                self.mid2.set_sprite(self.object.sprite(new_sprite));
            }
            // Old is 3rd, New is 2nd
            (17..=24, 9..=16) => {
                self.mid3.hide();
                let new_sprite = arr[16 - new_amount];
                self.mid2.set_sprite(self.object.sprite(new_sprite));
            }
            // Old is 2nd, New is 3rd
            (9..=16, 17..=24) => {
                self.mid2.set_sprite(self.object.sprite(arr[0]));

                self.mid3.show();
                let new_sprite = arr[24 - new_amount];
                self.mid3.set_sprite(self.object.sprite(new_sprite));
            }
            // Both are 3rd sprite
            (17..=24, 17..=24) => {
                let new_sprite = arr[24 - new_amount];
                self.mid3.set_sprite(self.object.sprite(new_sprite));
            }
            // Old is 4th, New is 3rd
            (25..=32, 17..=24) => {
                self.mid4.hide();
                let new_sprite = arr[24 - new_amount];
                self.mid3.set_sprite(self.object.sprite(new_sprite));
            }
            // Old is 3rd, New is 4th
            (17..=24, 25..=32) => {
                self.mid3.set_sprite(self.object.sprite(arr[0]));

                self.mid4.show();
                let new_sprite = arr[32 - new_amount];
                self.mid4.set_sprite(self.object.sprite(new_sprite));
            }
            // Both are 4th sprite
            (25..=32, 25..=32) => {
                let new_sprite = arr[32 - new_amount];
                self.mid4.set_sprite(self.object.sprite(new_sprite));
            }
            // Both are last sprite
            (33.., 33..) => {
                let new_sprite = arr[40 - new_amount];
                self.end.set_sprite(self.object.sprite(new_sprite));
            }
            // Old is last, new is 4th
            (33.., 25..=32) => {
                self.end.hide();
                let new_sprite = arr[32 - new_amount];
                self.mid4.set_sprite(self.object.sprite(new_sprite));
            }
            // old is 4th, new is last
            (25..=32, 33..) => {
                self.mid4.set_sprite(self.object.sprite(arr[0]));

                self.end.show();
                let new_sprite = arr[40 - new_amount];
                self.end.set_sprite(self.object.sprite(new_sprite));
            }
            _ => {
                println!("TODO: Implement the cases where the start and end blocks arent the same");
            }
        };

        self.bar_amt = new_amount;
    }

    pub fn lose_amount(&mut self, spent: usize){
        let new_amount = self.bar_amt - spent;

        self.update_bar(new_amount);
    }

    pub fn gain_amount(&mut self, gained: usize){
        let mut new_amount = self.bar_amt + gained;
        if new_amount >= self.bar_max {
            self.bar_amt = self.bar_max;
            new_amount = self.bar_max;
        }

        self.update_bar(new_amount);
    }

    pub fn hide_mana_mid1(&mut self) {
        self.mid1.hide();
    }

    pub fn hide_all(&mut self) {
        self.mid1.hide();
        self.mid2.hide();
        self.mid3.hide();
        self.mid4.hide();
        self.end.hide();
    }
    pub fn show_all(&mut self) {
        self.mid1.show();
        self.mid2.show();
        self.mid3.show();
        self.mid4.show();
        self.end.show();
    }
}