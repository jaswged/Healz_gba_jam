use agb::display::object::{Object, Tag};
use agb::fixnum::num;

trait Health {
    fn heal(&mut self, heal: i16);
    fn damage(&mut self, damage: i16);
}

struct Character<'obj>{
    health: i16,
    dps: i16,
    sprite: Object<'obj>,
}

struct Position {
    x: i32,
    y: i32,
}

impl Health for Character<'_>{
    // todo could be same method to save rom size?
    fn heal(&mut self, heal: i16) {
        self.health += heal;
    }

    fn damage(&mut self, damage: i16) {
        self.health -= damage;
    }
}