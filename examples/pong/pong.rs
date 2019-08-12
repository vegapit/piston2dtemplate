use piston_window::*;
use std::process;
use crate::sprites::{Ball, LeftPaddle, RightPaddle, Score, Divider};
use piston2dtemplate::{Sprite2d, Game2d, Screen};
use std::{thread, time};

struct PongConfig {
    pub ball_size: f64,
    pub initial_ball_speed: f64,
    pub paddle_height: f64,
    pub paddle_width: f64,
    pub paddle_speed: f64
}

impl Default for PongConfig {

    fn default() -> Self {
        Self {
            ball_size: 15f64,
            initial_ball_speed: 300f64,
            paddle_height: 100f64,
            paddle_width: 20f64,
            paddle_speed: 300f64
        }
    }

}

pub struct Pong {
    ball: Ball,
    lpaddle: LeftPaddle,
    rpaddle: RightPaddle,
    score: Score,
    divider: Divider,
    screen: Screen
}

impl Pong {

    pub fn new(screen: Screen) -> Self {
        let config = PongConfig::default();
        Self{
            ball: Ball::new(&screen,config.initial_ball_speed,config.ball_size),
            rpaddle: RightPaddle::new(&screen,config.paddle_height,config.paddle_width),
            lpaddle: LeftPaddle::new(&screen,config.paddle_height,config.paddle_width),
            score: Score::new(&screen),
            divider: Divider::new(&screen),
            screen: screen
        }
    }

}

impl Game2d for Pong {

    fn render(&mut self, glyphs: &mut Glyphs, ctx: &Context, gfx: &mut G2d) {
        self.lpaddle.render(glyphs, ctx, gfx);
        self.rpaddle.render(glyphs, ctx, gfx);
        self.ball.render(glyphs, ctx, gfx);
        self.divider.render(glyphs, ctx, gfx);
        self.score.render(glyphs, ctx, gfx);
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.rpaddle.update(args.dt);
        self.lpaddle.update(args.dt);
        match self.ball.update(args.dt, &self.lpaddle, &self.rpaddle) {
            2 => {
                if self.score.score2increment() {
                    println!("Player 2 wins!");
                    println!("Final Score: {} - {}", self.score.score1, self.score.score2);
                    thread::sleep( time::Duration::from_millis(3000) );
                    process::exit(0);
                } else {
                    self.reset();
                }
            },
            1 => {
                if self.score.score1increment() {
                    println!("Player 1 wins!");
                    println!("Final Score: {} - {}", self.score.score1, self.score.score2);
                    thread::sleep( time::Duration::from_millis(3000) );
                    process::exit(0);
                } else {
                    self.reset();
                }
            },
            _ => {},
        }
    }

    fn reset(&mut self) {
        let config = PongConfig::default();
        self.ball = Ball::new(&self.screen,config.initial_ball_speed,config.ball_size);
        self.rpaddle = RightPaddle::new(&self.screen,config.paddle_height,config.paddle_width);
        self.lpaddle = LeftPaddle::new(&self.screen,config.paddle_height,config.paddle_width);
    }

    fn press(&mut self, btn: &Button){
        let config = PongConfig::default();
        if let &Button::Keyboard(key) = btn {
            match key {
                Key::Up => self.rpaddle.speed = -config.paddle_speed,
                Key::Down => self.rpaddle.speed = config.paddle_speed,
                Key::Q => self.lpaddle.speed = -config.paddle_speed,
                Key::A => self.lpaddle.speed = config.paddle_speed,
                _ => () 
            }
        }
    }

    fn release(&mut self, btn: &Button){
        if let &Button::Keyboard(key) = btn {
            match key {
                Key::Up => self.rpaddle.speed = 0.0,
                Key::Down => self.rpaddle.speed = 0.0,
                Key::Q => self.lpaddle.speed = 0.0,
                Key::A => self.lpaddle.speed = 0.0,
                _ => () 
            }
        }
    }
}