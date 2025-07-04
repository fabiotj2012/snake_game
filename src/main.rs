// src/main.rs

// This code will only be compiled for native targets, not for wasm32.
#![cfg(not(target_arch = "wasm32"))]

use ggez::{
    conf,
    event::{self, EventHandler},
    graphics::{self, Color, Rect, Text},
    input::keyboard::{KeyCode, KeyInput},
    Context, ContextBuilder, GameResult,
};

// Import the core game logic from our library
use snake_game::{Direction, Game};

const GRID_SIZE: (i32, i32) = (20, 20);
const PIXEL_SCALE: f32 = 20.0;
const FPS: u32 = 10;

// Struct to hold the application state for ggez
struct AppState {
    game: Game,
}

impl AppState {
    fn new(_ctx: &mut Context) -> AppState {
        AppState {
            game: Game::new(GRID_SIZE.0, GRID_SIZE.1),
        }
    }
}

// ggez's event handler implementation
impl EventHandler for AppState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // The game logic is ticked based on the desired FPS
        while ctx.time.check_update_time(FPS) {
            // Only tick if the game is started and not over
            if self.game.game_started && !self.game.game_over {
                self.game.tick();
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(51, 51, 51));

        // Draw the food
        let food = self.game.food;
        let food_rect = Rect::new(
            food.x as f32 * PIXEL_SCALE,
            food.y as f32 * PIXEL_SCALE,
            PIXEL_SCALE,
            PIXEL_SCALE,
        );
        canvas.draw(&graphics::Quad, graphics::DrawParam::new().dest(food_rect.point()).scale(food_rect.size()).color(Color::RED));

        // Draw the snake
        for segment in &self.game.snake.body {
            let snake_rect = Rect::new(
                segment.x as f32 * PIXEL_SCALE,
                segment.y as f32 * PIXEL_SCALE,
                PIXEL_SCALE,
                PIXEL_SCALE,
            );
            canvas.draw(&graphics::Quad, graphics::DrawParam::new().dest(snake_rect.point()).scale(snake_rect.size()).color(Color::from_rgb(0, 255, 0)));
        }

        // Draw score
        let score_text = Text::new(format!("Score: {}", self.game.score));
        canvas.draw(
            &score_text,
            graphics::DrawParam::new()
                .color(Color::WHITE)
                .dest(ggez::mint::Point2 { x: 10.0, y: 10.0 }),
        );

        // Draw start/game over message
        if !self.game.game_started || self.game.game_over {
            let message = if !self.game.game_started {
                "Press SPACE to Start".to_string()
            } else {
                format!("Game Over! Score: {}\nPress SPACE to Restart", self.game.score)
            };
            let mut text = Text::new(message);
            text.set_scale(30.0);
            let screen_width = GRID_SIZE.0 as f32 * PIXEL_SCALE;
            let screen_height = GRID_SIZE.1 as f32 * PIXEL_SCALE;
            let text_dimensions = text.measure(ctx)?;
            let x = (screen_width - text_dimensions.x) / 2.0;
            let y = (screen_height - text_dimensions.y) / 2.0;

            canvas.draw(
                &text,
                graphics::DrawParam::new()
                    .color(Color::WHITE)
                    .dest(ggez::mint::Point2 { x, y }),
            );
        }

        canvas.finish(ctx)
    }

    // The new key_down_event signature
    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeated: bool) -> GameResult {
        if let Some(keycode) = input.keycode {
            match keycode {
                KeyCode::Up => self.game.change_snake_direction(Direction::Up),
                KeyCode::Down => self.game.change_snake_direction(Direction::Down),
                KeyCode::Left => self.game.change_snake_direction(Direction::Left),
                KeyCode::Right => self.game.change_snake_direction(Direction::Right),
                KeyCode::Space => {
                    if !self.game.game_started || self.game.game_over {
                        self.game.start_game();
                    }
                }
                _ => (),
            }
        }
        Ok(())
    }
}

// Main function for the native executable
pub fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("snake_game", "Gemini")
        .window_setup(conf::WindowSetup::default().title("Snake Game (Rust Native)"))
        .window_mode(
            conf::WindowMode::default()
                .dimensions(GRID_SIZE.0 as f32 * PIXEL_SCALE, GRID_SIZE.1 as f32 * PIXEL_SCALE),
        )
        .build()?;

    let state = AppState::new(&mut ctx);
    event::run(ctx, event_loop, state)
}