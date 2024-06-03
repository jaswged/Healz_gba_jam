use alloc::vec::Vec;
// change our imports to include what we will use
use agb::{display::object::{Graphics, Object, OamManaged, Tag}, include_aseprite, println};
use agb::display::object::{Sprite};
use agb::fixnum::{Num, Vector2D};
use crate::bar;

// pub static GRAPHICS: &Graphics = include_aseprite!("gfx/sprites.aseprite");
use crate::game_manager::GRAPHICS;

// static HEALTH_BAR_SKULL: &Tag = GRAPHICS.tags().get("bhp_skull");
// static HEALTH_BAR_START: &Tag = GRAPHICS.tags().get("bhp_start");
// static HEALTH_MID: &Tag = GRAPHICS.tags().get("bhp_health");
// static HEALTH_MID_EMPTY: &Tag = GRAPHICS.tags().get("bhp_empty");
// static HEALTH_BAR_END: &Tag = GRAPHICS.tags().get("bhp_end");

// Get sprites to replace on the objects when damage is taken
// static ICON_SPRITE: &Sprite = HEALTH_BAR_SKULL.sprite(0);
// static START_SPRITE: &Sprite = HEALTH_BAR_START.sprite(0);
// static END_SPRITE: &Sprite = HEALTH_BAR_END.sprite(0);
// static MID_MT_SPRITE: &Sprite = HEALTH_MID_EMPTY.sprite(0);
// static MID_FILL_SPRITE: &Sprite = HEALTH_MID.sprite(0);

/** File to encapsulate all types of Bars
 * 
 * Boss Health Bar
 *   Skull beginning. 20px, 2 for end leaving 50 health slots
 * Boss timer bar
 *   Todo
 * Character health bar
 *   HP beginning. 12px, 2 for end leaving 34 health slots
 * Player mana bar same size as character health bar
*/
struct SpriteStruct<'obj>{
    icon_sprite: &'obj Sprite,
    start_sprite: &'obj Sprite,
    end_sprite: &'obj Sprite,
    fill_sprite: &'obj Sprite,
    mt_sprite: &'obj Sprite
}

pub enum BarType {  
    Boss_health,
    Char_health,
    // Char_mana,
    // Boss_timer
}

struct BarSegment<'obj>{
    id: usize,
    bar_seg: Object<'obj>,
}

impl<'obj> BarSegment<'obj>{
    fn new(bar_seg: Object<'obj>, id: usize) -> Self {
        Self {
            id,
            bar_seg
        }
    }
}

pub struct Bar<'obj> {
    bar_type: BarType,
    bar_icon: Object<'obj>,
    bar_start: Object<'obj>,
    bar_mid: Object<'obj>,
    bar_end:  Object<'obj>,
    // segments: Vec<BarSegment<'obj>>,
    bar_amt: usize,
    bar_ptr: usize,
    object: &'obj OamManaged<'obj> // reference to agb object
}

// impl<'obj> Bar<'obj> {
//     pub fn new(object: &'obj OamManaged<'obj>, bar_type: BarType, start_x: i32, start_y: i32, bar_width: usize) -> Self {
//         // todo sprite from enum based on passed in type?
//         let my_sprites: SpriteStruct = match bar_type {
//             bar::BarType::Boss_health=> {
//                 SpriteStruct{
//                     icon_sprite: ICON_SPRITE,
//                     start_sprite: START_SPRITE,
//                     end_sprite: END_SPRITE,
//                     fill_sprite: MID_FILL_SPRITE,
//                     mt_sprite: MID_MT_SPRITE
//                 }
//             },
//             bar::BarType::Char_health => {
//                 SpriteStruct{
//                     icon_sprite: MID_MT_SPRITE, 
//                     start_sprite: MID_MT_SPRITE,
//                     end_sprite: MID_MT_SPRITE,
//                     fill_sprite: MID_MT_SPRITE,
//                     mt_sprite: MID_MT_SPRITE
//                 }
//             }
//         };

//         let mut bar_icon = object.object_sprite(my_sprites.icon_sprite);
//         let mut bar_start = object.object_sprite(my_sprites.start_sprite);
//         let mut bar_end = object.object_sprite(my_sprites.end_sprite);

//         bar_icon.show();
//         bar_start.show();
//         bar_end.show();

//         let mut bar_mid = object.object_sprite(my_sprites.fill_sprite);

//         // let mut segments: Vec<BarSegment> = Vec::new();
//         // for i in 0..bar_width{
//         //     let mut tmp = BarSegment{id: i, bar_seg: object.object_sprite(my_sprites.fill_sprite)};
//         //     tmp.bar_seg.show();
//         //     segments.push(tmp);
//         // }

//         let mut new_bar = Self {
//             bar_type,
//             bar_icon,
//             bar_start,
//             bar_mid,
//             bar_end,
//             // segments,
//             bar_amt: 100, // todo where to get amount from?
//             bar_ptr: bar_width -1,
//             object
//         };

//         new_bar.set_position(start_x, start_y);
//         new_bar
//     }

//     fn set_position(&mut self, x: i32, y: i32) {
//         // todo the spacing amounts vary by bar_type. Test with just boss for now!?
//         self.bar_icon.set_position((x, y));
//         // Start is 4px wide
//         self.bar_start.set_position((x+16, y));

//         // // Each mid section is 1px
//         // let mut cnt = x + 20;
//         // for mut o in &mut self.segments{
//         //     o.bar_seg.set_position((cnt, y));
//         //     cnt += 1;
//         // }

//         self.bar_mid.set_position((x+20, y));

//         // End is 2 px wide
//         self.bar_end.set_position((x+70, y)); // 80
//     }
// }