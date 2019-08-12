use num::Complex;
use piston_window::{G2dTextureContext, Texture, TextureSettings, G2dTexture, Context, Glyphs, G2d};
use piston_window::{Button, UpdateArgs, Key};
use piston2dtemplate::{Game2d, Screen};
use image::{ImageBuffer, Rgba};

struct MandelbrotConfig {
    pub max_iterations: usize
}

impl Default for MandelbrotConfig {
    fn default() -> Self {
        Self {
            max_iterations: 64
        }
    }
}

pub struct Mandelbrot {
    screen: Screen,
    opt_texture: Option<G2dTexture>,
    texture_context: G2dTextureContext,
    zoom: f64,
    x_shift: f64,
    y_shift: f64
}

impl Mandelbrot {

    pub fn new(screen: Screen, texture_context: G2dTextureContext) -> Self {
        Self{
            opt_texture: None,
            texture_context: texture_context,
            screen: screen,
            zoom: 1f64,
            x_shift: 0f64,
            y_shift: 0f64
        }
    }

    fn get_colors(i: usize) -> [u8;4] {
        let mut colors = [[0u8;4];16];
        colors[0] = [66, 30, 15, 255];
        colors[1] = [25, 7, 26, 255];
        colors[2] = [9, 1, 47, 255];
        colors[3] = [4, 4, 73, 255];
        colors[4] = [0, 7, 100, 255];
        colors[5] = [12, 44, 138, 255];
        colors[6] = [24, 82, 177, 255];
        colors[7] = [57, 125, 209, 255];
        colors[8] = [134, 181, 229, 255];
        colors[9] = [211, 236, 248, 255];
        colors[10] = [241, 233, 191, 255];
        colors[11] = [248, 201, 95, 255];
        colors[12] = [255, 170, 0, 255];
        colors[13] = [204, 128, 0, 255];
        colors[14] = [153, 87, 0, 255];
        colors[15] = [106, 52, 3, 255];
        colors[i]
    }

}

impl Game2d for Mandelbrot {

    fn render(&mut self, _: &mut Glyphs, ctx: &Context, gfx: &mut G2d) {
        piston_window::image(self.opt_texture.as_ref().unwrap(), ctx.transform, gfx);
    }

    fn update(&mut self, _: &UpdateArgs) {}

    fn reset(&mut self) {}

    fn press(&mut self, btn: &Button) {
        if let &Button::Keyboard(key) = btn {
            match key {
                Key::PageUp => {
                    self.zoom /= 2.0;
                    if self.zoom < 1.0 {
                        self.zoom = 1.0;
                    }
                },                
                Key::PageDown => {
                    self.zoom *= 2.0
                },
                Key::Up => {
                    self.y_shift -= 1.0 / self.zoom;
                },
                Key::Down => {
                    self.y_shift += 1.0 / self.zoom;
                },
                Key::Right => {
                    self.x_shift += 1.0 / self.zoom;
                },
                Key::Left => {
                    self.x_shift -= 1.0 / self.zoom;
                },
                _ => () 
            }
        }
    }

    fn release(&mut self, btn: &Button) {
        if let &Button::Keyboard(key) = btn {
            match key {
                Key::PageUp | Key::PageDown | Key::Up | Key::Down | Key::Right | Key ::Left => {
                    self.generate_texture();
                },
                _ => ()
            }
        }
    }

}

impl Mandelbrot {

    pub fn generate_texture(&mut self) {
        let config = MandelbrotConfig::default();
        
        let width_u = self.screen.width as usize;
        let height_u = self.screen.height as usize;
        
        let mut image_buffer = ImageBuffer::new(width_u as u32, height_u as u32);

        let (x_0, y_0) = (-0.5 + self.x_shift, 0.0 + self.y_shift);
        let (x_step, y_step) = ( 3.0 / (self.screen.width * self.zoom), 3.0 / (self.screen.height * self.zoom) );
        let x_min = x_0 - x_step * self.screen.width / 2.0;
        let y_min = y_0 - y_step * self.screen.height / 2.0;

        for (col, row, px) in image_buffer.enumerate_pixels_mut() {
            
            let cx = (col as f64) * x_step + x_min;
            let cy = (row as f64) * y_step + y_min;

            let c = Complex::new(cx as f32, cy as f32);
            let mut z = Complex::new(0.0, 0.0);
            
            let mut i = 0;
            for t in 0..config.max_iterations {
                if z.norm() > 2.0 {
                    break
                }
                z = z.powf(2f32) + c;
                i = t;
            }

            if i == 0 || i == config.max_iterations - 1 {
                *px = Rgba([7,7,7,255]);
            } else{
                let j = i % 16;
                *px = Rgba( Self::get_colors(j) );
            }
        }

        let texture : G2dTexture = Texture::from_image(
            &mut self.texture_context,
            &image_buffer,
            &TextureSettings::new()
        ).unwrap();

        self.opt_texture = Some( texture );
    }

}