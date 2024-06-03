use agb::display::object::{OamManaged, Object, Tag};
use crate::game_manager::GRAPHICS;
use crate::health_bar::HealthBar;

static HEALER_SPRITE_TAG: &Tag = GRAPHICS.tags().get("healer");
static BARB_SPRITE_TAG: &Tag = GRAPHICS.tags().get("barb");
static TANKEY_SPRITE_TAG: &Tag = GRAPHICS.tags().get("tankey");
static WIZARD_SPRITE_TAG: &Tag = GRAPHICS.tags().get("wizard");

pub enum Profession {
    HEALER,
    WIZARD,
    TANK,
    BARB
}

pub struct Character<'obj>{
    dps: i16,
    profession: Profession,
    instance: Object<'obj>,
    pub health_bar: HealthBar<'obj>,
    object: &'obj OamManaged<'obj>,
}

impl<'obj> Character<'obj> {
    pub fn new(object: &'obj OamManaged<'obj>, start_x: i32, start_y: i32, profession: Profession, dps: i16) -> Self {
        let sprite_tag = match profession{
            Profession::HEALER => HEALER_SPRITE_TAG,
            Profession::WIZARD => WIZARD_SPRITE_TAG,
            Profession::TANK => TANKEY_SPRITE_TAG,
            Profession::BARB => BARB_SPRITE_TAG
        };
        let mut instance = object.object_sprite(sprite_tag.sprite(0));
        instance.set_position((start_x, start_y));
        instance.show();

        let health_bar = HealthBar::new(&object, start_x + 4, start_y-12);

        Character{
            dps,
            profession,
            instance,
            health_bar,
            object
        }
    }

    fn show_health(){
        // show health later after creating the char so you can show dialog stuff
        todo!();
    }
}
