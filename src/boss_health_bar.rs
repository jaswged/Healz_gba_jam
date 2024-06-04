use agb::{display::object::{Object, OamManaged}};
use agb::display::object::Sprite;

use crate::game_manager::GRAPHICS;

static HP_1_SPRITE: &Sprite = GRAPHICS.tags().get("bhp_1").sprite(0);
static HP_2_SPRITE: &Sprite = GRAPHICS.tags().get("bhp_2").sprite(0);
static HP_3_SPRITE: &Sprite = GRAPHICS.tags().get("bhp_3").sprite(0);
static HP_4_SPRITE: &Sprite = GRAPHICS.tags().get("bhp_4").sprite(0);
static HP_5_SPRITE: &Sprite = GRAPHICS.tags().get("bhp_5").sprite(0);
static HP_6_SPRITE: &Sprite = GRAPHICS.tags().get("bhp_6").sprite(0);
static HP_7_SPRITE: &Sprite = GRAPHICS.tags().get("bhp_7").sprite(0);
static HP_8_SPRITE: &Sprite = GRAPHICS.tags().get("bhp_8").sprite(0);
static HP_9_SPRITE: &Sprite = GRAPHICS.tags().get("bhp_9").sprite(0);
static HP_10_SPRITE: &Sprite = GRAPHICS.tags().get("bhp_10").sprite(0);
static HP_11_SPRITE: &Sprite = GRAPHICS.tags().get("bhp_11").sprite(0);
static HP_12_SPRITE: &Sprite = GRAPHICS.tags().get("bhp_12").sprite(0);
static HP_13_SPRITE: &Sprite = GRAPHICS.tags().get("bhp_13").sprite(0);
static HP_14_SPRITE: &Sprite = GRAPHICS.tags().get("bhp_14").sprite(0);
static HP_15_SPRITE: &Sprite = GRAPHICS.tags().get("bhp_15").sprite(0);
static HP_16_SPRITE: &Sprite = GRAPHICS.tags().get("bhp_16").sprite(0);
static HP_SPRITE_ARR: [&Sprite; 16] = [HP_16_SPRITE,HP_15_SPRITE,HP_14_SPRITE,HP_13_SPRITE,
    HP_12_SPRITE,HP_11_SPRITE,HP_10_SPRITE,HP_9_SPRITE,
    HP_8_SPRITE,HP_7_SPRITE,HP_6_SPRITE,HP_5_SPRITE,
    HP_4_SPRITE,HP_3_SPRITE,HP_2_SPRITE,HP_1_SPRITE];

pub struct BossHealthBar<'obj> {
    pub health_amt: usize,
    health_mid1: Object<'obj>,
    health_mid2: Object<'obj>,
    health_mid3: Object<'obj>,
    health_end: Object<'obj>,
    object: &'obj OamManaged<'obj>
}

impl<'obj> BossHealthBar<'obj> {
    //! Boss health bar takes up 9 tiles or 72 pixels
    //! The skull beginning portion takes 20 and the end is 2px
    //! This leaves a perfect 50 pixels for boss health of 100 at 2 hp per!
    pub fn new(object: &'obj OamManaged<'obj>, start_x: i32, start_y: i32) -> Self {
        let health_amt = 50;
        let filled = HP_SPRITE_ARR[0];

        let mut health_mid1 = object.object_sprite(filled);
        let mut health_mid2 = object.object_sprite(filled);
        let mut health_mid3 = object.object_sprite(filled);
        let mut health_end = object.object_sprite(HP_2_SPRITE);

        health_mid1.show();
        health_mid2.show();
        health_mid3.show();
        health_end.show();

        let mut b_health_bar = Self {
            health_amt,
            health_mid1,
            health_mid2,
            health_mid3,
            health_end,
            object,
        };

        b_health_bar.set_position(start_x, start_y);

        b_health_bar
    }

    fn set_position(&mut self, x: i32, y: i32) {
        self.health_mid1.set_position((x, y));
        self.health_mid2.set_position((x+16, y));
        self.health_mid3.set_position((x+32, y));
        self.health_end.set_position((x+48, y));
    }

    pub fn update_bar(&mut self, new_health: usize) {
        // Match on ranges
        // first = 0..=16;
        // second = 17..=32;
        // third = 33..=48;
        // last 49..=50
        match (self.health_amt, new_health){
            // Both are first sprite
            (0..=16, 0..=16) => {
                let new_sprite = HP_SPRITE_ARR[16-new_health];
                self.health_mid1.set_sprite(self.object.sprite(new_sprite));
            },
            // Old is 2nd, New is 1st,
            (17..=32, 0..=16) => {
                self.health_mid2.hide();
                let new_sprite = HP_SPRITE_ARR[16-new_health];
                self.health_mid1.set_sprite(self.object.sprite(new_sprite));
            },
            // Both are second sprite
            (17..=32, 17..=32) => {
                let new_sprite = HP_SPRITE_ARR[32-new_health];
                self.health_mid2.set_sprite(self.object.sprite(new_sprite));
            },
            // Old is 3rd, New is 2nd
            (33..=48, 17..=32) => {
                self.health_mid3.hide();
                let new_sprite = HP_SPRITE_ARR[32-new_health];
                self.health_mid2.set_sprite(self.object.sprite(new_sprite));
            },
            // Both are 3rd sprite
            (33..=48, 33..=48) => {
                let new_sprite = HP_SPRITE_ARR[48-new_health];
                self.health_mid3.set_sprite(self.object.sprite(new_sprite));
            },
            // Old is last, New is 3rd
            (49.., 33..=48) => {
                self.health_end.hide();
                let new_sprite = HP_SPRITE_ARR[48-new_health];
                self.health_mid3.set_sprite(self.object.sprite(new_sprite));
            },
            // Both are last sprite
            (49.., 49..) => {
                let new_sprite = HP_SPRITE_ARR[64-new_health];
                self.health_end.set_sprite(self.object.sprite(new_sprite));
            },
            _ => todo!("Unexpected edge case of health ranges. too much dps!"),
        };

        self.health_amt = new_health;
    }

    pub fn hide_all(&mut self) {
        self.health_mid1.hide();
        self.health_mid2.hide();
        self.health_mid3.hide();
        self.health_end.hide();
    }

    pub fn hide_mid1(&mut self) {
        self.health_mid1.hide();
    }
}