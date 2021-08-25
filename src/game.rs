use crate::assets::Assets;
use crate::player::Player;
use ggez::{Context, GameResult, graphics, GameError, timer};
use ggez::event::{EventHandler, MouseButton, KeyMods, Button, GamepadId, Axis, ErrorOrigin, KeyCode};
use glam::Vec2;
use crate::DESIRED_FPS;
use ggez::graphics::Color;

pub struct MicroGolf {
    assets: Assets,
    player: Player,
    window_width: f32,
    window_height: f32,
}

impl MicroGolf {

    pub fn new(ctx: &mut Context) -> GameResult<Self> {

        let assets = Assets::new(ctx).expect("Failed to create assets");
        let player = Player::new(0, 0);

        let (width, height) = graphics::drawable_size(ctx);
        let game = Self {
            assets,
            player,
            window_width: width,
            window_height: height,
        };

        Ok(game)
    }

    pub fn hit(&mut self, ctx: &Context) {
        todo!()
    }

    pub fn new_hole(&mut self) {
        self.player.ball.pos = Vec2::new(self.window_width / 2. - 8., self.window_height - (self.window_height / 4.));
        //self.player.current_hole += 1;
        self.player.strokes = 0;
    }

    pub fn handle_collisions(&mut self, ctx: &Context) {
        todo!()
    }

    pub fn draw_player(&mut self, ctx: &mut Context) {
        let drawparams = graphics::DrawParam::new()
            .dest(self.player.ball.pos);
        graphics::draw(ctx, &self.assets.ball, drawparams);
    }

    fn distance_from_ball_to_cursor(&self, x: f32, y: f32) -> f32 {
        let ball_pos = self.player.ball.pos;
        // pythagorean's theorem! a^2 + b^2 = c^2
        let mut dx = ball_pos.x - x;
        if dx < 0.0 { dx *= -1.0 }
        let mut dy = ball_pos.x - x;
        if dy < 0.0 { dy *= -1.0 }

        let c2 = (dx * dx) + (dy * dy); // a^2 + b^2 = c^2

        c2.sqrt()
    }

    fn angle_of_ball_to_cursor(&self, x: f32, y: f32) -> f32 {
        let mouse_pos = Vec2::new(x, y);
        self.player.ball.pos.angle_between(mouse_pos)
    }

}

impl EventHandler<ggez::GameError> for MicroGolf {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);

            self.new_hole();

            // handle ball movements

            // handle collisions and stuff
            //self.handle_collisions(ctx);

            // process if ball is in the hole
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::GREEN);

        self.draw_player(ctx);
        // TODO: draw obstacles and stuff here

        let strokes_dest = Vec2::new(10.0, 10.0);
        let hole_dest = Vec2::new(10.0, 50.0);

        let strokes_str = format!("Strokes: {}", self.player.strokes);
        let hole_str = format!("Hole: {}", self.player.current_hole);
        let strokes_display = graphics::Text::new((strokes_str, self.assets.font, 32.0));
        let hole_display = graphics::Text::new((hole_str, self.assets.font, 32.0));
        graphics::draw(ctx, &strokes_display, (strokes_dest, 0.0, Color::WHITE));
        graphics::draw(ctx, &hole_display, (hole_dest, 0.0, Color::WHITE));

        graphics::present(ctx).expect("Failed to present graphics");

        timer::yield_now();
        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {

    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {

    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) {

    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods, repeat: bool) {

    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {

    }
}