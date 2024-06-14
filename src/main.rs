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
use agb::sound::mixer::Frequency;
use crate::background::{show_splash_screen, Terrain};
use crate::bar::{BarType, Bar};
use crate::boss::Boss;
use crate::boss::BossType::{Crab, Cyclops, Shield, Wizard};
use crate::character::{Character, Profession};
use crate::sfx::Sfx;

// We define some easy ways of referencing the sprites
// region Todo group buttons into own file
static BTN_A_SPRITE: &Tag = GRAPHICS.tags().get("A");
static BTN_B_SPRITE: &Tag = GRAPHICS.tags().get("B");
static BTN_L_SPRITE: &Tag = GRAPHICS.tags().get("L");
static BTN_R_SPRITE: &Tag = GRAPHICS.tags().get("R");
// endregion

static SKULL_SPRITE_TAG: &Tag = GRAPHICS.tags().get("skull");
static BALLS_SPRITE_TAG: &Tag = GRAPHICS.tags().get("loading_balls");
static CHEST_SPRITE_TAG: &Tag = GRAPHICS.tags().get("chest");
static HOURGLASS_SPRITE_TAG: &Tag = GRAPHICS.tags().get("hourglass");
static LEAF_SPRITE_TAG: &Tag = GRAPHICS.tags().get("leaf");
static CAUTERIZE_SPRITE_TAG: &Tag = GRAPHICS.tags().get("cauterize");
static LIGHT_FLASH_SPRITE_TAG: &Tag = GRAPHICS.tags().get("light_flash");

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
    // Create a button controller. only allows for reading
    let mut input = ButtonController::new();
    let mut won = false;

    // Get the object manager
    let object: OamManaged = gba.display.object.get_managed();
    let (tiled, mut vram) = gba.display.video.tiled0();

    // Sounds
    let mut mixer = gba.mixer.mixer(Frequency::Hz32768);
    // let mut mixer = gba.mixer.mixer(Frequency::Hz18157);
    // let mut mixer = gba.mixer.mixer(Frequency::Hz10512);
    mixer.enable();
    let mut sfx = Sfx::new(&mut mixer);

    // Show title page. press Enter to continue
    let mut splash_screen = tiled.background(
        Priority::P0,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp,
    );
    sfx.title_screen();
    show_splash_screen(&mut input, &mut vram, background::SplashScreen::Start, &mut sfx, &mut splash_screen);

    loop {
       // Define all backgrounds
        let mut background_terrain: MapLoan<RegularMap> = tiled.background(
            Priority::P3,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );
        let mut background_names: MapLoan<RegularMap> = tiled.background(
            Priority::P2,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );
        let mut background_ui: MapLoan<RegularMap> = tiled.background(
            Priority::P1,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );

        // setup dungeon backdrop
        background::show_background_terrain(background_terrain, &mut vram, Terrain::Dungeon);

        // setup background_names
        background::show_background_names(background_names, &mut vram);

        // Players
        let chars_effects_pos = [(8, 12), (8, 76), (80, 12), (80, 76)];
        let wizard = Character::new(&object, chars_effects_pos[0],Profession::Wizard, 1);
        let healer = Character::new(&object, chars_effects_pos[1], Profession::Healer, 0);
        let tank = Character::new(&object, chars_effects_pos[2], Profession::Tank, 0);
        let barb = Character::new(&object, chars_effects_pos[3], Profession::Barb, 1);

        let mut chars = [wizard, healer, tank, barb];
        chars.iter_mut().for_each(Character::hide_health);

        let mut dps = chars.iter().map(|c| c.dps).sum::<usize>();

        let vblank = agb::interrupt::VBlank::get();
        vblank.wait_for_vblank();
        object.commit();

        /*********************** Dialog ***********************/
        let mut frame_counter: usize = 0;
        let mut idle = true;

        loop {
            input.update();

            frame_counter = frame_counter.wrapping_add(1);

            if frame_counter % 10 == 0 {
                if idle {
                    Character::update_idle_animations(&mut chars, frame_counter / 4);
                } else {
                    Character::update_animations(&mut chars, frame_counter/ 12);
                }
            }

            if input.is_just_pressed(Button::SELECT) {
                break;
            } else if input.is_just_pressed(Button::A) {
                idle = !idle;
            }

            sfx.frame();
            vblank.wait_for_vblank();
            object.commit();
        }
/*
        let mut renderer = BOXY_FONT.render_text((3u16, 17u16));

        // Renders 2 lines at a time.
        let strings = ["Last time this is", "the boss that wiped us.", "Healz, you better be", "on your A game!", "Is everyone ready?", ""];
        let mut i = 0;

        loop {
            world_display.commit(&mut vram);
            world_display.set_visible(true);
            input.update();

            // Show idle chars during dialog
            frame_counter = frame_counter.wrapping_add(1);

            if frame_counter % 10 == 0 {
                for c in &mut chars {
                    c.update_idle_animation(frame_counter);
                }
            }

            if input.is_just_pressed(Button::A) && i < strings.len() {
                // renderer.write_char('8', &mut vram, 2,0);
                let mut writer = renderer.writer(15, 0, &mut world_display, &mut vram);
                writeln!(&mut writer, "{}", strings[i]).unwrap();
                writeln!(&mut writer, "{}", strings[i+1]).unwrap();
                writer.commit();
                i += 2;

                sfx.text_speed();
            }
            if input.is_just_pressed(Button::START) {
                break;
            }

            sfx.frame();
            vblank.wait_for_vblank();
            world_display.commit(&mut vram);
            renderer.clear(&mut vram);
        } // End Dialog

        println!("tear down dungeon and show one with health bars");
        */
        // tear_down_dungeon_screen(&mut blank_bg, &mut vram);

        // let mut world_display = show_dungeon_screen(world_display, &mut vram, false);
        // let mut bg = show_dungeon_screen_tiled(&mut vram, &tiled, false);

        // Show ui elements and populate health bars
        background::show_background_ui(&mut background_ui, &mut vram);
        for c in &mut chars {
            c.show_health();
        }

        // Frame
        let mut frame = Frame::new(&object);

        // Mana Bar
        let mut mana_bar = Bar::new(&object, BarType::Mana, 28, 87);

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
        let mut hourglass: Object = object.object_sprite(HOURGLASS_SPRITE_TAG.sprite(0));
        hourglass.set_position((100, 130));

        let mut leaf: Object = object.object_sprite(LEAF_SPRITE_TAG.sprite(0));
        let mut caut: Object = object.object_sprite(CAUTERIZE_SPRITE_TAG.sprite(0));
        let mut flash_obj: Object = object.object_sprite(LIGHT_FLASH_SPRITE_TAG.sprite(0));
        let mut balls: Object = object.object_sprite(BALLS_SPRITE_TAG.sprite(0));
        balls.set_position((107, 131));

        let mut aoe_timer: usize = 0;
        let mut tank_hit: bool = false;

        // Cooldown fields
        let mut hot_target: usize = 0;
        let mut hot: i16 = -1;
        let mut cauterize: i16 = -1;
        let mut flash: i16 = -1;

        let mut boss_ind = 0;
        // Todo tuple of (BossType, Terrain)?
        let boss_types = [Cyclops, Crab, Wizard]; // Shield
        sfx.boss();

        /************************** Main Game Loop **************************/
        'game_loop: loop {
            // Boss
            // 280 is divisible by 35 for cooldown bar slots
            if boss_ind >= boss_types.len() {
                won = true;
                mana_bar.hide_all();
                chars.iter_mut().for_each(Character::hide_health);
                frame.hide();
                vblank.wait_for_vblank();
                object.commit();
                break;
            }
            let mut boss = Boss::new(&object, boss_types[boss_ind].clone(), 152, 48, 280);
            boss_ind += 1;

            background::show_background_ui(&mut background_ui, &mut vram);
            frame.show();

            // Begin game loop here
            println!("Begin game logic");
            loop {
                // Must call input.update() every frame or it won't update based on button presses.
                input.update();
                frame_counter = frame_counter.wrapping_add(1);

                // region Game over checks
                // Game Over All characters dead
                for (i, c) in &mut chars.iter_mut().enumerate() {
                    if c.just_died {
                        sfx.player_died();
                        println!("Char died at {}. Dps before subtract {}, amount to subtract {}", i, dps, c.dps);
                        // todo this gets called twice each death...
                        if c.dps <= dps {
                            dps -= c.dps;
                        } else {
                            dps = 1;
                        }
                        c.just_died = false;
                        println!("After yo");
                        // *alive.remove(i);
                    }
                }

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
                    leaf.hide();
                    caut.hide();
                    flash_obj.hide();
                    agb::display::busy_wait_for_vblank();
                    object.commit();

                    background::hide_background_ui(&mut background_ui, &mut vram);

                    // show_game_over_screen(&mut input, &mut vram, &tiled, &mut sfx);
                    show_splash_screen(&mut input, &mut vram, background::SplashScreen::Over, &mut sfx, &mut splash_screen);

                    break 'game_loop; // returns you to the dungeon entrance or title screen
                }

                if boss.is_dead {
                    println!("You win bruv. Good Going. Go get your Lewt!");

                    // boss fight over; hide spell effects?
                    leaf.hide();
                    caut.hide();
                    flash_obj.hide();
                    hourglass.hide();
                    chars[1].stop_meditating();
                    mana_bar.hide_all();
                    frame.hide();
                    but_a.hide();
                    but_b.hide();
                    but_l.hide();
                    but_r.hide();
                    boss.hide_cooldown();
                    chars.iter_mut().for_each(Character::hide_health);
                    background::hide_background_ui(&mut background_ui, &mut vram);

                    loop {
                        // Todo show the banner sprites again from a struct and try to text write over them
                        // maybe additional dialog about rezzing or heal up for next fight

                        input.update();
                        frame_counter = frame_counter.wrapping_add(1);

                        if input.is_just_pressed(Button::START | Button::SELECT)
                        {
                            break;
                        }

                        if frame_counter % 10 == 0 {
                            Character::update_idle_animations(&mut chars, frame_counter / 4);
                        }

                        sfx.frame();
                        agb::display::busy_wait_for_vblank();
                        object.commit();
                    }

                    // todo heal up and rez characters and change background for next room
                    for c in &mut chars {
                        c.revive();
                    }
                    mana_bar.fill_bar();

                    agb::display::busy_wait_for_vblank();
                    object.commit();
                    break; // Sends you to the next boss
                } // end boss is dead
                // endregion

                // Animations
                if frame_counter % 8 == 0 {
                    hourglass.set_sprite(object.sprite(HOURGLASS_SPRITE_TAG.animation_sprite(frame_counter)));
                    boss.cooldown_bar.gain_amount(1);

                    if flash > 0 {
                        // flash_obj.set_sprite(object.sprite(LIGHT_FLASH_SPRITE_TAG.animation_sprite(frame_counter)));
                        flash_obj.set_sprite(object.sprite(LIGHT_FLASH_SPRITE_TAG.animation_sprite(flash as usize)));
                        flash -= 1;
                    }
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
                    // update char animations
                    Character::update_animations(&mut chars, frame_counter / 12);
                    boss.update(frame_counter);

                    if cauterize > 0 {
                        caut.set_sprite(object.sprite(CAUTERIZE_SPRITE_TAG.animation_sprite(frame_counter)));
                        cauterize -= 1;

                        if cauterize == 0 {
                            sfx.cauterize_ready();
                            hourglass.hide();
                            caut.hide();
                            cauterize = -1;
                        }
                    }
                }

                // Damage boss based on alive dps
                if dps != 0 && frame_counter % (60 / dps) == 0 {
                    boss.take_damage(1);
                }

                if hot == 0 {
                    sfx.hot_ready();
                    leaf.hide();
                    hot -= 1;
                }

                if flash == 0 {
                    flash_obj.hide();
                    flash -= 1;
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

                // Maybe have a "Cooldown indicator" like a In center of 4 spells
                // todo create a player "class" to keep track of all user functions
                if !chars[1].is_dead {
                    if input.is_pressed(Button::R) {
                        // Trigger R is pressed. Hold to charge mana
                        if frame_counter % 8 == 0 {
                            mana_bar.gain_amount(1);
                        }
                        // show meditation sprite
                        chars[1].start_meditating();
                    } else if input.is_just_released(Button::R) {
                        // set sprite back to normal
                        chars[1].stop_meditating();
                    } else if input.is_just_pressed(Button::A) {
                        // Cast Bandage
                        if mana_bar.bar_amt >= 2 && !chars[frame.selected_char].is_dead {
                            // todo add a cast time meter? .5 secs
                            chars[frame.selected_char].take_heals(4);
                            mana_bar.lose_amount(2);
                            flash = 3;
                            flash_obj.set_position(chars_effects_pos[frame.selected_char]);
                            flash_obj.show();
                        } else {
                            println!("Out of manna bruv or too soon");
                            sfx.player_oom();
                        }
                    } else if input.is_just_pressed(Button::B) {
                        // Cast Cauterize
                        if mana_bar.bar_amt >= 5 && cauterize <= 0 && !chars[frame.selected_char].is_dead {
                            // start timer for how long spell lasts or cooldown
                            chars[frame.selected_char].take_heals(8);
                            mana_bar.lose_amount(5);
                            // begin ability cooldown.
                            cauterize = 4;
                            caut.set_position(chars_effects_pos[frame.selected_char]);
                            caut.show();
                            hourglass.show();
                            sfx.fire_hit();
                        } else {
                            println!("Out of manna bruv or too soon");
                            sfx.player_oom();
                        }
                    } else if input.is_just_pressed(Button::L) {
                        // Cast Regenerate
                        if mana_bar.bar_amt >= 4 && hot <= 0  && !chars[frame.selected_char].is_dead {
                            mana_bar.lose_amount(4);
                            hot_target = frame.selected_char;
                            hot = 30;
                            // Show hour glass cooldown, spawn sprite effect over chosen char and decrement
                            leaf.set_position(chars_effects_pos[hot_target]);
                            leaf.show();
                        }
                        else {
                            println!("Out of manna bruv or too soon");
                            sfx.player_oom();
                        }
                    };
                }
                // else {
                //     println!("Cant cast spell when dead my dude!")
                //     // todo show a sprite/message of tank saying, "Wipe it. Healer died again..."
                // }

                // Wait for vblank, then commit the objects to the screen
                sfx.frame();
                agb::display::busy_wait_for_vblank();
                object.commit();
            }
        }

        println!("Before final show splash screen end");
        if won {
            // show_game_over_screen(&mut input, &mut vram, &tiled, &mut sfx);
            // "See you guys again next week for heroics"
            background::hide_background_ui(&mut background_ui, &mut vram);
            show_splash_screen(&mut input, &mut vram, background::SplashScreen::End, &mut sfx, &mut splash_screen);
            won = false;
        }
    }
}
