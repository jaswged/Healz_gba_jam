use crate::boss_health_bar::BossHealthBar;
use agb::display::object::{OamManaged, Object, Tag};
use agb::println;
use crate::bar::{BarType, Bar};
use crate::CHEST_SPRITE_TAG;
use crate::game_manager::GRAPHICS;

static SHIELD_TAG: &Tag = GRAPHICS.tags().get("boss_shield");
static CRAB_TAG: &Tag = GRAPHICS.tags().get("boss_crab");
static WIZARD_TAG: &Tag = GRAPHICS.tags().get("boss_wizard");

#[derive(Clone)]
pub enum BossType{
    Shield,
    Crab,
    Wizard,
}

pub struct Boss<'obj>{
    // aoe_timer should be easily divisible by 35 for the aeo bar
    boss_type: BossType,
    dps: i16,
    instance: Object<'obj>,
    pub is_dead: bool,
    pub health_bar: BossHealthBar<'obj>,
    pub cooldown_bar: Bar<'obj>,
    object: &'obj OamManaged<'obj>,
    pub aoe_timer: usize,
}

impl<'obj> Boss<'obj> {
    pub fn new(object: &'obj OamManaged<'obj>, boss_type: BossType, start_x: i32, start_y: i32, aoe_timer: usize) -> Self {
        let tag = match boss_type {
            BossType::Shield => { SHIELD_TAG }
            BossType::Crab => { CRAB_TAG }
            BossType::Wizard => { WIZARD_TAG }
        };
        let mut instance = object.object_sprite(tag.sprite(0));
        instance.set_position((start_x, start_y));
        instance.show();

        let health_bar = BossHealthBar::new(object, 173, 19);
        let cooldown_bar = Bar::new(&object, BarType::Cooldown, 188, 30);

        Boss {
            boss_type,
            dps: 3,
            instance,
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
}