use ggez::mint::Vector2;
use std::fmt::{Display, Formatter};

pub mod ball;
pub mod hole;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Velocity {
    pub speed: f32,
    pub direction: f32,
}

impl Velocity {
    pub const ZERO: Self = Self { speed: 0.0, direction: 0.0 };

    pub fn new(speed: f32, direction: f32) -> Self {
        Self {
            speed, direction
        }
    }

    pub fn friction(&mut self, amt: f32) {
        self.speed = if self.speed - amt < 0. {
            0.0
        } else {
            self.speed - amt
        };
    }
}

impl Display for Velocity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Velocity(speed: {}, direction: {})", self.speed, self.direction)
    }
}