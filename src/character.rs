use crate::game_manager::GRAPHICS;
use crate::SKULL_SPRITE_TAG;
use agb::{display::object::{OamManaged, Object, Tag}, println};
use crate::bar::{BarType, Bar};

static HEALER_SPRITE_TAG: &Tag = GRAPHICS.tags().get("healer");
static BARB_SPRITE_TAG: &Tag = GRAPHICS.tags().get("barb");
static TANKEY_SPRITE_TAG: &Tag = GRAPHICS.tags().get("tankey");
static WIZARD_SPRITE_TAG: &Tag = GRAPHICS.tags().get("wizard");

pub enum Profession {
    Healer,
    Wizard,
    Tank,
    Barb,
}

pub struct Character<'obj> {
    pub dps: usize,
    profession: Profession,
    pub instance: Object<'obj>,
    pub is_dead: bool,
    pub health_bar: Bar<'obj>,
    object: &'obj OamManaged<'obj>,
    pub tag: &'obj Tag,
}

impl<'obj> Character<'obj> {
    pub fn new(object: &'obj OamManaged<'obj>, start_pos: (i32, i32), profession: Profession, dps: usize) -> Self {
        let sprite_tag = match profession{
            Profession::Healer => HEALER_SPRITE_TAG,
            Profession::Wizard => WIZARD_SPRITE_TAG,
            Profession::Tank => TANKEY_SPRITE_TAG,
            Profession::Barb => BARB_SPRITE_TAG,
        };
        let mut instance = object.object_sprite(sprite_tag.sprite(0));
        instance.set_position((start_pos.0 + 16, start_pos.1 + 16));
        instance.show();

        let health_bar = Bar::new(object, BarType::Health, start_pos.0 + 20, start_pos.1+4);

        Character {
            dps,
            profession,
            instance,
            is_dead: false,
            health_bar,
            object,
            tag: sprite_tag,
        }
    }

    pub fn take_damage(&mut self, damage: usize) {
        if damage >= self.health_bar.bar_amt {
            self.health_bar.bar_amt = 0;
            self.health_bar.hide_mana_mid1();

            self.is_dead = true;

            // Set sprite to Skull
            self.instance.set_sprite(self.object.sprite(SKULL_SPRITE_TAG.sprite(0)));
            return
        }
        let new_health = self.health_bar.bar_amt - damage;

        self.health_bar.update_bar(new_health);
    }

    pub fn take_heals(&mut self, heals: usize) {
        if self.is_dead {
            return;
        }

        // todo here jason
        let mut new_health = self.health_bar.bar_amt + heals;
        if new_health >= self.health_bar.bar_max {
            // todo overhealed number added up here.
            self.health_bar.bar_amt = self.health_bar.bar_max;
            new_health = self.health_bar.bar_max;
        }

        self.health_bar.update_bar(new_health);
    }

    pub fn hide(&mut self) {
        self.instance.hide();
    }

    pub fn show(&mut self) {
        self.instance.show();
    }

    pub fn hide_health(&mut self) {
        // hide health after creating the char so you can show dialog stuff
        self.health_bar.hide_all();
    }

    pub fn show_health(&mut self) {
        // show health later after creating the char so you can show dialog stuff
        self.health_bar.show_all();
    }

    pub fn full_heal(&mut self) {
        println!("Full heal");
        self.health_bar.fill_bar();
    }
}
