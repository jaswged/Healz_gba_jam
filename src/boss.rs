use crate::bar::{Bar, BarType};
use crate::boss_health_bar::BossHealthBar;
use crate::game_manager::GRAPHICS;
use crate::CHEST_SPRITE_TAG;
use agb::display::object::{OamManaged, Object, Tag};

static BAT_TAG: &Tag = GRAPHICS.tags().get("boss_bat");
static CRAB_TAG: &Tag = GRAPHICS.tags().get("boss_crab");
static CRABBIES_TAG: &Tag = GRAPHICS.tags().get("boss_crabbies");
static CYCLOPS_TAG: &Tag = GRAPHICS.tags().get("boss_cyclops");
static DEMON_TAG: &Tag = GRAPHICS.tags().get("boss_demon");
static MINOTAUR_TAG: &Tag = GRAPHICS.tags().get("boss_minotaur");
static WIZARD_TAG: &Tag = GRAPHICS.tags().get("boss_wizard");
static NAME_1_TAG: &Tag = GRAPHICS.tags().get("boss_1_name");
static NAME_WIZARD: &Tag = GRAPHICS.tags().get("wizard_name");
static NAME_FERRIS: &Tag = GRAPHICS.tags().get("ferris_name");
static NAME_MINOTAUR: &Tag = GRAPHICS.tags().get("minotaur_name");
static NAME_CYCLOPS: &Tag = GRAPHICS.tags().get("cyclops_name");
static NAME_DEMON: &Tag = GRAPHICS.tags().get("demon_name");

#[derive(Clone)]
pub enum BossType {
    Bats,
    Crab,
    Cyclops,
    Demon,
    Minotaur,
    Wizard,
}

pub struct Boss<'obj> {
    // aoe_timer should be easily divisible by 35 for the aeo bar
    pub dps_mod: usize,
    instance: Object<'obj>,
    name_obj_1: Object<'obj>,
    name_obj_2: Object<'obj>,
    name_obj_3: Object<'obj>,
    name_obj_4: Object<'obj>,
    pub sprite_tag: &'obj Tag,
    pub is_dead: bool,
    pub health_bar: BossHealthBar<'obj>,
    pub cooldown_bar: Bar<'obj>,
    object: &'obj OamManaged<'obj>,
    pub aoe_timer: usize,
}

impl<'obj> Boss<'obj> {
    pub fn new(object: &'obj OamManaged<'obj>, boss_type: BossType, start_x: i32, start_y: i32, aoe_timer: usize, dps: usize) -> Self {
        // Start_x: 152, start_y: 48
        let (name_tag, sprite_tag) = match boss_type {
            BossType::Bats => (NAME_1_TAG, BAT_TAG),
            BossType::Crab => (NAME_FERRIS, CRAB_TAG),
            BossType::Cyclops => (NAME_CYCLOPS, CYCLOPS_TAG),
            BossType::Demon => (NAME_DEMON, DEMON_TAG),
            BossType::Minotaur => (NAME_MINOTAUR, MINOTAUR_TAG),
            BossType::Wizard => (NAME_WIZARD, WIZARD_TAG),
        };
        let mut instance = object.object_sprite(sprite_tag.sprite(0));
        instance.set_position((start_x, start_y));
        instance.show();

        let mut name_obj_1 = object.object_sprite(name_tag.sprite(0));
        name_obj_1.set_position((start_x + 14, start_y - 45)).show();
        let mut name_obj_2 = object.object_sprite(name_tag.sprite(1));
        name_obj_2.set_position((start_x + 30, start_y - 45)).show();
        let mut name_obj_3 = object.object_sprite(name_tag.sprite(2));
        name_obj_3.set_position((start_x + 46, start_y - 45)).show();
        let mut name_obj_4 = object.object_sprite(name_tag.sprite(3));
        name_obj_4.set_position((start_x + 62, start_y - 45)).show();

        let health_bar = BossHealthBar::new(object, 173, 19);
        let cooldown_bar = Bar::new(object, BarType::Cooldown, 188, 30);

        Boss {
            dps_mod: dps,
            instance,
            name_obj_1,
            name_obj_2,
            name_obj_3,
            name_obj_4,
            sprite_tag,
            is_dead: false,
            health_bar,
            cooldown_bar,
            object,
            aoe_timer,
        }
    }

    pub fn take_damage(&mut self, damage: usize) {
        // Could divide damage in half, so it effectively has 100 hp instead of 50
        if damage >= self.health_bar.health_amt {
            self.health_bar.health_amt = 0;
            self.health_bar.hide_mid1();
            self.is_dead = true;
            self.instance.set_sprite(self.object.sprite(CHEST_SPRITE_TAG.sprite(0)));
            self.instance.set_position((175, 70));
            return;
        }
        let new_health = self.health_bar.health_amt - damage;

        self.health_bar.update_bar(new_health);
    }

    pub fn hide(&mut self) {
        self.instance.hide();
        self.health_bar.hide_all();
        self.cooldown_bar.hide_all();
        self.name_obj_1.hide();
        self.name_obj_2.hide();
        self.name_obj_3.hide();
        self.name_obj_4.hide();
    }

    pub fn hide_cooldown(&mut self) {
        self.cooldown_bar.hide_all();
    }

    pub fn update(&mut self, frame: usize) {
        self.instance.set_sprite(self.object.sprite(self.sprite_tag.animation_sprite(frame / 13)));
    }
}