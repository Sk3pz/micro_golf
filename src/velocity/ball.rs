use crate::velocity::Velocity;
use ggez::mint::Point2;
use glam::Vec2;
use crate::{WINDOW_WIDTH, WINDOW_HEIGHT};
use ggez::graphics::window;

pub struct Ball {
    // the current position of the ball
    pub pos: Vec2,
    // velocity of the ball, where x is the speed and y is the direction (as an angle)
    pub velocity: Velocity, // TODO: can convert to an X velocity and a Y Velocity?
}

impl Ball {

    pub fn new() -> Self {
        Self {
            pos: Vec2::ZERO,
            velocity: Velocity::ZERO
        }
    }

    pub fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }

    pub fn hit(&mut self, power: f32, angle: f32) {
        println!("Hit angle: {}", angle);
        println!("Hit power: {}", power);
        self.velocity = Velocity::new(power, angle);
    }

    pub fn update(&mut self, seconds: f32, center: Vec2, window_size: Vec2) {
        let speed = self.velocity.speed * seconds;
        self.pos.x += (self.velocity.direction.sin() * speed);
        self.pos.y += (self.velocity.direction.cos() * speed);

        // let friction = 0.5f32;
        // self.velocity.friction(friction);

        // collision detection / bouncing
        // detect if it is colliding with a wall
        // todo: not working :(
        let x_left = self.pos.x;
        let y_top = self.pos.y;
        let x_right = self.pos.x + center.x;
        let y_bottom = self.pos.y + center.y;

        let left = x_left <= 0.;
        let right = x_right >= window_size.x;
        let top = y_top <= 0.;
        let bottom = y_bottom >= window_size.y;

        /***
        * a => bounce angle
        * w => wall angle
        * b => ball angle
        * a = 2w - b
        ***/

        // Collision with walls
        if top || bottom {
            self.velocity.direction = 2. * 360. - self.velocity.direction;
        }
        if left || right {
            self.velocity.direction = 2. * 180. - self.velocity.direction;
        }

        // TODO: Collision with other objects
    }

}