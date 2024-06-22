use agb::display::object::{OamManaged, Object, Tag};

use crate::game_manager::GRAPHICS;

static TANK_PORTRAIT_SPRITE_TAG: &Tag = GRAPHICS.tags().get("tankey_portrait");
static BLANK_PORTRAIT_SPRITE_TAG: &Tag = GRAPHICS.tags().get("blank_portrait");
static BARB_PORTRAIT_SPRITE_TAG: &Tag = GRAPHICS.tags().get("barb_portrait");
static WIZ_PORTRAIT_SPRITE_TAG: &Tag = GRAPHICS.tags().get("wizard_portrait");
static HEALZ_PORTRAIT_SPRITE_TAG: &Tag = GRAPHICS.tags().get("healer_portrait");
static DIALOG_1_SPRITE_TAG: &Tag = GRAPHICS.tags().get("dialog_1");
static DIALOG_2_SPRITE_TAG: &Tag = GRAPHICS.tags().get("dialog_2");
static DIALOG_3_SPRITE_TAG: &Tag = GRAPHICS.tags().get("dialog_3");
static DIALOG_4_SPRITE_TAG: &Tag = GRAPHICS.tags().get("dialog_4");
static DIALOG_5_SPRITE_TAG: &Tag = GRAPHICS.tags().get("dialog_5");


pub struct Dialog<'obj> {
    portrait_order: [&'obj Tag; 16],
    portrait: Object<'obj>,
    sprite_1: Object<'obj>,
    sprite_2: Object<'obj>,
    sprite_3: Object<'obj>,
    sprite_4: Object<'obj>,
    sprite_5: Object<'obj>,
    object: &'obj OamManaged<'obj>,
}

impl<'obj> Dialog<'obj> {
    pub fn new(object: &'obj OamManaged<'obj>) -> Self {
        let portrait_order = [
            BLANK_PORTRAIT_SPRITE_TAG,
            TANK_PORTRAIT_SPRITE_TAG, // This is first boss
            TANK_PORTRAIT_SPRITE_TAG, // Dont forget the AOE
            // Cyclops boss
            HEALZ_PORTRAIT_SPRITE_TAG, // Ha! Piece of cake.
            BARB_PORTRAIT_SPRITE_TAG,  // Good warmup. Whos ready for the sewer?
            // Minotaur boss
            WIZ_PORTRAIT_SPRITE_TAG, // This is the boss we wiped on last week
            HEALZ_PORTRAIT_SPRITE_TAG, // No problem. Big heals incoming
            // Crab boss
            BARB_PORTRAIT_SPRITE_TAG, // I think we're getting the hang of this raid.
            HEALZ_PORTRAIT_SPRITE_TAG, // Told yall i could do it. Lets keep going
            // Demon boss
            TANK_PORTRAIT_SPRITE_TAG, // Alright, this is the final boss
            WIZ_PORTRAIT_SPRITE_TAG,  // I've got a date with some shiny new loot!
            // Wizard boss
            TANK_PORTRAIT_SPRITE_TAG, // Huzzah. Great work team.
            BARB_PORTRAIT_SPRITE_TAG, // Come on already. Lets see what dropped.
            // End of dungeon
            WIZ_PORTRAIT_SPRITE_TAG, // No way, the bow dropped. We can't even use that.
            HEALZ_PORTRAIT_SPRITE_TAG, // Guess i'll see you next week for heroics
            BLANK_PORTRAIT_SPRITE_TAG,
        ];
        let dialog_x = 8;
        let dialog_y = 130;

        let mut portrait: Object = object.object_sprite(portrait_order[0].sprite(0));
        portrait.set_position((dialog_x, dialog_y - 8));

        let mut sprite_1: Object = object.object_sprite(DIALOG_1_SPRITE_TAG.sprite(0));
        sprite_1.set_position((dialog_x + 40, dialog_y));
        let mut sprite_2: Object = object.object_sprite(DIALOG_2_SPRITE_TAG.sprite(0));
        sprite_2.set_position((dialog_x + 72, dialog_y));
        let mut sprite_3: Object = object.object_sprite(DIALOG_3_SPRITE_TAG.sprite(0));
        sprite_3.set_position((dialog_x + 104, dialog_y));
        let mut sprite_4: Object = object.object_sprite(DIALOG_4_SPRITE_TAG.sprite(0));
        sprite_4.set_position((dialog_x + 136, dialog_y));
        let mut sprite_5: Object = object.object_sprite(DIALOG_5_SPRITE_TAG.sprite(0));
        sprite_5.set_position((dialog_x + 168, dialog_y));

        Self {
            portrait_order,
            portrait,
            sprite_1,
            sprite_2,
            sprite_3,
            sprite_4,
            sprite_5,
            object,
        }
    }

    pub fn show_next_dialog(&mut self, dialog_ind: usize) {
        self.portrait.set_sprite(self.object.sprite(self.portrait_order[dialog_ind].sprite(0)));
        self.sprite_1.set_sprite(self.object.sprite(DIALOG_1_SPRITE_TAG.sprite(dialog_ind)));
        self.sprite_2.set_sprite(self.object.sprite(DIALOG_2_SPRITE_TAG.sprite(dialog_ind)));
        self.sprite_3.set_sprite(self.object.sprite(DIALOG_3_SPRITE_TAG.sprite(dialog_ind)));
        self.sprite_4.set_sprite(self.object.sprite(DIALOG_4_SPRITE_TAG.sprite(dialog_ind)));
        self.sprite_5.set_sprite(self.object.sprite(DIALOG_5_SPRITE_TAG.sprite(dialog_ind)));
    }

    pub fn show(&mut self) {
        self.portrait.show();
        self.sprite_1.show();
        self.sprite_2.show();
        self.sprite_3.show();
        self.sprite_4.show();
        self.sprite_5.show();
    }

    pub fn hide(&mut self) {
        self.portrait.hide();
        self.sprite_1.hide();
        self.sprite_2.hide();
        self.sprite_3.hide();
        self.sprite_4.hide();
        self.sprite_5.hide();
    }
}
