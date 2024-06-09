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
mod banner;
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

use agb::{display::object::{Graphics, Tag}, include_aseprite, println, input::Button, include_background_gfx, rng, include_font};
use agb::display::object::{ChangeColour, DynamicSprite, OamManaged, Object, PaletteVram, Size, Sprite, TextAlignment};
use agb::{
    display::{
        tiled::{RegularBackgroundSize, TileFormat, TileSet, TileSetting, Tiled0, VRamManager},
        Priority,
        Font
    }
};
use agb::display::palette16::Palette16;
use agb::display::tiled::{MapLoan, RegularMap, TiledMap};
use agb::fixnum::{Num, num, Vector2D};
use agb::input::ButtonController;
use core::fmt::Write;
use agb::display::{HEIGHT, WIDTH};
use crate::background::{show_dungeon_screen, show_splash_screen, tear_down_dungeon_screen, show_game_over_screen};
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
static HOURGLASS_SPRITE_TAG: &Tag = GRAPHICS.tags().get("hourglass");
static LEAF_SPRITE_TAG: &Tag = GRAPHICS.tags().get("leaf");
static CAUTERIZE_SPRITE_TAG: &Tag = GRAPHICS.tags().get("cauterize");

static FONT: Font = include_font!("fonts/font.ttf", 8);
static BOXY_FONT: Font = include_font!("fonts/boxy.ttf", 8);

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
        let mut bg = show_dungeon_screen(&mut vram, &tiled, true);

        // Players
        let chars_effects_pos = [(8, 12), (8, 76), (80, 12), (80, 76)];
        let wizard = Character::new(&object, chars_effects_pos[0],Profession::Wizard, 1);
        let healer = Character::new(&object, chars_effects_pos[1], Profession::Healer, 0);
        let tank = Character::new(&object, chars_effects_pos[2], Profession::Tank, 0);
        let barb = Character::new(&object, chars_effects_pos[3], Profession::Barb, 1);

        let mut chars = [wizard, healer, tank, barb];
        // let chars_effects_pos = [(24, 28), (24, 92), (96, 28), (96, 92)]; // todo probably minus 16 from each? or invert and put above char creation with a +16

        let mut dps = chars.iter().map(|c| c.dps).sum::<usize>();
        // chars.iter_mut().for_each(Character::show);

        // todo Show bottom banner and initial "story" text. No spell text yet
        // dungeon.aseprite without the health bars on it for showing with text
        println!("Show blank dungeon for dialog");

        let mut renderer = BOXY_FONT.render_text((3u16, 17u16));

        // Renders 2 lines at a time.
        let strings = ["Last time this is", "the boss that wiped us.", "Healz, you better be", "on your A game!", "Is everyone ready?", ""];
        let mut i = 0;

        let vblank = agb::interrupt::VBlank::get();

        loop {
            input.update();

            if input.is_just_pressed(Button::A) && i < strings.len() {
                // renderer.write_char('8', &mut vram, 2,0);
                let mut writer = renderer.writer(15, 0, &mut bg, &mut vram);
                writeln!(&mut writer, "{}", strings[i]).unwrap();
                writeln!(&mut writer, "{}", strings[i+1]).unwrap();
                writer.commit();
                i += 2;
            }
            if input.is_just_pressed(Button::B) {
                break;
            }

            vblank.wait_for_vblank();
            bg.commit(&mut vram);
            renderer.clear(&mut vram);
        } // End Dialog

        println!("tear down dungeon and show one with health bars");
        tear_down_dungeon_screen(bg, &mut vram);
        bg = show_dungeon_screen(&mut vram, &tiled, false);
        for c in &mut chars {
            c.show_health();
        }

        // Frame
        let mut frame = Frame::new(&object);

        // Mana Bar
        let mut mana_bar = Bar::new(&object, BarType::Mana, 28, 87);

        // Boss
        // 280 is divisible by 35 for cooldown bar slots
        let mut boss = Boss::new(&object, 152, 48, 280);

        // buttons
        let mut but_a = object.object_sprite(BTN_A_SPRITE.sprite(0));
        let mut but_b = object.object_sprite(BTN_B_SPRITE.sprite(0));
        let mut but_l = object.object_sprite(BTN_L_SPRITE.sprite(0));
        let mut but_r = object.object_sprite(BTN_R_SPRITE.sprite(0));

        let bot_bar = HEIGHT;
        let right_side = WIDTH - 22; // 16 - 6
        but_b.set_position((6, bot_bar - 16)).show();
        but_a.set_position((right_side, bot_bar - 16)).show();
        but_l.set_position((6, bot_bar - 32)).show();
        but_r.set_position((right_side, bot_bar - 32)).show();

        // Spell effects
        let mut spell_effect = object.object_sprite(BTN_L_SPRITE.sprite(0));
        spell_effect.set_position((170, 100));//.show();

        let mut hourglass: Object = object.object_sprite(HOURGLASS_SPRITE_TAG.sprite(0));
        let mut hourglass_cauterize: Object = object.object_sprite(HOURGLASS_SPRITE_TAG.sprite(0));
        hourglass.set_position((100, 135));
        hourglass_cauterize.set_position((100, 145));
        let mut leaf: Object = object.object_sprite(LEAF_SPRITE_TAG.sprite(0));
        let mut caut: Object = object.object_sprite(CAUTERIZE_SPRITE_TAG.sprite(0));

        let mut frame_counter: usize = 0;
        let mut aoe_timer: usize = 0;
        let mut tank_hit: bool = false;

        // Cooldown fields
        let mut hot_target: usize = 0;
        let mut hot: i16 = -1;
        let mut cauterize: i16 = -1;

        // Begin game loop here
        println!("Begin game logic");
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
                mana_bar.hide_all();
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
                println!("How to proceed from here? Press button to start over");

                // Todo show the banner sprites again from a struct and try to text write over them

                loop {
                    input.update();
                    if input.is_just_pressed(Button::START | Button::SELECT )
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
                hourglass.set_sprite(object.sprite(HOURGLASS_SPRITE_TAG.animation_sprite(frame_counter)));
                boss.cooldown_bar.gain_amount(1);
            }

            if frame_counter % 10 == 0 {
                // six times per second
                if hot > 0 {
                    chars[hot_target].take_heals(1);
                    hot = hot - 1;
                }
            }

            if frame_counter % 15 == 0 {
                // 4 times per second
                if cauterize > 0 {
                    caut.set_sprite(object.sprite(CAUTERIZE_SPRITE_TAG.animation_sprite(frame_counter)));
                    cauterize -= 1;

                    if cauterize == 0 {
                        hourglass_cauterize.hide();
                        caut.hide();
                        cauterize = -1;
                    }
                }
            }

            // Damage boss based on alive dps
            if frame_counter % (60 / dps) == 0 {
                // todo take damage based on which chars are alive. if only healer, no damage...
                boss.take_damage(1);
            }

            if hot == 0 {
                println!("Hot is over");
                // todo play sound effect that hot is ready again
                hourglass.hide();
                leaf.hide();
                hot -= 1;
            }

            // Half a second
            if frame_counter % 30 == 0 {
                // Regain a point of mana from spirit/manar
                if !chars[1].is_dead {
                mana_bar.gain_amount(1);
                }

                // Damage the players
                if tank_hit && !chars[2].is_dead {
                    // Every other attack should be against the tank
                    chars[2].take_damage(3);
                } else {
                    // Damage a random character and vary the damage amount
                    let dmg = rng::gen() as usize % 2;

                    // only let boss attack alive characters
                    let mut alive = Vec::new();
                    for (i, c) in chars.iter().enumerate() {
                        if !c.is_dead {
                            alive.push(i);
                        }
                    };
                    // gives neg numbers so cast as usize!
                    let chosen = rng::gen() as usize % alive.len();
                    chars[*alive.get(chosen).unwrap()].take_damage(dmg + 1);
                }
                tank_hit = !tank_hit;
            }

            // Boss aoe barr full is 35 px wide
            if aoe_timer == boss.aoe_timer {
                // reset aoe_bar and timer
                aoe_timer = 0;
                for c in &mut chars {
                    c.take_damage(7);
                }
                boss.cooldown_bar.reset_cooldown();
            } else {
                aoe_timer += 1;
            }

            // ************* Input ************* //
            // DPAD update frame. i.e. Selected character
            // x_tri and y_tri describe with -1, 0 and 1 which way the d-pad is being pressed
            let mut left_right = input.just_pressed_x_tri() as i32;
            let up_down = input.just_pressed_y_tri() as i32;
            if left_right != 0 || up_down != 0 {
                frame.set_position(left_right, up_down);
            }

            // Todo put the spells into a if-else global_cooldown section
            // Maybe have a "Cooldown indicator" like a In center of 4 spells
            // todo create a player "class" to keep track of all user functions
            if !chars[1].is_dead {
                if input.is_just_pressed(Button::A) {
                    if mana_bar.bar_amt >= 2 {
                        // todo add a cast time meter? .5 secs
                        println!("A pressed. Cast Bandage!");
                        chars[frame.selected_char].take_heals(4);
                        mana_bar.lose_amount(2);
                    } else { println!("Out of manna bruv"); }
                } else if input.is_just_pressed(Button::B) {
                    // Cast Cauterize
                    if mana_bar.bar_amt >= 5 && cauterize <= 0 {
                        println!("B pressed Cast Cauterize!");
                        // start timer for how long spell lasts or cooldown
                        chars[frame.selected_char].take_heals(8);
                        mana_bar.lose_amount(5);
                        // todo begin ability cooldown.
                        // show hourglass. todo hide when cooldown is over
                        cauterize = 4;
                        hourglass_cauterize.set_position(chars_effects_pos[frame.selected_char]);
                        hourglass_cauterize.show();
                    } else { println!("Out of manna bruv"); }
                } else if input.is_just_pressed(Button::L) {
                    if mana_bar.bar_amt >= 4 && hot <= 0 {
                        println!("Cast Regenerate HOT!");
                        mana_bar.lose_amount(4);
                        hot_target = frame.selected_char;
                        hot = 30;
                        // Show hour glass cooldown, spawn sprite effect over chosen char and decrement
                        hourglass.show();
                        leaf.set_position(chars_effects_pos[hot_target]);
                        leaf.show();
                    }
                };

                if input.is_pressed(Button::R) {
                    // Trigger R is pressed. Hold to charge mana
                    // todo move this % check above to an above section to avoid duplicate checks
                    if frame_counter % 8 == 0 {
                        mana_bar.gain_amount(1);
                    }
                    // todo show meditation sprite
                } else {
                    // set sprite back to normal
                    chars[1].instance.set_sprite(object.sprite(chars[1].tag.sprite(0)));
                }
            }
            // else {
            //     println!("Cant cast spell when dead my dude!")
            //     // todo show a sprite/message of tank saying, "Wipe it. Healer died again..."
            // }

            // Wait for vblank, then commit the objects to the screen
            agb::display::busy_wait_for_vblank();
            object.commit();

            // We must call input.update() every frame otherwise it won't update based
            // on the actual button press state.
            input.update();
        }
    }
}
