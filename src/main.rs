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
mod boss;
mod boss_health_bar;
mod character;
mod frame;
mod game_manager;
mod sfx;
mod bar;

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
use crate::bar::{BarType, Bar};
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
static HOURGLASS_SPRITE: &Tag = GRAPHICS.tags().get("hourglass");

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

        // Mana Bar
        let mut mana_bar = Bar::new(&object, BarType::Mana, 28, 87);

        // Boss
        // 280 is divisible by 35 for cooldown bar slots
        let mut boss = Boss::new(&object, 152, 40, 280);

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
        spell_effect.set_position((170, 100));//.show();

        let skull_sprite_zero: &Sprite = SKULL_SPRITE_TAG.sprite(0);
        let btna_sprite_zero: &Sprite = BTN_A_SPRITE.sprite(0);
        let mut hourglass: Object = object.object_sprite(HOURGLASS_SPRITE.sprite(0));
        hourglass.set_position((90, 135));

        let mut left_right = 0;
        let mut up_down = 0;
        let mut frame_counter: usize = 0;
        let mut aoe_timer: usize = 0;

        let mut skull_hidden: bool = false;
        let mut tank_hit: bool = false;

        // Begin game loop here
        loop {
            frame_counter = frame_counter.wrapping_add(1);

            // region Game over checks
            // Game Over All characters dead
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
                mana_bar.hide_all();
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
                println!("How to proceed from here? Press button to start over");
                loop {
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
            // endregion

            // Animations
            if frame_counter % 8 == 0 {
                hourglass.set_sprite(object.sprite(HOURGLASS_SPRITE.animation_sprite(frame_counter)));
                boss.cooldown_bar.gain_amount(1);
            }

            if frame_counter & 10 == 0 {
                // hourglass.set_sprite(object.sprite(HOURGLASS_SPRITE.animation_sprite(frame_counter)));
                // boss.cooldown_bar.gain_amount(1);
                println!("every tenth");
            }

            // Half a second
            if frame_counter % 30 == 0 {
                if skull_hidden {
                    spell_effect.set_sprite(object.sprite(btna_sprite_zero));
                    // spell_effect.show();
                } else {
                    spell_effect.set_sprite(object.sprite(skull_sprite_zero));
                    // spell_effect.hide();
                }
                skull_hidden = !skull_hidden;
                boss.take_damage(1);

                // Damage the players
                if tank_hit && !chars[2].is_dead {
                    // Every other attack should be against the tank
                    chars[2].take_damage(4);
                } else {
                    // Damage a random character
                    // gives neg numbers so cast as usize!
                    // todo only let boss attack alive characters
                    let chosen = rng::gen() as usize % 4;
                    // vary the damage amount
                    let dmg = rng::gen() as usize % 2;
                    // println!("Should be 1-3 damage? {}", dmg + 1);
                    chars[chosen].take_damage(dmg + 1);
                }
                tank_hit = !tank_hit;
            }

            // Boss aoe barr full is 35 px wide
            if aoe_timer == boss.aoe_timer {
                // reset aoe_bar and timer
                aoe_timer = 0;
                for c in chars.iter_mut() {
                    c.take_damage(7)
                }
                boss.cooldown_bar.reset_cooldown();
            } else {
                aoe_timer += 1;
            }

            // ************* Input ************* //
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
            if chars[1].is_dead {
                println!("Cant cast spell when dead my dude!")
                // todo show a sprite/message of tank saying, "Wipe it. Healer died again..."
            }
            else{
                if input.is_just_pressed(Button::A) {
                    if mana_bar.bar_amt >= 5 {
                        // todo add a cast time meter? .5 secs
                        println!("A pressed. Cast Bandage!");
                        chars[frame.selected_char].take_heals(2);
                        mana_bar.lose_amount(5);
                    } else { println!("Out of manna bruv"); }
                } else if input.is_just_pressed(Button::B) {
                    if mana_bar.bar_amt >= 8 {
                        // the B button is pressed
                        println!("B pressed Cast Cauterize!");
                        // start timer for how long spell lasts or cooldown
                        chars[frame.selected_char].take_heals(8);
                        mana_bar.lose_amount(8);
                        // todo begin ability cooldown.
                        // show hourglass. todo hide when cooldown is over
                        hourglass.show();
                    } else { println!("Out of manna bruv"); }
                } else if input.is_just_pressed(Button::L) {
                    if mana_bar.bar_amt >= 3 {
                        // the B button is pressed
                        println!("Input B pressed");
                        println!("Cast Regenerate!");
                        mana_bar.lose_amount(3);
                        chars[frame.selected_char].take_heals(3);
                        // todo begin ability cooldown and add heal over time to selected char
                    } else { println!("Out of manna bruv"); }
                } else if input.is_pressed(Button::R) {
                    // the B button is pressed. Hold to charge mana
                    println!("Trigger R is held");
                    println!("Begin meditation!");
                    // todo slow down how fast mana is gained.
                    // todo move this % check above to an above section to avoid duplicate checks
                    if frame_counter % 10 == 0 {
                        mana_bar.gain_amount(1);
                    }
                }
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
