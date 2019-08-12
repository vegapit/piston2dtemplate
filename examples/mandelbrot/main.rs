extern crate num;
extern crate piston_window;
extern crate piston2dtemplate;

mod mandelbrot;

use piston_window::*;
use mandelbrot::Mandelbrot;
use piston2dtemplate::{Screen, run};

const DARKGRAY: [f32; 4] = [0.1, 0.1, 0.1, 1.0];

/// Plot the Mandelbrot set
/// Use PageUp and PageDown keys to adjust the zoom level
/// Use Arrow keys to move around the complex plane
fn main() {

    // Create a Screen
    let screen = Screen::new(1200.0,1000.0);
    
    // Create a Piston Window.
    let mut window : PistonWindow = WindowSettings::new("MandelbrotSet", screen.as_array())
        .fullscreen(false)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut glyphs = window.load_font("./assets/data-unifon.ttf").unwrap();
    
    let texture_context = window.create_texture_context();

    // Create a new game
    let mut mandelbrot = Mandelbrot::new( screen, texture_context );
    mandelbrot.generate_texture();
    
    run( window, &mut mandelbrot, &mut glyphs, DARKGRAY);

}