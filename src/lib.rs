extern crate piston_window;
extern crate image;

mod screen;

use piston_window::*;
pub use crate::screen::Screen;

pub trait Sprite2d {
    fn render(&self, glyphs: &mut Glyphs, ctx: &Context, gfx: &mut G2d);
}

pub trait Game2d {
    fn render(&mut self, glyphs: &mut Glyphs, ctx: &Context, gfx: &mut G2d);
    fn update(&mut self, u: &UpdateArgs);
    fn press(&mut self, btn: &Button);
    fn release(&mut self, btn: &Button);
    fn reset(&mut self);
}

pub fn run<'a, T: Game2d>(mut window: PistonWindow, game: &'a mut T, glyphs: &mut Glyphs, bgcolor: [f32;4]) {
    let eventsettings = EventSettings::new().ups(60);
    let mut events = Events::new( eventsettings );
    // Event loop
    while let Some(event) = events.next(&mut window) {
        if let Some(_) = event.render_args() {
            window.draw_2d(&event, |ctx, gfx, device| {
                // Clear the screen.
                clear(bgcolor, gfx);
                // Render all game objects
                game.render(glyphs, &ctx, gfx);
                // Update glyphs before rendering.
                glyphs.factory.encoder.flush( device );
            });
        }
        if let Some(u) = event.update_args() {
            game.update(&u);
        }
        if let Some(btn) = event.press_args() {
            game.press(&btn);
        }
        if let Some(btn) = event.release_args() {
            game.release(&btn);
        }
    }   
}