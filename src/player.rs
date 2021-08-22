use crate::entity::ball::Ball;

pub struct Player {
    pub ball: Ball,
    pub current_hole: usize,
    pub strokes: usize,
}

impl Player {

    pub fn new(starting_hole: usize, starting_strokes: usize) -> Self {
        Self {
            ball: Ball::new(),
            current_hole: starting_hole,
            strokes: starting_strokes,
        }
    }

}