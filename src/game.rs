use macroquad::prelude::*;

use crate::{snake::Snake, settings::{BACKGROUND_COLOR, MOVEMENT_STEP, WINDOW_WIDTH, WINDOW_HEIGHT}, population::Population};

pub struct Game {
    movement_timer: f32,
    best_snake: Option<Snake>,
    population: Population
}

impl Game {
    pub fn new() -> Self {
        Game {
            movement_timer: 0.0,
            population: Population::new(2000),
            best_snake: None,
        }
    }

    pub async fn run(mut self) {
        loop {
            clear_background(BACKGROUND_COLOR);
            self.update();
            self.draw();
            next_frame().await
        }
    }

    fn draw(&self) {
        if let Some(best_snake) = self.best_snake.as_ref() {
            best_snake.draw();
            best_snake.apple.draw();
        }
        else {
            draw_text(
                "training...", 
                (WINDOW_WIDTH / 2) as f32 - 60.0, 
                (WINDOW_HEIGHT / 2) as f32, 
                32.0, 
                WHITE
            );
        }
    }
    
    fn update(&mut self) {        
        self.movement_timer += get_frame_time();

        if self.best_snake.is_none() {
            loop {
                let i = self.population.train();
                if i == 0 {
                    self.population.evaluate();
                    self.population.sort();
                    let best_snake = self.population.best();
                    println!("generation: {}", self.population.generation);
                    println!("best fitness: {}", best_snake.fitness);
                    self.best_snake = Some(best_snake);
                    break;
                }
            }
        }

        if self.movement_timer > MOVEMENT_STEP {
            if let Some(best_snake) = self.best_snake.as_mut() {
                if !best_snake.is_dead {
                    best_snake.update();
                }
                else {
                    self.best_snake = None;
                    self.population.up_generation();
                }
            }
            self.movement_timer = 0.0;
        }
    }
}

