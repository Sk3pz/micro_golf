use crate::assets::Assets;
use ggez::{Context, GameResult, graphics, GameError, timer};
use ggez::event::{EventHandler, MouseButton, KeyMods, Button, GamepadId, Axis, ErrorOrigin, KeyCode};
use glam::Vec2;
use crate::{DESIRED_FPS};
use ggez::graphics::{Color, DrawParam};
use std::borrow::Borrow;
use std::fmt::{Display, Formatter};
use crate::velocity::hole::Hole;
use crate::barrier::Barrier;
use crate::velocity::Velocity;
use crate::velocity::ball::Ball;
use std::f32::consts::PI;

pub struct MousePressed {
    pub left: bool,
    pub right: bool,
    pub middle: bool,
}

pub struct MicroGolf {
    assets: Assets,
    ball: Ball,
    current_hole: usize,
    strokes: usize,
    hole: Hole,
    barriers: Vec<Box<dyn Barrier>>,
    window_width: f32,
    window_height: f32,
    mouse_pos: Vec2,
    mouse_pressed: MousePressed,
    debug: bool,
    debug_step: bool,
}

impl MicroGolf {

    pub fn new(ctx: &mut Context) -> GameResult<Self> {

        let assets = Assets::new(ctx).expect("Failed to create assets");

        let (width, height) = graphics::drawable_size(ctx);
        let mut game = Self {
            assets,
            ball: Ball::new(),
            current_hole: 0,
            strokes: 0,
            hole: Hole::new(),
            barriers: Vec::new(),
            window_width: width,
            window_height: height,
            mouse_pos: Vec2::ZERO,
            mouse_pressed: MousePressed { left: false, right: false, middle: false },
            debug: false,
            debug_step: false,
        };

        game.new_hole();

        Ok(game)
    }

    pub fn new_hole(&mut self) {
        self.ball.pos = Vec2::new(self.window_width / 2. - 16., self.window_height - (self.window_height / 4.));
        self.ball.velocity = Velocity::ZERO;
        //self.player.current_hole += 1;
        self.strokes = 0;
        self.hole.pos = Vec2::new(self.window_width / 2. - 16. , self.window_height / 6.);
    }

    pub fn handle_collisions(&mut self, ctx: &Context) {
        todo!()
    }

    pub fn draw_player(&mut self, ctx: &mut Context) {
        let drawparams = graphics::DrawParam::new()
            .dest(self.ball.pos);
        graphics::draw(ctx, &self.assets.ball, drawparams);
    }
    pub fn draw_hole(&mut self, ctx: &mut Context) {
        let drawparams = graphics::DrawParam::new().dest(self.hole.pos);
        graphics::draw(ctx, &self.assets.hole, drawparams);
    }

    pub fn obstacle(&mut self, ctx: &mut Context, b: Box<dyn Barrier>) {
        // TODO
    }

    pub fn ball_center(&self) -> Vec2 {
        Vec2::new(self.ball.pos.x + 16., self.ball.pos.y + 16.)
    }

    // calculates the difference between the ball and the cursor
    fn cursor_distance(&self) -> f32 {
        let x = self.mouse_pos.x;
        let y = self.mouse_pos.y;
        let ball_pos = self.ball_center();
        // a^2 + b^2 = c^2
        let dx = ball_pos.x - x;
        let dy = ball_pos.y - y;

        let mut c2 = (dx * dx) + (dy * dy); // a^2 + b^2 = c^2
        if c2 < 0.0 { // ensure c^2 is positive because the square root can not be taken from a negative number
            c2 *= -1.;
        }
        c2.sqrt().trunc()
    }

    // Calculates the angle between the ball and the cursor
    fn ball_angle(&self) -> f32 {
        let mut delta_x = self.mouse_pos.x - self.ball_center().x;
        let mut delta_y = self.mouse_pos.y - self.ball_center().y;

        delta_x.atan2(delta_y)
        //delta_x.atan2(delta_y) * (180.0 / PI)
    }

    fn current_power(&self) -> f32 {
        self.cursor_distance()
    }

}

impl EventHandler<ggez::GameError> for MicroGolf {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dfps = if self.debug_step {
            1
        } else {
            DESIRED_FPS
        };
        while timer::check_update_time(ctx, dfps) {
            let seconds = 1.0 / (dfps as f32);

            self.ball.update(
                seconds,
                Vec2::new(self.assets.ball.dimensions().w / 2., self.assets.ball.dimensions().w / 2.),
                Vec2::new(self.window_width, self.window_height),
            );
            self.hole.update();
            //self.new_hole();

            // handle ball movements

            // handle collisions and stuff
            // self.handle_collisions(ctx);

            // process if ball is in the hole
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::new(0., 1., 0.5, 1.));

        self.draw_player(ctx);
        self.draw_hole(ctx);
        // TODO: draw obstacles and stuff here

        // Display debug information
        if self.debug {
            // display debug text
            let debug_text_color = Color::BLACK;
            let debug_text_size = 10.0;
            let distance_display = graphics::Text::new(
                (format!("Mouse distance: {} pixels", self.cursor_distance().trunc()),
                 self.assets.font, debug_text_size));
            graphics::draw(ctx, &distance_display, (Vec2::new(10.0, self.window_height - 20.0), 0.0, debug_text_color));
            let pos_display = graphics::Text::new(
                (format!("Mouse position: {}, {}", self.mouse_pos.x, self.mouse_pos.y),
                 self.assets.font, debug_text_size));
            graphics::draw(ctx, &pos_display, (Vec2::new(10.0, self.window_height - 40.0), 0.0, debug_text_color));
            let ball_pos_display = graphics::Text::new(
                (format!("Ball position: {}, {}", self.ball.pos.x, self.ball.pos.y),
                 self.assets.font, debug_text_size));
            graphics::draw(ctx, &ball_pos_display, (Vec2::new(10.0, self.window_height - 60.0), 0.0, debug_text_color));
            let ball_angle_display = graphics::Text::new(
                (format!("Ball angle: {}", self.ball_angle()),
                 self.assets.font, debug_text_size));
            graphics::draw(ctx, &ball_angle_display, (Vec2::new(10.0, self.window_height - 80.0), 0.0, debug_text_color));
            let vel_display = graphics::Text::new(
                (format!("Ball velocity: {}", self.ball.velocity),
                 self.assets.font, debug_text_size));
            graphics::draw(ctx, &vel_display, (Vec2::new(10.0, self.window_height - 100.0), 0.0, debug_text_color));
            let avel_display = graphics::Text::new(
                (format!("Ball velocity direction: {}", self.ball.velocity.direction),
                 self.assets.font, debug_text_size));
            graphics::draw(ctx, &avel_display, (Vec2::new(10.0, self.window_height - 120.0), 0.0, debug_text_color));

            let mut m;
            let mut mesh_error;
            let mut error_pos;
            if self.ball.velocity.speed == 0.0 {
                // display line to cursor
                let mut mesh = graphics::MeshBuilder::new();
                // Calculate slope of the current line to shorten it
                mesh.line(&[
                    self.mouse_pos,
                    self.ball_center()
                ], 4.0, Color::new(0.2, 0.2, 0.2, 1.0));
                m = mesh.build(ctx);
                mesh_error = graphics::Text::new((format!("Failed to build mesh"), self.assets.font, 32.0));
                error_pos = Vec2::new(10.0, 150.);
            } else {
                // display a line on the projected path
                let mut mesh = graphics::MeshBuilder::new();
                let line_length: f32 = 5.0;
                let future_pos = Vec2::new(
                    self.ball.pos.x - (self.ball.velocity.direction.sin() * (self.ball.velocity.speed * line_length)),
                    self.ball.pos.y - (self.ball.velocity.direction.cos() * (self.ball.velocity.speed * line_length)),
                );
                mesh.line(&[
                    future_pos,
                    self.ball_center()
                ], 4.0, Color::new(0.2, 0.2, 0.2, 1.0));
                m = mesh.build(ctx);
                mesh_error = graphics::Text::new((format!("Failed to build mesh"), self.assets.font, 32.0));
                error_pos = Vec2::new(10.0, 150.);
            }

            if m.is_ok() {
                graphics::draw(ctx, &m.unwrap(), DrawParam::new());
            } else {
                graphics::draw(ctx, &mesh_error, (error_pos, 0.0, Color::WHITE));
            }
        }

        // Display normal information
        let strokes_dest = Vec2::new(10.0, 10.0);
        let hole_dest = Vec2::new(10.0, 50.0);

        let strokes_str = format!("Strokes: {}", self.strokes);
        let hole_str = format!("Hole: {}", self.current_hole);
        let strokes_display = graphics::Text::new((strokes_str, self.assets.font, 32.0));
        let hole_display = graphics::Text::new((hole_str, self.assets.font, 32.0));

        let arrow_length = self.current_power().max(3.0); // TODO

        graphics::draw(ctx, &strokes_display, (strokes_dest, 0.0, Color::WHITE));
        graphics::draw(ctx, &hole_display, (hole_dest, 0.0, Color::WHITE));

        graphics::present(ctx).expect("Failed to present graphics");

        timer::yield_now();
        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        match button {
            MouseButton::Left => self.mouse_pressed.left = true,
            MouseButton::Right => self.mouse_pressed.right = true,
            MouseButton::Middle => self.mouse_pressed.middle = true,
            MouseButton::Other(_) => {}
        }
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        match button {
            MouseButton::Left => {
                self.mouse_pressed.left = false;
                self.ball.hit(self.current_power(), self.ball_angle());
            },
            MouseButton::Right => self.mouse_pressed.right = false,
            MouseButton::Middle => self.mouse_pressed.middle = false,
            MouseButton::Other(_) => {}
        }
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) {
        self.mouse_pos = Vec2::new(x.trunc(), y.trunc());
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods, repeat: bool) {
        println!("Key Pressed: {:?}", keycode);

        // TODO: Handle key presses here
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        // TODO: Handle key releases here
        match keycode {
            KeyCode::Space => {
                self.ball.hit(5.0, self.ball_angle());
            }
            KeyCode::H => {
                self.new_hole();
            }
            KeyCode::LBracket => {
                match keymods {
                    KeyMods::SHIFT => {
                        self.debug_step = !self.debug;
                    }
                    _ => {}
                }
                self.debug = !self.debug;
            }
            _ => {}
        }
    }
}