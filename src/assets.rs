use ggez::graphics::{Image, Font};
use ggez::{Context, GameResult, graphics};

pub struct Assets {
    pub ball: graphics::Image,
    pub hole: Image,
    pub square_barrier: Image,
    pub font: Font,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let ball = Image::new(ctx, "/gfx/ball.png").expect("Failed to load ball asset!");
        let hole = Image::new(ctx, "/gfx/hole.png").expect("Failed to load hole asset!");
        let square_barrier = Image::new(ctx, "/gfx/ball.png").expect("Failed to load barrier asset!");

        let font = Font::new(ctx, "/fonts/PressStart2P-Regular.ttf").expect("Failed to load main game font!");

        Ok(Self {
            ball,
            hole,
            square_barrier,
            font,
        })
    }
}