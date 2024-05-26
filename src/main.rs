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

mod paddle;

use paddle::Paddle;

use agb::{
    display::object::{Graphics, Tag}, include_aseprite, input::Button
};

// Import the sprites in to this static. This holds the sprite 
// and palette data in a way that is manageable by agb.
// static GRAPHICS: &Graphics = include_aseprite!("gfx/sprites.aseprite");

// We define some easy ways of referencing the sprites
static BALL: &Tag = paddle::GRAPHICS.tags().get("Ball");

// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    // agb::no_game(gba);

    // Get the object manager
    let object = gba.display.object.get_managed();

    // Create a button controller. only allows for reading
    let mut input = agb::input::ButtonController::new();

    // Create an object with the ball sprite
    let mut ball = object.object_sprite(BALL.sprite(0));
    let mut ball2 = object.object_sprite(BALL.sprite(0));

    // Paddle
    let mut paddle_a = Paddle::new(&object, 8, 8, false); // the left paddle
    let mut paddle_b = Paddle::new(&object, 240 - 16 - 8, 8, true); // the right paddle

    // Place this at some point on the screen, (50, 50) for example
    ball.set_x(50).set_y(50).show();
    ball2.set_x(100).set_y(100).show();

    let mut ball_x = 50;
    let mut ball_y = 50;
    let mut ball2_x = 100;
    let mut ball2_y = 100;

    let mut x_velocity = 1;
    let mut y_velocity = 1;
    let mut x2_velocity = 0;
    let mut y2_velocity = 0;

    loop {
        // This will calculate the new position and enforce the position
        // of the ball remains within the screen
        ball_x = (ball_x + x_velocity).clamp(0, agb::display::WIDTH - 16); // 16 because of sprite size
        ball_y = (ball_y + y_velocity).clamp(0, agb::display::HEIGHT - 16);
        ball2_x = (ball2_x + x2_velocity).clamp(0, agb::display::WIDTH - 16); // 16 because of sprite size
        ball2_y = (ball2_y + y2_velocity).clamp(0, agb::display::HEIGHT - 16);

        // We check if the ball reaches the edge of the screen and reverse it's direction
        if ball_x == 0 || ball_x == agb::display::WIDTH - 16 {
            x_velocity = -x_velocity;
        }
        if ball_y == 0 || ball_y == agb::display::HEIGHT - 16 {
            y_velocity = -y_velocity;
        }

        // x_tri and y_tri describe with -1, 0 and 1 which way the d-pad
        // buttons are being pressed
        x2_velocity = input.x_tri() as i32;
        y2_velocity = input.y_tri() as i32;

        if input.is_just_pressed(Button::A){
            agb::println!("Input A pressed");
        }

        if input.is_pressed(Button::B) {
            // the B button is pressed
            agb::println!("Input B pressed");
        }

        ball2.set_x(ball2_x as u16).set_y(ball2_y as u16);

        // Set the position of the ball to match our new calculated position
        ball.set_x(ball_x as u16).set_y(ball_y as u16);

        // Now commit the object controller so this change is reflected on the screen.
        // This isn't how we will do this in the final version of the code, but will do
        // for this example.
        
        // Wait for vblank, then commit the objects to the screen
        agb::display::busy_wait_for_vblank();
        object.commit();
    
        // We must call input.update() every frame otherwise it won't update based
        // on the actual button press state.
        input.update();
    }
}
