use crate::game_manager::GRAPHICS;
use crate::SKULL_SPRITE_TAG;
use agb::display::object::{OamManaged, Object, Tag};
use crate::bar::{BarType, Bar};

static HEALER_SPRITE_TAG: &Tag = GRAPHICS.tags().get("healer_idle");
static HEALER_ACT_SPRITE_TAG: &Tag = GRAPHICS.tags().get("healer_act");
static HEALER_MEDITATE_SPRITE_TAG: &Tag = GRAPHICS.tags().get("healer_meditate");
static BARB_SPRITE_TAG: &Tag = GRAPHICS.tags().get("barb_idle");
static BARB_ACT_SPRITE_TAG: &Tag = GRAPHICS.tags().get("barb_act");
static TANKEY_SPRITE_TAG: &Tag = GRAPHICS.tags().get("tankey_idle");
static TANKEY_ACT_SPRITE_TAG: &Tag = GRAPHICS.tags().get("tankey_act");
static WIZARD_SPRITE_TAG: &Tag = GRAPHICS.tags().get("wizard_idle");
static WIZARD_ACT_SPRITE_TAG: &Tag = GRAPHICS.tags().get("wizard_act");

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
    pub just_died: bool,
    pub health_bar: Bar<'obj>,
    object: &'obj OamManaged<'obj>,
    pub idle_tag: &'obj Tag,
    pub action_tag: &'obj Tag,
}

impl<'obj> Character<'obj> {
    pub fn new(object: &'obj OamManaged<'obj>, start_pos: (i32, i32), profession: Profession, dps: usize) -> Self {
        let idle_tag = match profession{
            Profession::Healer => HEALER_SPRITE_TAG,
            Profession::Wizard => WIZARD_SPRITE_TAG,
            Profession::Tank => TANKEY_SPRITE_TAG,
            Profession::Barb => BARB_SPRITE_TAG,
        };
        let action_tag = match profession{
            Profession::Healer => HEALER_ACT_SPRITE_TAG,
            Profession::Wizard => WIZARD_ACT_SPRITE_TAG,
            Profession::Tank => TANKEY_ACT_SPRITE_TAG,
            Profession::Barb => BARB_ACT_SPRITE_TAG,
        };

        let mut instance = object.object_sprite(idle_tag.sprite(0));
        instance.set_position((start_pos.0 + 16, start_pos.1 + 16));
        instance.show();

        let health_bar = Bar::new(object, BarType::Health, start_pos.0 + 20, start_pos.1+4);

        Character {
            dps,
            profession,
            instance,
            is_dead: false,
            just_died: false,
            health_bar,
            object,
            idle_tag,
            action_tag
        }
    }

    pub fn take_damage(&mut self, damage: usize) {
        if damage >= self.health_bar.bar_amt {
            self.health_bar.bar_amt = 0;
            self.health_bar.hide_all();

            self.is_dead = true;
            self.just_died = true;

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
        // hide health after creating the char, so you can show dialog stuff
        self.health_bar.hide_all();
    }

    pub fn show_health(&mut self) {
        // show health later after creating the char so you can show dialog stuff
        self.health_bar.show_all();
    }

    pub fn revive(&mut self){
        self.is_dead = false;
        self.full_heal();
    }

    pub fn full_heal(&mut self) {
        self.health_bar.fill_bar();
    }

    pub fn start_meditating(&mut self) {
        self.action_tag = HEALER_MEDITATE_SPRITE_TAG;
    }

    pub fn stop_meditating(&mut self) {
        self.action_tag = HEALER_ACT_SPRITE_TAG;
    }

    pub fn update_animation(&mut self, frame: usize) {
        if !self.is_dead {
            self.instance.set_sprite(self.object.sprite(self.action_tag.animation_sprite(frame)));
        }
    }

    pub fn update_idle_animation(&mut self, frame: usize) {
        if !self.is_dead {
            self.instance.set_sprite(self.object.sprite(self.idle_tag.animation_sprite(frame)));
        }
    }

    pub fn update_animations(chars: &mut [Character; 4], frame: usize) {
        for c in chars {
            if !c.is_dead {
                c.instance.set_sprite(c.object.sprite(c.action_tag.animation_sprite(frame)));
            }
        }
    }

    pub fn update_idle_animations(chars: &mut [Character; 4], frame: usize) {
        for c in chars {
            if !c.is_dead {
                c.instance.set_sprite(c.object.sprite(c.idle_tag.animation_sprite(frame)));
            }
        }
    }
}
