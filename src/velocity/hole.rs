use glam::Vec2;

pub struct Hole {
    pub pos: Vec2,
}

impl Hole {
    pub fn new() -> Self {
        Self {
            pos: Vec2::ZERO,
        }
    }

    pub fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }

    pub fn update(&mut self) {
        // TODO
    }

}