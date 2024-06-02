use alloc::vec::{self, Vec};
use agb::{display::object::{Graphics, Object, OamManaged, Tag}};
use agb::display::Priority;
use agb::display::tiled::{RegularBackgroundSize, Tiled0, VRamManager};
use agb::display::object::Sprite;
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
static HP_SPRITE_ARR: [&Sprite; 8] = [HP_8_SPRITE,HP_7_SPRITE,HP_6_SPRITE,HP_5_SPRITE,HP_4_SPRITE,HP_3_SPRITE,HP_2_SPRITE,HP_1_SPRITE];


pub struct HealthBar<'obj> {
    health_amt: usize,
    health_mid1: Object<'obj>,
    health_mid2: Object<'obj>,
    health_mid3: Object<'obj>,
    health_mid4: Object<'obj>,
    health_end: Object<'obj>,
    object: &'obj OamManaged<'obj>,
}

impl<'obj> HealthBar<'obj> {
    pub fn new(object: &'obj OamManaged<'obj>, start_x: i32, start_y: i32) -> Self {
        let mut health_amt = 32; // 32 for easy math. set back to 35 if figure out a solution
        let filled = HP_SPRITE_ARR[0];
        let mut health_mid1 = object.object_sprite(filled);
        let mut health_mid2 = object.object_sprite(filled);
        let mut health_mid3 = object.object_sprite(filled);
        let mut health_mid4 = object.object_sprite(filled);
        let mut health_end = object.object_sprite(filled);

        health_mid1.show();
        health_mid2.show();
        health_mid3.show();
        health_mid4.show();
        // health_end.show();

        let mut health_bar = Self {
            health_amt,
            health_mid1,
            health_mid2,
            health_mid3,
            health_mid4,
            health_end,
            object
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

    pub fn take_damage(&mut self, damage: usize){
        println!("Took {} damage!", damage);
        // todo here jason
        if damage >= self.health_amt {
            println!("Is Dead!");
            self.health_amt = 0;
            self.health_mid1.hide();
            // todo what now? How to trigger game over? set skull sprite on player next frame?
            return
        }
        let new_health = self.health_amt - damage;

        // todo. if the damage value passed in is less than 8 you only have to look at 2 blocks
        // is there an easy way to know if they are in the same block?

        // todo match on ranges
        println!("Test ranges where they are the same");
        let mut orig_block = match (self.health_amt, new_health){
            (0..=8, 0..=8) => {
                println!("First sprite");
                // Calculate new sprite off of the new value
                println!("Diff is {}", 8-new_health);
                let new_sprite = HP_SPRITE_ARR[8-new_health];
                self.health_mid1.set_sprite(self.object.sprite(new_sprite));
                (1, &mut self.health_mid1, 8-self.health_amt)
            },
            (9..=16, 9..=16) => {
                println!("Second sprite");
                println!("Diff is {}", 16-new_health);
                // self.health_mid2.set_sprite(self.object.sprite(HP_1_SPRITE));
                (2, &mut self.health_mid2, 16-self.health_amt)
            },
            (17..=24, 17..=24) => {
                println!("Third sprite");
                println!("Diff is {}", 24-new_health);
                // self.health_mid3.set_sprite(self.object.sprite(HP_1_SPRITE));
                (3, &mut self.health_mid3, 24-self.health_amt)
            },
            (25..=32, 25..=32) => {
                println!("Fourth sprite");
                println!("Diff is {}", 32-new_health);
                // self.health_mid4.set_sprite(self.object.sprite(HP_1_SPRITE));
                (4, &mut self.health_mid4, 32-self.health_amt)
            },
            (33.., 33..) => {
                println!("Overhealed?! End sprite");
                // self.health_end.set_sprite(self.object.sprite(HP_1_SPRITE));
                (5, &mut self.health_end, 35-self.health_amt)
            }
            _ => todo!("Implement the cases where the start and end blocks arent the same"),
        };
        // let mut next_block = match new_health{
        //     0..=8 => {
        //         println!("First sprite");
        //         (1, &mut self.health_mid1, 8-new_health)
        //     },
        //     9..=16 => {
        //         println!("Second sprite");
        //         (2, &mut self.health_mid2, 16-new_health)
        //     },
        //     17..=24 => {
        //         println!("Third sprite");
        //         (3, &mut self.health_mid3, 24-new_health)
        //     },
        //     25..=32 => {
        //         println!("Fourth sprite");
        //         (4, &mut self.health_mid4, 32-new_health)
        //     },
        //     33.. => {
        //         println!("Overhealed?! End sprite");
        //         (5, &mut self.health_end, 35-new_health)
        //     },
        // };

        // if orig_block.0 == next_block.0 {
        //     println!("Is same block, just need to update the sprite");
        //     println!("Diff of nextBlock: {}", next_block.2);
        //     // Calculate new sprite off of the new value
        //     next_block.1.set_sprite(self.object.sprite(HP_1_SPRITE));
        // }

        self.health_amt = new_health;

        println!("Current health is: {}", self.health_amt);
        // self.update_bar(damage);
    }

    // fn get_block(&mut self, amt: usize) -> (usize, &mut Object<'obj>, usize) {
    //     match amt{
    //         0..=8 => {
    //             println!("First sprite");
    //             (1, &mut self.health_mid1, 8-amt)
    //         },
    //         9..=16 => {
    //             println!("Second sprite");
    //             (2, &mut self.health_mid2, 16-amt)
    //         },
    //         17..=24 => {
    //             println!("Third sprite");
    //             (3, &mut self.health_mid3, 24-amt)
    //         },
    //         25..=32 => {
    //             println!("Fourth sprite");
    //             (4, &mut self.health_mid4, 32-amt)
    //         },
    //         33.. => {
    //             println!("Overhealed?! End sprite");
    //             (5, &mut self.health_end, 35-amt)
    //         },
    //     }
    // }

    // fn update_bar(&mut self, damage: usize) {
    //     // currently decrement only!
    //     println!("which one of the sprites to update");
    //     // convert from len 35 to which of
    //     let todo = 0;
    //     println!("{}", todo);
    //     // println!("\nUpdate health bar. Ptr at {}", self.heath_ptr);
    // }
}