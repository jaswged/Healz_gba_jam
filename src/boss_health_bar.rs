// change our imports to include what we will use
use agb::{
    display::object::{Graphics, Object, OamManaged, Tag},
    include_aseprite,
};

// pub static GRAPHICS: &Graphics = include_aseprite!("gfx/sprites.aseprite");
use crate::game_manager::GRAPHICS;

static HEALTH_BAR_SKULL: &Tag = GRAPHICS.tags().get("bhp_skull");
static HEALTH_BAR_START: &Tag = GRAPHICS.tags().get("bhp_start");
static HEALTH_MID: &Tag = GRAPHICS.tags().get("bhp_health");
static HEALTH_MID_EMPTY: &Tag = GRAPHICS.tags().get("bhp_empty");
static HEALTH_BAR_END: &Tag = GRAPHICS.tags().get("bhp_end");

pub struct BossHealthBar<'obj> {
    health_skull: Object<'obj>,
    health_start: Object<'obj>,
    health_mid: Object<'obj>,
    health_mt: Object<'obj>,
    health_end: Object<'obj>,
}

impl<'obj> BossHealthBar<'obj> {
    pub fn new(object: &'obj OamManaged<'_>, start_x: i32, start_y: i32) -> Self {
        let mut health_skull = object.object_sprite(HEALTH_BAR_SKULL.sprite(0));
        let mut health_start = object.object_sprite(HEALTH_BAR_START.sprite(0));
        let mut health_mid = object.object_sprite(HEALTH_MID.sprite(0));
        let mut health_mt = object.object_sprite(HEALTH_MID_EMPTY.sprite(0));
        let mut health_end = object.object_sprite(HEALTH_BAR_END.sprite(0));

        health_skull.show();
        health_start.show();
        health_mid.show();
        health_mt.show();
        health_end.show();

        let mut b_health_bar = Self {
            health_skull,
            health_start,
            health_mid,
            health_mt,
            health_end,
        };

        b_health_bar.set_position(start_x, start_y);

        b_health_bar
    }

    fn set_position(&mut self, x: i32, y: i32) {
        // new! use of the `set_position` method. This is a helper feature using agb's vector types.
        self.health_skull.set_position((x, y));
        // Start is 4px wide
        self.health_start.set_position((x+16, y));

        // Each mid section is 1px
        /* todo the spacing between and duplication logic. See asciimon
        let mut to_return = "#".repeat(self.health as usize);
        let remaining = self.max_health - self.health;
        if remaining != 0{
            let a = "_".repeat(remaining as usize);
            to_return.push_str(&a);
        }
         */
        self.health_mid.set_position((x+20, y));
        self.health_mt.set_position((x+21, y));

        // End goes at a fixed distance from start
        self.health_end.set_position((x+22, y)); // 80
    }
}