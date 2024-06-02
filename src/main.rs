// Games made using `agb` are no_std which means you don't have access to the standard
// rust library. This is because the game boy advance doesn't really have an operating
// system, so most of the content of the standard library doesn't apply.
//
// Provided you haven't disabled it, agb does provide an allocator, so it is possible
// to use both the `core` and the `alloc` built in crates.
#![no_std]
// `agb` defines its own `main` function, so you must declare your game's main function
// using the #[agb::entry] proc macro. Failing to do so will cause failure in linking
// which won't be a particularly clear error message.
#![no_main]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

extern crate alloc;

mod frame;
mod game_manager;
mod character;
mod health_bar;
mod boss_health_bar;
mod sfx;
mod background;
mod bar;

use alloc::vec::Vec;
use frame::Frame;
use crate::game_manager::{GameManager, GRAPHICS};

use agb::{display::object::{Graphics, Tag}, include_aseprite, println, input::Button, include_background_gfx};
use agb::display::object::{DynamicSprite, OamManaged, PaletteVram, Size, Sprite};
use agb::{
    display::{
        tiled::{RegularBackgroundSize, TileFormat, TileSet, TileSetting, Tiled0, TiledMap, VRamManager},
        Priority,
    }
};
use agb::display::palette16::Palette16;
use agb::display::tiled::{MapLoan, RegularMap};
use agb::fixnum::{Num, num, Vector2D};
use agb::input::ButtonController;
use crate::background::{show_dungeon_background, show_splash_screen};
use crate::boss_health_bar::BossHealthBar;
use crate::health_bar::HealthBar;
use crate::bar::{Bar, BarType};

// We define some easy ways of referencing the sprites
// static MAIN_SPRITE: &Tag = GRAPHICS.tags().get("main"); // TODO menu

// TODO These are tags, not sprites
// region Todo group buttons into own file
static BTN_A_SPRITE: &Tag = GRAPHICS.tags().get("A");
static BTN_B_SPRITE: &Tag = GRAPHICS.tags().get("B");
static BTN_L_SPRITE: &Tag = GRAPHICS.tags().get("L");
static BTN_R_SPRITE: &Tag = GRAPHICS.tags().get("R");
// endregion

// region todo group characters into their file
static SKULL_SPRITE_TAG: &Tag = GRAPHICS.tags().get("skull");
static BARB_SPRITE_TAG: &Tag = GRAPHICS.tags().get("barb");
static TANKEY_SPRITE_TAG: &Tag = GRAPHICS.tags().get("tankey");
static WIZARD_SPRITE_TAG: &Tag = GRAPHICS.tags().get("wizard");
static HEALER_SPRITE_TAG: &Tag = GRAPHICS.tags().get("healer");
static BOSS_SPRITE: &Tag = GRAPHICS.tags().get("boss");
// endregion

// UI sprites
static BANNER_L_SPRITE: &Tag = GRAPHICS.tags().get("banner_l");
static BANNER_M_SPRITE: &Tag = GRAPHICS.tags().get("banner_mid");


// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    game_main(gba) // Use separate fn since agb::entry macro breaks rust analyzer
}

fn game_main(mut gba: agb::Gba) -> ! {
    // Get the object manager
    let object: OamManaged = gba.display.object.get_managed();
    let game_manager = GameManager{
        currently_selected_char: 0
    };

    // Create a button controller. only allows for reading
    let mut input = ButtonController::new();
    let (tiled, mut vram) = gba.display.video.tiled0();

    // Show title page. press to continue
    show_splash_screen(&mut input, &mut vram, &tiled);

    // Background
    // let background: &'a mut BackgroundRegular<'b>;
    show_dungeon_background(&mut vram, &tiled);

    // todo Show bottom banner and initial "story" text

    // Health bar sprites
    let hp_1_sprite: &Sprite = GRAPHICS.tags().get("hp_1").sprite(0);
    let hp_2_sprite: &Sprite = GRAPHICS.tags().get("hp_2").sprite(0);
    let hp_3_sprite: &Sprite = GRAPHICS.tags().get("hp_3").sprite(0);
    let hp_4_sprite: &Sprite = GRAPHICS.tags().get("hp_4").sprite(0);
    let hp_5_sprite: &Sprite = GRAPHICS.tags().get("hp_5").sprite(0);
    let hp_6_sprite: &Sprite = GRAPHICS.tags().get("hp_6").sprite(0);
    let hp_7_sprite: &Sprite = GRAPHICS.tags().get("hp_7").sprite(0);
    let hp_8_sprite: &Sprite = GRAPHICS.tags().get("hp_8").sprite(0);

    let mut HP_SPRITE_MAP: Vec<&Sprite> = Vec::new();
    HP_SPRITE_MAP.push(hp_8_sprite);
    HP_SPRITE_MAP.push(hp_7_sprite);
    HP_SPRITE_MAP.push(hp_6_sprite);
    HP_SPRITE_MAP.push(hp_5_sprite);
    HP_SPRITE_MAP.push(hp_4_sprite);
    HP_SPRITE_MAP.push(hp_3_sprite);
    HP_SPRITE_MAP.push(hp_2_sprite);
    HP_SPRITE_MAP.push(hp_1_sprite);

    // Spell effects
    let mut spell_effect = object.object_sprite(SKULL_SPRITE_TAG.sprite(0));
    spell_effect.set_position((170, 100));
    // spell_effect.hide();

    let skull_sprite_zero: &Sprite = SKULL_SPRITE_TAG.sprite(0);
    let btna_sprite_zero: &Sprite = BTN_A_SPRITE.sprite(0);

    // Players todo structs
    // Keep skull sprite for when dead? or tombstone or just laying flat sprite
    let mut char0 = object.object_sprite(HEALER_SPRITE_TAG.sprite(0));
    char0.set_x(32).set_y(28).show();
    let mut char1 = object.object_sprite(WIZARD_SPRITE_TAG.sprite(0));
    char1.set_x(32).set_y(92).show();
    let mut char2 = object.object_sprite(TANKEY_SPRITE_TAG.sprite(0));
    char2.set_x(96).set_y(28).show();
    let mut char3 = object.object_sprite(BARB_SPRITE_TAG.sprite(0));
    char3.set_x(96).set_y(92).show();

    // Player health bar todo put into a struct together with above?
    // todo having more than one hp bar hides all the character sprites?
    println!("Create first health bar");
    let mut hp0 = HealthBar::new(&object, &HP_SPRITE_MAP, 28, 16);
    let mut hp1 = HealthBar::new(&object, &HP_SPRITE_MAP, 28, 80);
    let mut hp2 = HealthBar::new(&object, &HP_SPRITE_MAP, 100, 16);
    let mut hp3 = HealthBar::new(&object, &HP_SPRITE_MAP, 100, 80);

    // Frame
    let mut frame = Frame::new(&object, 0, 0);

    // Boss
    let mut boss = object.object_sprite(BOSS_SPRITE.sprite(0));
    boss.set_x(152).set_y(32).show();
    // Boss Health Bar
    // Todo: put this as an attribute on a char/boss entity with Health and such
    // let mut bhp = BossHealthBar::new(&object, 152, 16);
    // let mut boss_bar =Bar::new(&object, BarType::Boss_health, 152, 16, 50);

    // buttons
    let mut but_a = object.object_sprite(BTN_A_SPRITE.sprite(0));
    let mut but_b = object.object_sprite(BTN_B_SPRITE.sprite(0));
    let mut but_l = object.object_sprite(BTN_L_SPRITE.sprite(0));
    let mut but_r = object.object_sprite(BTN_R_SPRITE.sprite(0));

    let bot_bar = agb::display::HEIGHT as u16;
    let right_side = agb::display::WIDTH as u16 - 22; // 16 - 6
    but_b.set_x(6).set_y(bot_bar-16).show();
    but_a.set_x(right_side).set_y(bot_bar-16).show();
    but_l.set_x(6).set_y(bot_bar-32).show();
    but_r.set_x(right_side).set_y(bot_bar-32).show();

    let mut left_right = 0;
    let mut up_down = 0;
    let mut timer = 0; // what is this for

    let mut skull_hidden: bool = false;

    // Begin game loop here
    loop {
        timer += 1;
        // DPAD update frame. i.e. Selected character
        // x_tri and y_tri describe with -1, 0 and 1 which way the d-pad is being pressed
        left_right = input.just_pressed_x_tri() as i32;
        up_down = input.just_pressed_y_tri() as i32;
        if left_right != 0 || up_down != 0 {
            // todo need to set the currently selected character. maybe put frame as an attr on game_manager
            frame.set_position(left_right, up_down);
        }

        // TOdo put the spells into a if-elseif block so only 1 can be hit at a time
        // Maybe have a "Cooldown indicator" like a In center of 4 spells
        // todo create a player "class" to keep track of all user functions
        if input.is_just_pressed(Button::A){
            // todo add a cast time meter? .5 secs
            println!("A pressed. Cast Bandage!");
            hp0.take_damage(2);
        } else if input.is_just_pressed(Button::B) {
            // the B button is pressed
            println!("B pressed Cast Cauterize!");
            // start timer for how long spell lasts or cooldown
            if skull_hidden {
                spell_effect.set_sprite(object.sprite(btna_sprite_zero));
                // spell_effect.show();
            } else {
                spell_effect.set_sprite(object.sprite(skull_sprite_zero));
                // spell_effect.hide();
            }
            skull_hidden = !skull_hidden;

            // todo temp play with spell_effect scale?
            // todo begin ability cooldown.
        }else if input.is_just_pressed(Button::L) {
            // the B button is pressed
            println!("Input B pressed");
            println!("Cast Regenerate!");

            // bhp.take_damage(6);
            // todo begin ability cooldown and add heal over time to selected char
        }else if input.is_just_pressed(Button::R) { // todo set back to `is_pressed`
            // the B button is pressed. Hold to charge mana
            println!("Trigger R is held");
            println!("Begin meditation!");
            // bhp.take_damage(2);
        }

        // Wait for vblank, then commit the objects to the screen
        agb::display::busy_wait_for_vblank();
        // todo may need to expose a "commit" for any struct objects? or is object shared
        object.commit();
    
        // We must call input.update() every frame otherwise it won't update based
        // on the actual button press state.
        input.update();
    }
}
