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

use agb::{display::object::{Graphics, Tag}, include_aseprite, println, input::Button, include_background_gfx, rng};
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
use crate::character::{Character, Profession};

// We define some easy ways of referencing the sprites
// TODO These are tags, not sprites
// region Todo group buttons into own file
static BTN_A_SPRITE: &Tag = GRAPHICS.tags().get("A");
static BTN_B_SPRITE: &Tag = GRAPHICS.tags().get("B");
static BTN_L_SPRITE: &Tag = GRAPHICS.tags().get("L");
static BTN_R_SPRITE: &Tag = GRAPHICS.tags().get("R");
// endregion

// region todo group characters into their file
static SKULL_SPRITE_TAG: &Tag = GRAPHICS.tags().get("skull");
static BOSS_SPRITE: &Tag = GRAPHICS.tags().get("boss");
// endregion

// // UI sprites
// static BANNER_L_SPRITE: &Tag = GRAPHICS.tags().get("banner_l");
// static BANNER_M_SPRITE: &Tag = GRAPHICS.tags().get("banner_mid");

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

    // Create a button controller. only allows for reading
    let mut input = ButtonController::new();
    let (tiled, mut vram) = gba.display.video.tiled0();

    // Show title page. press to continue
    show_splash_screen(&mut input, &mut vram, &tiled);

    // Background
    // let background: &'a mut BackgroundRegular<'b>;
    show_dungeon_background(&mut vram, &tiled);

    // Players todo structs
    // Keep skull sprite for when dead? or tombstone or just laying flat sprite
    let mut wizard = Character::new(&object, 24, 28, Profession::WIZARD, 2);
    let mut healer = Character::new(&object, 24, 92, Profession::HEALER, 0);
    let mut tank = Character::new(&object, 96, 28, Profession::TANK, 1);
    let mut barb = Character::new(&object, 96, 92, Profession::BARB, 2);

    let mut chars = [wizard, healer, tank, barb];
    let mut game_manager = GameManager::new();

    // todo Show bottom banner and initial "story" text. No spell text yet
    // dungeon.aseprite without the health bars on it for showing with text

    // Frame
    let mut frame = Frame::new(&object);

    // Boss
    let mut boss = object.object_sprite(BOSS_SPRITE.sprite(0));
    boss.set_x(152).set_y(32).show();
    // Boss Health Bar
    // Todo: put this as an attribute on a char/boss entity with Health and such
    let mut bhp = BossHealthBar::new(&object, 173, 19);

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

    // Spell effects
    let mut spell_effect = object.object_sprite(BTN_L_SPRITE.sprite(0));
    spell_effect.set_position((170, 100)).show();
    // spell_effect.hide();

    let skull_sprite_zero: &Sprite = SKULL_SPRITE_TAG.sprite(0);
    let btna_sprite_zero: &Sprite = BTN_A_SPRITE.sprite(0);

    let mut left_right = 0;
    let mut up_down = 0;
    let mut frame_counter: usize = 0;

    let mut skull_hidden: bool = false;

    // Begin game loop here
    loop {
        frame_counter = frame_counter.wrapping_add(1);

        if frame_counter % 30 == 0{
            println!("Is the 30th frame. Do something!");
            if skull_hidden {
                spell_effect.set_sprite(object.sprite(btna_sprite_zero));
                // spell_effect.show();
            } else {
                spell_effect.set_sprite(object.sprite(skull_sprite_zero));
                // spell_effect.hide();
            }
            skull_hidden = !skull_hidden;
            bhp.take_damage(5);

            // Damage a random character
            let chosen = rng::gen() % 4;
            // let chosen2 = Number::from_raw( rng::gen() % 4);
            // todo ^ threw an error. didn't give 0-3 as expected
            println!("Chosen character would be {}", chosen);
            // chars[chosen as usize].health_bar.take_damage(2);
        }

        // DPAD update frame. i.e. Selected character
        // x_tri and y_tri describe with -1, 0 and 1 which way the d-pad is being pressed
        left_right = input.just_pressed_x_tri() as i32;
        up_down = input.just_pressed_y_tri() as i32;
        if left_right != 0 || up_down != 0 {
            frame.set_position(left_right, up_down);
            // Set the currently selected character. todo maybe put frame as an attr on game_manager
            // game_manager.currently_selected_char = frame.selected_char; is this needed?
        }

        // TOdo put the spells into a if-else global_cooldown section
        // Maybe have a "Cooldown indicator" like a In center of 4 spells
        // todo create a player "class" to keep track of all user functions
        if input.is_just_pressed(Button::A){
            // todo add a cast time meter? .5 secs
            println!("A pressed. Cast Bandage!");
            chars[frame.selected_char].health_bar.take_damage(2);
        } else if input.is_just_pressed(Button::B) {
            // the B button is pressed
            println!("B pressed Cast Cauterize!");
            // start timer for how long spell lasts or cooldown
            bhp.take_damage(4);

            // todo begin ability cooldown.
        }else if input.is_just_pressed(Button::L) {
            // the B button is pressed
            println!("Input B pressed");
            println!("Cast Regenerate!");

            chars[frame.selected_char].health_bar.take_heals(3);
            // todo begin ability cooldown and add heal over time to selected char
        }else if input.is_pressed(Button::R) {
            // the B button is pressed. Hold to charge mana
            println!("Trigger R is held");
            println!("Begin meditation!");
            chars[frame.selected_char].health_bar.take_heals(1);
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
