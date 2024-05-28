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

mod frame;
mod game_manager;
mod character;
mod health_bar;

use frame::Frame;
use crate::game_manager::{GameManager, GRAPHICS};

use agb::{
    display::object::{Graphics, Tag},
    include_aseprite,
    println,
    input::Button
};
use crate::health_bar::HealthBar;

// We define some easy ways of referencing the sprites
// static MAIN_SPRITE: &Tag = GRAPHICS.tags().get("main"); // TODO menu
// TODO start/select?
static BTN_A_SPRITE: &Tag = GRAPHICS.tags().get("A");
static BTN_B_SPRITE: &Tag = GRAPHICS.tags().get("B");
static BTN_L_SPRITE: &Tag = GRAPHICS.tags().get("L");
static BTN_R_SPRITE: &Tag = GRAPHICS.tags().get("R");
static SKULL_SPRITE: &Tag = GRAPHICS.tags().get("skull");
static CHAR_SPRITE: &Tag = GRAPHICS.tags().get("tankey");

// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    // Get the object manager
    let object = gba.display.object.get_managed();
    let game_manager = GameManager{
        currently_selected_char: 0
    };

    // Create a button controller. only allows for reading
    let mut input = agb::input::ButtonController::new();

    // TODO main menu and A to continue.

    // Background
    // let background: &'a mut BackgroundRegular<'b>;

    // Create an object with the button sprite
    let mut skull_ball = object.object_sprite(SKULL_SPRITE.sprite(0));
    let mut but_a = object.object_sprite(BTN_A_SPRITE.sprite(0));
    let mut but_b = object.object_sprite(BTN_B_SPRITE.sprite(0));
    let mut but_l = object.object_sprite(BTN_L_SPRITE.sprite(0));
    let mut but_r = object.object_sprite(BTN_R_SPRITE.sprite(0));

    // Players
    // Skulls pretend their players? or dead ones at least. lol
    let mut char0 = object.object_sprite(SKULL_SPRITE.sprite(0));
    char0.set_x(32).set_y(28).show();
    let mut char1 = object.object_sprite(SKULL_SPRITE.sprite(0));
    char1.set_x(32).set_y(84).show();
    let mut char2 = object.object_sprite(CHAR_SPRITE.sprite(0));
    char2.set_x(64).set_y(8).show();
    let mut char3 = object.object_sprite(SKULL_SPRITE.sprite(0));
    char3.set_x(96).set_y(84).show();

    // Frame
    let mut frame = Frame::new(&object, 0, 0);

    // Boss Health Bar
    // Todo: put this as an attribute on a char/boss entity with Health and such
    let mut bhp = HealthBar::new(&object, 144, 16);

    // Bottom bar
    skull_ball.set_x(170).set_y(125).show();
    let bot_bar = agb::display::HEIGHT as u16;
    let right_side = agb::display::WIDTH as u16 - 16;
    but_a.set_x(0).set_y(bot_bar-16).show();
    but_b.set_x(right_side).set_y(bot_bar-16).show();
    but_l.set_x(0).set_y(bot_bar-32).show();
    but_r.set_x(right_side).set_y(bot_bar-32).show();

    let mut left_right = 0;
    let mut up_down = 0;

    // Begin game loop here
    loop {
        // DPAD update frame. i.e. Selected character
        // x_tri and y_tri describe with -1, 0 and 1 which way the d-pad is being pressed
        left_right = input.just_pressed_x_tri() as i32;
        up_down = input.just_pressed_y_tri() as i32;
        if left_right != 0 || up_down != 0 {
            // todo need to set the currently selected character. maybe put frame as an attr on game_manager
            frame.set_position(left_right, up_down);
        }

        // TOdo put the spells into a if-elseif block so only 1 can be hit at a time
        if input.is_pressed(Button::A){
            // todo add a cast time meter? .5 secs
            println!("Input A pressed");
            println!("Cast Bandage!");
        } else if input.is_just_pressed(Button::B) {
            // the B button is pressed
            println!("Input B pressed");
            println!("Cast Cauterize!");
            skull_ball.hide();
            // test hide sprite when button is pushed
            // todo begin ability cooldown.
        }else if input.is_just_pressed(Button::L) {
            // the B button is pressed
            println!("Input B pressed");
            println!("Cast Regenerate!");
            // todo begin ability cooldown and add heal over time to selected char
        }else if input.is_pressed(Button::R) {
            // the B button is pressed. Hold to charge mana
            println!("Trigger R is held");
            println!("Begin meditation!");
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
