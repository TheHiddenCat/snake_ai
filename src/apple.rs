use rand::prelude::*;
use macroquad::{prelude::RED, shapes::draw_rectangle};

use crate::settings::{CELL_SIZE, CELL_AMOUNT_X, CELL_AMOUNT_Y};

#[derive(Debug, Clone)]
pub struct Apple {
    pub x: i32,
    pub y: i32,
    pub is_eaten: bool,
}

impl Apple {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(0..CELL_AMOUNT_X),
            y: rng.gen_range(0..CELL_AMOUNT_Y),
            is_eaten: false,
        }
    }

    #[inline]
    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.x as f32 * CELL_SIZE, 
            self.y as f32 * CELL_SIZE, 
            CELL_SIZE, 
            CELL_SIZE, 
            RED
        );
    }
}