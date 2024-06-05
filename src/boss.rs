use crate::boss_health_bar::BossHealthBar;
use crate::game_manager::GRAPHICS;
use agb::display::object::{OamManaged, Object, Tag};
use agb::println;
use crate::SKULL_SPRITE_TAG;

static BOSS_SPRITE: &Tag = GRAPHICS.tags().get("boss");

pub struct Boss<'obj>{
    dps: i16,
    instance: Object<'obj>,
    pub is_dead: bool,
    pub health_bar: BossHealthBar<'obj>,
    object: &'obj OamManaged<'obj>,
}

impl<'obj> Boss<'obj> {
    pub fn new(object: &'obj OamManaged<'obj>, start_x: i32, start_y: i32) -> Self {
        let mut instance = object.object_sprite(BOSS_SPRITE.sprite(0));
        instance.set_position((start_x, start_y));
        instance.show();

        let health_bar = BossHealthBar::new(object, 173, 19);

        Boss {
            dps: 3,
            instance,
            is_dead: false,
            health_bar,
            object,
        }
    }

    pub fn take_damage(&mut self, damage: usize) {
        // todo divide damage in half, so it effectively has 100 hp instead of 50
        println!("Took {} damage!", damage);
        // todo here jason
        if damage >= self.health_bar.health_amt {
            println!("Boss is Dead! You win bruv");
            self.health_bar.health_amt = 0;
            self.health_bar.hide_mid1();
            self.is_dead = true;
            self.instance.set_sprite(self.object.sprite(SKULL_SPRITE_TAG.sprite(0)));
            return;
        }
        let new_health = self.health_bar.health_amt - damage;

        self.health_bar.update_bar(new_health);
    }

    pub fn hide(&mut self) {
        self.instance.hide();
        self.health_bar.hide_all();
    }
}