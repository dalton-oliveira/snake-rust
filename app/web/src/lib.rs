pub mod render;
mod sprites;
pub mod utils;

extern crate js_sys;
use snake::{game::Game, types::Direction};
use wasm_bindgen::prelude::*;

extern crate web_sys;

#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub enum FrontKey {
    Up,
    Right,
    Down,
    Left,
}

#[wasm_bindgen]
pub struct Universe {
    game: Game,
    render: render::BinaryRender,
}
const WIDTH: u32 = 20;
const HEIGHT: u32 = 10;

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let mut render = render::BinaryRender::new(WIDTH, HEIGHT);
        let mut game = Game::new(&mut render, WIDTH as usize, HEIGHT as usize);
        game.add_food(&mut render);
        Universe { game, render }
    }

    pub fn key_down(&mut self, to: FrontKey) -> bool {
        let snake = &mut self.game.snake;
        return match to {
            FrontKey::Up => snake.head_to(Direction::Up),
            FrontKey::Down => snake.head_to(Direction::Down),
            FrontKey::Left => snake.head_to(Direction::Left),
            FrontKey::Right => snake.head_to(Direction::Right),
        };
    }

    pub fn tick(&mut self) {
        self.game.tick(&mut self.render);
    }
}
