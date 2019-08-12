use piston_window::*;
use piston2dtemplate::{Sprite2d, Screen};

const LIGHTGRAY: [f32; 4] = [0.4, 0.4, 0.4, 1.0];

pub struct Divider{
    pub screen_width: f64,
    pub screen_height: f64
}

impl Sprite2d for Divider {
    
    fn render(&self, _: &mut Glyphs, c: &Context, g: &mut G2d) {
        let shape = rectangle::rectangle_by_corners(0.0,0.0,1.0,self.screen_height);
        rectangle(LIGHTGRAY, shape, c.transform.trans(self.screen_width / 2.0,0.0), g);
    }

}

impl Divider {

    pub fn new(screen: &Screen) -> Divider {
        Divider{
            screen_width: screen.width,
            screen_height: screen.height
        }
    }

}