use agb::{display::object::{Graphics, Object, OamManaged, Tag}, include_aseprite, println};

use crate::game_manager::GRAPHICS;
static TOP_LEFT: &Tag = GRAPHICS.tags().get("top_left");
static TOP_RIGHT: &Tag = GRAPHICS.tags().get("top_right");
static BOT_LEFT: &Tag = GRAPHICS.tags().get("bot_left");
static BOT_RIGHT: &Tag = GRAPHICS.tags().get("bot_right");

pub struct Frame<'obj> {
    top_left: Object<'obj>,
    top_right: Object<'obj>,
    bot_left: Object<'obj>,
    bot_right: Object<'obj>,
    left: i32,
    up: i32
}

impl<'obj> Frame<'obj> {
    pub fn new(object: &'obj OamManaged<'_>, start_x: i32, start_y: i32) -> Self {
        let mut top_left = object.object_sprite(TOP_LEFT.sprite(0));
        let mut top_right = object.object_sprite(TOP_RIGHT.sprite(0));
        let mut bot_left = object.object_sprite(BOT_LEFT.sprite(0));
        let mut bot_right = object.object_sprite(BOT_RIGHT.sprite(0));

        top_left.show();
        top_right.show();
        bot_left.show();
        bot_right.show();
        // paddle_end.set_vflip(true).show();
        // paddle_end.set_hflip(true);

        let mut frame = Self {
            top_left,
            top_right,
            bot_left,
            bot_right,
            left: 0,
            up: 0
        };

        frame.update_position();
        frame
    }

    pub fn set_position(&mut self, left_right: i32, up_down: i32){
        self.left = (self.left + left_right).clamp(0, 1);
        self.up = (self.up + up_down).clamp(0, 1);
        println!("INside set position");
        println!("L-R: {}, U-D: {}", left_right, up_down);
        println!("Left: {}, up: {}", self.left, self.up);
        self.update_position();
    }

    fn update_position(&mut self) {
        if self.left == 0 && self.up == 0 { // 00
            self.top_left.set_position((0, 0));
            self.top_right.set_position((64, 0));
            self.bot_left.set_position((0, 56));
            self.bot_right.set_position((64, 56));
        } else if self.left == 0 && self.up == 1 { // 01
            self.top_left.set_position((64, 0));
            self.top_right.set_position((128, 0));
            self.bot_left.set_position((64, 56));
            self.bot_right.set_position((128, 56));
        } else if self.left == 1 && self.up == 0 { // 10
            self.top_left.set_position((0, 56));
            self.top_right.set_position((64, 56));
            self.bot_left.set_position((0, 112));
            self.bot_right.set_position((64, 112));
        } else { // 11
            self.top_left.set_position((0, 0));
            self.top_right.set_position((64, 0));
            self.bot_left.set_position((0, 56));
            self.bot_right.set_position((64, 56));
        }
    }
}