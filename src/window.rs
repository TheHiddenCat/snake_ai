use macroquad::prelude::Conf;

use crate::settings::{WINDOW_WIDTH, WINDOW_HEIGHT};

pub fn config() -> Conf {
    Conf {
        window_title: "Snake AI".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        icon: None,
        ..Default::default()
    }
}