use crate::boss_health_bar::BossHealthBar;
use agb::display::object::{OamManaged, Object, Tag};
use agb::println;
use crate::bar::{BarType, Bar};
use crate::CHEST_SPRITE_TAG;
use crate::game_manager::GRAPHICS;

static CYCLOPS_TAG: &Tag = GRAPHICS.tags().get("boss_cyclops");
static MINOTAUR_TAG: &Tag = GRAPHICS.tags().get("boss_minotaur");
static SHIELD_TAG: &Tag = GRAPHICS.tags().get("boss_shield");
static CRAB_TAG: &Tag = GRAPHICS.tags().get("boss_crab");
static WIZARD_TAG: &Tag = GRAPHICS.tags().get("boss_wizard");
static NAME_TAG: &Tag = GRAPHICS.tags().get("boss_1_name");


#[derive(Clone)]
pub enum BossType{
    Cyclops,
    Minotaur,
    Shield,
    Crab,
    Wizard,
}

pub struct Boss<'obj>{
    // aoe_timer should be easily divisible by 35 for the aeo bar
    boss_type: BossType,
    dps: i16,
    instance: Object<'obj>,
    pub sprite_tag: &'obj Tag,
    pub name_tag: &'obj Tag,
    pub is_dead: bool,
    pub health_bar: BossHealthBar<'obj>,
    pub cooldown_bar: Bar<'obj>,
    object: &'obj OamManaged<'obj>,
    pub aoe_timer: usize,
}

impl<'obj> Boss<'obj> {
    pub fn new(object: &'obj OamManaged<'obj>, boss_type: BossType, start_x: i32, start_y: i32, aoe_timer: usize) -> Self {
        let sprite_tag = match boss_type {
            BossType::Cyclops => {CYCLOPS_TAG }
            BossType::Minotaur => { MINOTAUR_TAG }
            BossType::Shield => { SHIELD_TAG }
            BossType::Crab => { CRAB_TAG }
            BossType::Wizard => { WIZARD_TAG }
        };
        let mut instance = object.object_sprite(sprite_tag.sprite(0));
        instance.set_position((start_x, start_y));
        instance.show();

        println!("Create boss name tags");
        let name_tag = NAME_TAG;
        let mut name_1 = object.object_sprite(sprite_tag.sprite(0));
        name_1.set_position((start_x, start_y - 16)).show();
        let mut name_2 = object.object_sprite(sprite_tag.sprite(1));
        name_2.set_position((start_x + 16, start_y - 16)).show();
        let mut name_3 = object.object_sprite(sprite_tag.sprite(2));
        name_3.set_position((start_x + 32, start_y - 16)).show();
        let mut name_4 = object.object_sprite(sprite_tag.sprite(3));
        name_4.set_position((start_x + 48, start_y - 16)).show();

        let health_bar = BossHealthBar::new(object, 173, 19);
        let cooldown_bar = Bar::new(&object, BarType::Cooldown, 188, 30);

        Boss {
            boss_type,
            dps: 3,
            instance,
            sprite_tag,
            name_tag,
            is_dead: false,
            health_bar,
            cooldown_bar,
            object,
            aoe_timer,
        }
    }

    pub fn take_damage(&mut self, damage: usize) {
        // todo divide damage in half, so it effectively has 100 hp instead of 50
        if damage >= self.health_bar.health_amt {
            println!("Boss is Dead! You win bruv");
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
    }

    pub fn hide_cooldown(&mut self){
        self.cooldown_bar.hide_all();
    }

    pub fn update(&mut self, frame: usize) {
        self.instance.set_sprite(self.object.sprite(self.sprite_tag.animation_sprite(frame)));
    }
}