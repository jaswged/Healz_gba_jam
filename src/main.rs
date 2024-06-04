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

mod background;
mod bar;
mod boss;
mod boss_health_bar;
mod character;
mod frame;
mod game_manager;
mod health_bar;
mod sfx;

use crate::game_manager::{GameManager, GRAPHICS};
use alloc::vec::Vec;
use frame::Frame;

use agb::{display::object::{Graphics, Tag}, include_aseprite, println, input::Button, include_background_gfx, rng};
use agb::display::object::{DynamicSprite, OamManaged, Object, PaletteVram, Size, Sprite};
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
use crate::background::{show_dungeon_screen, show_splash_screen, tear_down_dungeon_screen, show_game_over_screen};
use crate::boss_health_bar::BossHealthBar;
use crate::health_bar::HealthBar;
use crate::bar::{Bar, BarType};
use crate::boss::Boss;
use crate::character::{Character, Profession};

// We define some easy ways of referencing the sprites
// region Todo group buttons into own file
static BTN_A_SPRITE: &Tag = GRAPHICS.tags().get("A");
static BTN_B_SPRITE: &Tag = GRAPHICS.tags().get("B");
static BTN_L_SPRITE: &Tag = GRAPHICS.tags().get("L");
static BTN_R_SPRITE: &Tag = GRAPHICS.tags().get("R");
// endregion

static SKULL_SPRITE_TAG: &Tag = GRAPHICS.tags().get("skull");

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
    let mut input = ButtonController::new();
    loop {
        // Get the object manager
        let object: OamManaged = gba.display.object.get_managed();

        // Create a button controller. only allows for reading
        let (tiled, mut vram) = gba.display.video.tiled0();

        // Show title page. press to continue
        show_splash_screen(&mut input, &mut vram, &tiled);

        println!("After splash screen");

        // Background
        let mut bg = show_dungeon_screen(&mut vram, &tiled);

        // Players
        let wizard = Character::new(&object, 24, 28, Profession::Wizard, 2);
        let healer = Character::new(&object, 24, 92, Profession::Healer, 0);
        let tank = Character::new(&object, 96, 28, Profession::Tank, 1);
        let barb = Character::new(&object, 96, 92, Profession::Barb, 2);

        let mut chars = [wizard, healer, tank, barb];

        // todo Show bottom banner and initial "story" text. No spell text yet
        // dungeon.aseprite without the health bars on it for showing with text

        // Frame
        let mut frame = Frame::new(&object);

        // Boss
        let mut boss = Boss::new(&object, 152, 32);

        // buttons
        let mut but_a = object.object_sprite(BTN_A_SPRITE.sprite(0));
        let mut but_b = object.object_sprite(BTN_B_SPRITE.sprite(0));
        let mut but_l = object.object_sprite(BTN_L_SPRITE.sprite(0));
        let mut but_r = object.object_sprite(BTN_R_SPRITE.sprite(0));

        let bot_bar = agb::display::HEIGHT;
        let right_side = agb::display::WIDTH - 22; // 16 - 6
        but_b.set_position((6, bot_bar - 16)).show();
        but_a.set_position((right_side, bot_bar - 16)).show();
        but_l.set_position((6, bot_bar - 32)).show();
        but_r.set_position((right_side, bot_bar - 32)).show();

        // Spell effects
        let mut spell_effect = object.object_sprite(BTN_L_SPRITE.sprite(0));
        spell_effect.set_position((170, 100)).show();

        let skull_sprite_zero: &Sprite = SKULL_SPRITE_TAG.sprite(0);
        let btna_sprite_zero: &Sprite = BTN_A_SPRITE.sprite(0);

        let mut left_right = 0;
        let mut up_down = 0;
        let mut frame_counter: usize = 0;

        let mut skull_hidden: bool = false;

        // Begin game loop here
        loop {
            frame_counter = frame_counter.wrapping_add(1);

            // Did anyone die last frame
            if chars.iter().all(|c| c.is_dead) {
                println!("You lose!");

                // Hide all active sprites
                // Todo create a vec with all active sprites and do one loop?
                chars.iter_mut().for_each(Character::hide);
                frame.hide();
                boss.hide();
                but_a.hide();
                but_b.hide();
                but_l.hide();
                but_r.hide();
                spell_effect.hide();
                tear_down_dungeon_screen(bg, &mut vram);
                agb::display::busy_wait_for_vblank();
                object.commit();

                show_game_over_screen(&mut input, &mut vram, &tiled);

                break; // returns you to the title screen
            };

            if boss.is_dead {
                println!("You win bruv. Good Going. Go get your Lewt!");
                // todo boss fight over

                println!("gameover loop you Won");
                // tear_down_dungeon_background(bg, &mut vram);
                // show_game_over_screen(&mut input, &mut vram, &tiled);
                loop {
                    println!("How to proceed from here?");
                    input.update();
                    if input.is_just_pressed(Button::A | Button::B | Button::START | Button::SELECT)
                    {
                        println!("Button pressed");
                        break;
                    }
                    agb::display::busy_wait_for_vblank();
                }

                // todo. spawn new boss and next room? don't break
                tear_down_dungeon_screen(bg, &mut vram);
                break; // returns you to the title screen
            }

            if frame_counter % 30 == 0 {
                println!("Is the 30th frame. Do something!");
                if skull_hidden {
                    spell_effect.set_sprite(object.sprite(btna_sprite_zero));
                    // spell_effect.show();
                } else {
                    spell_effect.set_sprite(object.sprite(skull_sprite_zero));
                    // spell_effect.hide();
                }
                skull_hidden = !skull_hidden;
                boss.take_damage(2);

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
            }

            // TOdo put the spells into a if-else global_cooldown section
            // Maybe have a "Cooldown indicator" like a In center of 4 spells
            // todo create a player "class" to keep track of all user functions
            if input.is_pressed(Button::A) {
                // todo add a cast time meter? .5 secs
                println!("A pressed. Cast Bandage!");
                chars[frame.selected_char].take_damage(2);
            } else if input.is_just_pressed(Button::B) {
                // the B button is pressed
                println!("B pressed Cast Cauterize!");
                // start timer for how long spell lasts or cooldown

                // todo begin ability cooldown.
            } else if input.is_just_pressed(Button::L) {
                // the B button is pressed
                println!("Input B pressed");
                println!("Cast Regenerate!");

                chars[frame.selected_char].take_heals(3);
                // todo begin ability cooldown and add heal over time to selected char
            } else if input.is_pressed(Button::R) {
                // the B button is pressed. Hold to charge mana
                println!("Trigger R is held");
                println!("Begin meditation!");
                chars[frame.selected_char].take_heals(1);
            }

            // Wait for vblank, then commit the objects to the screen
            agb::display::busy_wait_for_vblank();
            object.commit();

            // We must call input.update() every frame otherwise it won't update based
            // on the actual button press state.
            input.update();
        }
    }
}
