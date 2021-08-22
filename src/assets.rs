use ggez::graphics::{Image, Font};
use ggez::{Context, GameResult};

pub struct Assets {
    ball: Image,
    square_barrier: Image,
    font: Font,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let ball = Image::new(ctx, "/gfx/ball.png").expect("Failed to load ball asset!");
        let square_barrier = Image::new(ctx, "/gfx/ball.png").expect("Failed to load barrier asset!");

        let font = Font::new(ctx, "/fonts/PressStart2P-Regular.ttf").expect("Failed to load main game font!");

        Ok(Self {
            ball,
            square_barrier,
            font,
        })
    }
}