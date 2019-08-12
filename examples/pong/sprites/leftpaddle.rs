use piston_window::*;
use piston2dtemplate::{Screen,Sprite2d};

const RED: [f32; 4] = [0.8, 0.0, 0.0, 1.0];

pub struct LeftPaddle{
    pub pos: f64,
    pub speed: f64,
    pub screen_height: f64,
    pub height: f64,
    pub width: f64
}

impl Sprite2d for LeftPaddle {
    
    fn render(&self, _: &mut Glyphs, ctx: &Context, g: &mut G2d) {
        let shape = rectangle::rectangle_by_corners(0.0,0.0,self.width,self.height);
        rectangle(RED, shape, ctx.transform.trans(0.0,self.pos), g);
    }

}

impl LeftPaddle{

    pub fn new(screen: &Screen, height: f64, width: f64) -> LeftPaddle {
        LeftPaddle{ 
            pos: (screen.height - height) / 2.0, 
            speed: 0.0,
            screen_height: screen.height,
            height: height,
            width: width
        }
    }
    
    pub fn update(&mut self, dt: f64) {
        if self.pos + self.speed * dt <= 0.0 {
            self.pos = 0.0;
        } else if self.pos + self.speed * dt >= self.screen_height - self.height {
            self.pos = self.screen_height - self.height;
        } else {
            self.pos += self.speed * dt;
        }
    }

}