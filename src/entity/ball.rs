use crate::entity::Entity;
use ggez::mint::Point2;
use glam::Vec2;
use crate::{WINDOW_WIDTH, WINDOW_HEIGHT};

pub struct Ball {
    // the current position of the ball
    pub pos: Vec2,
    // velocity of the ball, where x is the speed and y is the direction (as an angle)
    pub velocity: Vec2,
}

impl Ball {

    pub fn new() -> Self {
        Self {
            pos: Vec2::ZERO,
            velocity: Vec2::ZERO
        }
    }

    pub fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }

    pub fn hit(&mut self, power: f32, angle: f32) {
        self.velocity = Vec2::new(power, angle);
    }

    pub fn update(&mut self) {
        // TODO: Friction
    }

}

impl Entity for Ball {

}