extern crate piston_window;
extern crate rand;
extern crate piston2dtemplate;

mod pong;
mod sprites;

use piston_window::*;
use pong::Pong;
use piston2dtemplate::{Screen, run};

const DARKGRAY: [f32; 4] = [0.1, 0.1, 0.1, 1.0];

/// Pong game simulation
/// Winner is best ouf of 5 games
/// Player 1 keys : Q and A
/// Player 2 keys : UP and DOWN
fn main() {

    // Create a Screen
    let screen = Screen::default();
    
    // Create a Piston Window.
    let mut window : PistonWindow = WindowSettings::new("Pong", screen.as_array())
        .fullscreen(false)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut glyphs = window.load_font("./assets/data-unifon.ttf").unwrap();
    
    // Create a new game
    let mut pong = Pong::new( screen );

    run( window, &mut pong, &mut glyphs, DARKGRAY);

}