use macroquad::{prelude::Color, color_u8};

pub const BACKGROUND_COLOR: Color = color_u8!(51, 45, 56, 255);
pub const CELL_SIZE: f32 = 16.0;
pub const MOVEMENT_STEP: f32 = 0.03;
pub const WINDOW_WIDTH: i32 = CELL_SIZE as i32 * CELL_AMOUNT_X;
pub const WINDOW_HEIGHT: i32 = CELL_SIZE as i32 * CELL_AMOUNT_Y;
pub const CELL_AMOUNT_X: i32 = 48;
pub const CELL_AMOUNT_Y: i32 = 48;