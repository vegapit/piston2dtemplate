use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use std::f64::consts::PI;
use piston_window::*;

use super::LeftPaddle;
use super::RightPaddle;
use piston2dtemplate::{Sprite2d, Screen};

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub struct Ball{
    pub x: f64,
    pub y: f64,
    vx: f64,
    vy: f64,
    size: f64,
    screen_height: f64,
    screen_width: f64
}

impl Sprite2d for Ball {

    fn render(&self, _: &mut Glyphs, ctx: &Context, g: &mut G2d) {
        let shape = rectangle::rectangle_by_corners(0.0,0.0,self.size,self.size);
        rectangle(WHITE, shape, ctx.transform.trans(self.x,self.y), g);
    }

}

impl Ball{

    pub fn new(screen: &Screen, initial_ball_speed: f64, ball_size: f64) -> Ball {
        let mut rng = ::rand::thread_rng();
        let theta = Range::new(-0.25 * PI, 0.25 * PI).ind_sample(&mut rng);
        let mut direction : f64 = 1.0;
        if rng.next_f64() < 0.5 {
            direction *= -1.0;
        }
        Ball{ 
            x: screen.width / 2.0,
            y: screen.height / 2.0, 
            vx: direction * initial_ball_speed * theta.cos(),
            vy: direction * initial_ball_speed * theta.sin(),
            size: ball_size,
            screen_height: screen.height,
            screen_width: screen.width
        }
    }

    pub fn update(&mut self, dt: f64, lpad: &LeftPaddle, rpad: &RightPaddle) -> u32 {
        // Wall Hit
        if (self.y <= 0.0) || (self.y >= self.screen_height - self.size) {
            self.vy *= -1.0;
        }
        // Left Paddle Hit
        if self.x <= lpad.width {
            if (self.y + self.size >= lpad.pos) && (self.y <= lpad.pos + lpad.height){
                self.x = lpad.width;
                self.vx *= -1.05;
                self.vy += lpad.speed / 10.0;
            }
        }
        // Right Paddle Hit
        if self.x + self.size >= self.screen_width - rpad.width {
            if (self.y + self.size >= rpad.pos) && (self.y <= rpad.pos + rpad.height){
                self.x = self.screen_width - rpad.width - self.size;
                self.vx *= -1.05;
                self.vy += rpad.speed / 10.0;
            }
        }
        // Update position
        self.x += self.vx * dt;
        self.y += self.vy * dt;
        // Check Winner
        if self.x <= 0.0 {
            2
        }else if self.x >= self.screen_width {
            1
        }else{
            0
        }
    }

}

