// change our imports to include what we will use
use agb::{
    display::object::{Graphics, Object, OamManaged, Tag},
    include_aseprite,
};

pub static GRAPHICS: &Graphics = include_aseprite!("gfx/sprites.aseprite");

static PADDLE_END: &Tag = GRAPHICS.tags().get("Paddle End");
static PADDLE_MID: &Tag = GRAPHICS.tags().get("Paddle Mid");

pub struct Paddle<'obj> {
    start: Object<'obj>,
    mid: Object<'obj>,
    end: Object<'obj>,
}

impl<'obj> Paddle<'obj> {
    pub fn new(object: &'obj OamManaged<'_>, start_x: i32, start_y: i32, flip: bool) -> Self {
        let mut paddle_start = object.object_sprite(PADDLE_END.sprite(0));
        let mut paddle_mid = object.object_sprite(PADDLE_MID.sprite(0));
        let mut paddle_end = object.object_sprite(PADDLE_END.sprite(0));

        if flip{
            paddle_start.set_hflip(true);
            paddle_mid.set_hflip(true);
            paddle_end.set_hflip(true);
        }

        paddle_start.show();
        paddle_mid.show();
        paddle_end.set_vflip(true).show();

        let mut paddle = Self {
            start: paddle_start,
            mid: paddle_mid,
            end: paddle_end,
        };

        paddle.set_position(start_x, start_y);

        paddle
    }

    fn set_position(&mut self, x: i32, y: i32) {
        // new! use of the `set_position` method. This is a helper feature using
        // agb's vector types. For now we can just use it to avoid adding them 
        // separately
        self.start.set_position((x, y));
        self.mid.set_position((x, y + 16));
        self.end.set_position((x, y + 32));
    }
}
