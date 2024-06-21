use agb::display::object::{Object, OamManaged, Tag};

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
    dialog_portraits: [&'obj Tag; 7],
    portrait: Object<'obj>,
    dialog_1: Object<'obj>,
    dialog_2: Object<'obj>,
    dialog_3: Object<'obj>,
    dialog_4: Object<'obj>,
    dialog_5: Object<'obj>,
    object: &'obj OamManaged<'obj>,
}

impl<'obj> Dialog<'obj> {
    pub fn new(object: &'obj OamManaged<'obj>) -> Self {
        let dialog_portraits = [
            BLANK_PORTRAIT_SPRITE_TAG,
            TANK_PORTRAIT_SPRITE_TAG,
            TANK_PORTRAIT_SPRITE_TAG,
            HEALZ_PORTRAIT_SPRITE_TAG,
            BARB_PORTRAIT_SPRITE_TAG,
            WIZ_PORTRAIT_SPRITE_TAG,
            TANK_PORTRAIT_SPRITE_TAG
        ];
        let dialog_x = 8;
        let dialog_y = 128;

        let mut portrait: Object = object.object_sprite(dialog_portraits[0].sprite(0));
        portrait.set_position((dialog_x, dialog_y - 8));

        let mut dialog_1: Object = object.object_sprite(DIALOG_1_SPRITE_TAG.sprite(0));
        dialog_1.set_position((dialog_x + 40, dialog_y));
        let mut dialog_2: Object = object.object_sprite(DIALOG_2_SPRITE_TAG.sprite(0));
        dialog_2.set_position((dialog_x + 72, dialog_y));
        let mut dialog_3: Object = object.object_sprite(DIALOG_3_SPRITE_TAG.sprite(0));
        dialog_3.set_position((dialog_x + 104, dialog_y));
        let mut dialog_4: Object = object.object_sprite(DIALOG_4_SPRITE_TAG.sprite(0));
        dialog_4.set_position((dialog_x + 136, dialog_y));
        let mut dialog_5: Object = object.object_sprite(DIALOG_5_SPRITE_TAG.sprite(0));
        dialog_5.set_position((dialog_x + 168, dialog_y));

        let mut frame = Self {
            dialog_portraits,
            portrait,
            dialog_1,
            dialog_2,
            dialog_3,
            dialog_4,
            dialog_5,
            object,
        };

        frame
    }

    pub fn show_next_dialog(&mut self, dialog_ind: usize) {
        self.portrait.set_sprite(self.object.sprite(self.dialog_portraits[dialog_ind].sprite(0)));
        self.dialog_1.set_sprite(self.object.sprite(DIALOG_1_SPRITE_TAG.sprite(dialog_ind)));
        self.dialog_2.set_sprite(self.object.sprite(DIALOG_2_SPRITE_TAG.sprite(dialog_ind)));
        self.dialog_3.set_sprite(self.object.sprite(DIALOG_3_SPRITE_TAG.sprite(dialog_ind)));
        self.dialog_4.set_sprite(self.object.sprite(DIALOG_4_SPRITE_TAG.sprite(dialog_ind)));
        self.dialog_5.set_sprite(self.object.sprite(DIALOG_5_SPRITE_TAG.sprite(dialog_ind)));
    }

    pub fn show(&mut self) {
        self.portrait.show();
        self.dialog_1.show();
        self.dialog_2.show();
        self.dialog_3.show();
        self.dialog_4.show();
        self.dialog_5.show();
    }

    pub fn hide(&mut self) {
        self.portrait.hide();
        self.dialog_1.hide();
        self.dialog_2.hide();
        self.dialog_3.hide();
        self.dialog_4.hide();
        self.dialog_5.hide();
    }
}
