// src/lib.rs

// Common imports for both native and WASM
use rand::Rng;

// Structs and Enums for the core game logic.
// These are public so they can be used by the native executable.
// The `Clone`, `Copy`, `PartialEq`, and `Debug` traits are useful for both targets.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub struct Snake {
    pub body: Vec<Point>,
    pub direction: Direction,
}

impl Snake {
    pub fn new(start_pos: Point, direction: Direction) -> Snake {
        Snake {
            body: vec![start_pos],
            direction,
        }
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        let is_opposite = match (&self.direction, new_direction) {
            (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => true,
            (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => true,
            _ => false,
        };

        if !is_opposite {
            self.direction = new_direction;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    pub width: i32,
    pub height: i32,
    pub snake: Snake,
    pub food: Point,
    pub score: u32,
    pub game_over: bool,
    pub game_started: bool, // New field
    rng: rand::rngs::ThreadRng, // Random number generator
}

// Core game logic, platform-agnostic
impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        let start_pos = Point { x: width / 2, y: height / 2 };
        let snake = Snake::new(start_pos, Direction::Right);
        let rng = rand::thread_rng();

        let mut game = Game {
            width,
            height,
            snake,
            food: Point { x: 0, y: 0 }, // Temporary position
            score: 0,
            game_over: false,
            game_started: false, // Initialize as false
            rng,
        };
        game.spawn_food();
        game
    }

    // Now uses the `rand` crate
    fn spawn_food(&mut self) {
        loop {
            let x = self.rng.gen_range(0..self.width);
            let y = self.rng.gen_range(0..self.height);
            let new_food_pos = Point { x, y };
            if !self.snake.body.iter().any(|p| *p == new_food_pos) {
                self.food = new_food_pos;
                break;
            }
        }
    }

    pub fn tick(&mut self) {
        // Only tick if the game is started and not over
        if !self.game_started || self.game_over {
            return;
        }

        let mut new_head = self.snake.body[0];
        match self.snake.direction {
            Direction::Up => new_head.y -= 1,
            Direction::Down => new_head.y += 1,
            Direction::Left => new_head.x -= 1,
            Direction::Right => new_head.x += 1,
        }

        // Wall collision
        if new_head.x < 0 || new_head.x >= self.width || new_head.y < 0 || new_head.y >= self.height {
            self.game_over = true;
            return;
        }

        // Self collision
        if self.snake.body.iter().skip(1).any(|p| *p == new_head) {
            self.game_over = true;
            return;
        }

        self.snake.body.insert(0, new_head);

        if new_head == self.food {
            self.score += 1;
            self.spawn_food();
        } else {
            self.snake.body.pop();
        }
    }
    
    // This is a core logic function, not tied to wasm
    pub fn change_snake_direction(&mut self, direction: Direction) {
        self.snake.change_direction(direction);
    }

    // New method to start/restart the game
    pub fn start_game(&mut self) {
        self.snake = Snake::new(
            Point { x: self.width / 2, y: self.height / 2 },
            Direction::Right,
        );
        self.score = 0;
        self.game_over = false;
        self.game_started = true;
        self.spawn_food();
    }
}


// This block of code will only be compiled for the wasm32 target
#[cfg(target_arch = "wasm32")]
pub mod wasm {
    use super::*; // Import everything from the parent module
    use wasm_bindgen::prelude::*;

    // This is a wrapper around the main `Game` struct that will be exposed to JS.
    #[wasm_bindgen(js_name = Game)]
    pub struct WasmGame(Game);

    #[wasm_bindgen(js_class = Game)]
    impl WasmGame {
        #[wasm_bindgen(constructor)]
        pub fn new(width: i32, height: i32) -> WasmGame {
            WasmGame(Game::new(width, height))
        }

        pub fn tick(&mut self) {
            self.0.tick();
        }

        #[wasm_bindgen(js_name = change_snake_direction)]
        pub fn change_snake_direction(&mut self, direction: WasmDirection) {
            self.0.change_snake_direction(direction.into());
        }

        // New method to expose to JS
        #[wasm_bindgen(js_name = start_game)]
        pub fn start_game(&mut self) {
            self.0.start_game();
        }

        // Getters that return copies of data
        pub fn width(&self) -> i32 { self.0.width }
        pub fn height(&self) -> i32 { self.0.height }
        pub fn food(&self) -> WasmPoint { self.0.food.into() }
        pub fn score(&self) -> u32 { self.0.score }
        #[wasm_bindgen(js_name = game_over)]
        pub fn game_over(&self) -> bool { self.0.game_over }
        #[wasm_bindgen(js_name = game_started)] // Expose new field
        pub fn game_started(&self) -> bool { self.0.game_started }

        // Functions to get pointers for efficient memory reading from JS
        #[wasm_bindgen(js_name = get_body_ptr)]
        pub fn get_body_ptr(&self) -> *const Point {
            self.0.snake.body.as_ptr()
        }

        #[wasm_bindgen(js_name = get_body_len)]
        pub fn get_body_len(&self) -> usize {
            self.0.snake.body.len()
        }
    }

    // We need to create wasm-bindgen compatible versions of our enums and structs
    // because the original ones are now pure Rust.
    #[wasm_bindgen(js_name = Direction)]
    #[derive(Clone, Copy)]
    pub enum WasmDirection {
        Up,
        Down,
        Left,
        Right,
    }

    // Conversion from WasmDirection to the core Direction
    impl From<WasmDirection> for Direction {
        fn from(d: WasmDirection) -> Self {
            match d {
                WasmDirection::Up => Direction::Up,
                WasmDirection::Down => Direction::Down,
                WasmDirection::Left => Direction::Left,
                WasmDirection::Right => Direction::Right,
            }
        }
    }
    
    #[wasm_bindgen(js_name = Point)]
    #[derive(Clone, Copy)]
    pub struct WasmPoint {
        pub x: i32,
        pub y: i32,
    }

    // Conversion from core Point to WasmPoint
    impl From<Point> for WasmPoint {
        fn from(p: Point) -> Self {
            WasmPoint { x: p.x, y: p.y }
        }
    }
}