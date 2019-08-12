use piston_window::*;
use piston2dtemplate::{Sprite2d, Screen};
 
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub struct Score {
    pub score1: u32,
    pub score2: u32,
    pub screen_width: f64
}

impl Sprite2d for Score {

    fn render(&self, glyphs: &mut Glyphs, c: &Context, g: &mut G2d) {
        text(WHITE, 24, format!("{:.0}",self.score1).as_str(), glyphs, c.transform.trans(self.screen_width / 4.0,30.0), g).expect("Text rendering failed");
        text(WHITE, 24, format!("{:.0}",self.score2).as_str(), glyphs, c.transform.trans(3.0 * self.screen_width / 4.0,30.0), g).expect("Text rendering failed");
    }

}

impl Score {

    pub fn new(screen: &Screen) -> Score {
        Score{
            score1: 0,
            score2: 0,
            screen_width: screen.width
        }
    }

    pub fn score1increment(&mut self) -> bool {
        self.score1 += 1;
        if self.score1 == 5 { true } else { false }
    }

    pub fn score2increment(&mut self) -> bool {
        self.score2 += 1;
        if self.score2 == 5 { true } else { false }
    }

}