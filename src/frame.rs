use alloc::format;
use agb::{display::object::{Graphics, Object, OamManaged, Tag}, println};

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
    up: i32,
    pub selected_char: usize
}

impl<'obj> Frame<'obj> {
    pub fn new(object: &'obj OamManaged<'_>) -> Self {
        let mut top_left = object.object_sprite(TOP_LEFT.sprite(0));
        let mut top_right = object.object_sprite(TOP_RIGHT.sprite(0));
        let mut bot_left = object.object_sprite(BOT_LEFT.sprite(0));
        let mut bot_right = object.object_sprite(BOT_RIGHT.sprite(0));

        let mut frame = Self {
            top_left,
            top_right,
            bot_left,
            bot_right,
            left: 0,
            up: 0,
            selected_char: 0
        };

        frame.show();

        frame.update_position();
        frame
    }

    pub fn set_position(&mut self, left_right: i32, up_down: i32){
        self.left = (self.left + left_right).clamp(0, 1);
        self.up = (self.up + up_down).clamp(0, 1);
        self.update_position();
    }

    fn update_position(&mut self) {
        // left|up
        if self.left == 0 && self.up == 0 { // 00
            self.top_left.set_position((0, 1));
            self.top_right.set_position((64, 1));
            self.bot_left.set_position((0, 56));
            self.bot_right.set_position((64, 56));
            self.selected_char = 0;
        } else if self.left == 0 && self.up == 1 { // 01 Bot Left
            self.top_left.set_position((0, 64));
            self.top_right.set_position((64, 64));
            self.bot_left.set_position((0, 112));
            self.bot_right.set_position((64, 112));
            self.selected_char = 1;
        } else if self.left == 1 && self.up == 0 { // 10 Top Right
            self.top_left.set_position((72, 1));
            self.top_right.set_position((136, 1));
            self.bot_left.set_position((72, 56));
            self.bot_right.set_position((136, 56));
            self.selected_char = 2;
        } else { // 11
            self.top_left.set_position((72, 64));
            self.top_right.set_position((136, 64));
            self.bot_left.set_position((72, 112));
            self.bot_right.set_position((136, 112));
            self.selected_char = 3;
        }
    }

    pub fn show(&mut self){
        self.top_left.show();
        self.top_right.show();
        self.bot_left.show();
        self.bot_right.show();
    }
    pub fn hide(&mut self){
        self.top_left.hide();
        self.top_right.hide();
        self.bot_left.hide();
        self.bot_right.hide();
    }

    fn update(&mut self){
        // todo move frame in/out every couple frames for a breathe animation
    }
}