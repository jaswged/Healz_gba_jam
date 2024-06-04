use agb::display::object::{OamManaged, Object, Tag};
use agb::println;
use crate::game_manager::GRAPHICS;
use crate::health_bar::HealthBar;
use crate::SKULL_SPRITE_TAG;

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
    pub is_dead: bool,
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
            is_dead: false,
            health_bar,
            object
        }
    }

    pub fn take_damage(&mut self, damage: usize){
        println!("Took {} damage!", damage);
        if damage >= self.health_bar.health_amt {
            println!("Is Dead!");
            self.health_bar.health_amt = 0;
            self.health_bar.hide_mid1();

            self.is_dead = true;

            // Set sprite to Skull
            self.instance.set_sprite(self.object.sprite(SKULL_SPRITE_TAG.sprite(0)));
            return
        }
        let new_health = self.health_bar.health_amt - damage;

        self.health_bar.update_bar(new_health);
    }

    pub fn take_heals(&mut self, heals: usize){
        if self.is_dead {return}

        // todo here jason
        let mut new_health = self.health_bar.health_amt + heals;
        if new_health >= self.health_bar.health_max {
            println!("Is fully healed!");
            // todo overhealed number added up here.
            self.health_bar.health_amt = self.health_bar.health_max;
            new_health = self.health_bar.health_max;
        }

        self.health_bar.update_bar(new_health);
    }

    pub fn hide(&mut self){
        println!("should be hiding yourself");
        self.instance.hide();
    }

    pub fn hide_health(&mut self){
        // hide health after creating the char so you can show dialog stuff
        self.health_bar.hide_all();
    }

    pub fn show_health(&mut self){
        // show health later after creating the char so you can show dialog stuff
        self.health_bar.show_all();
    }
}
