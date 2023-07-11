use std::collections::VecDeque;

use macroquad::{prelude::{GREEN, DARKGREEN}, shapes::draw_rectangle, rand::gen_range};
use ndarray::Array2;

use crate::{settings::{CELL_SIZE, CELL_AMOUNT_X, CELL_AMOUNT_Y}, direction::Direction, apple::Apple, brain::Brain};

#[derive(Debug, Clone)]
pub struct Snake {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
    pub tail: VecDeque<Tail>,
    pub should_grow: bool,
    pub score: u32,
    pub steps_left: u32,
    pub lifetime: u32,
    pub is_dead: bool,
    pub vision: Vec<f32>,
    pub brain: Brain,
    pub fitness: u64,
    pub replay: bool,
    pub apple: Apple,
    pub apples: VecDeque<Apple>
}

#[derive(Debug, Clone)]
pub struct Tail {
    pub x: i32,
    pub y: i32,
}


#[derive(Debug, Clone, Default)]
pub struct Ray {
    apple: f32,
    body: f32,
    wall: f32,
}

impl From<Ray> for [f32; 3] {
    fn from(value: Ray) -> Self {
        [value.apple, value.body, value.wall]
    }
}


impl Tail {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y
        }
    }
}

impl Default for Snake {
    fn default() -> Self {
        let (x, y) = (16, 16);
        Snake {
            direction: Direction::Up,
            x,
            y,
            tail: (1..=2).map(|i| Tail::new(x, y + i)).collect(),
            should_grow: false,
            score: 0,
            steps_left: 100,
            is_dead: false,
            lifetime: 0,
            vision: vec![],
            brain: Brain::new(),
            apples: VecDeque::new(),
            apple: Apple::new(),
            fitness: 0,
            replay: false,
        }
    }
}

impl Snake {
    pub fn slither(&mut self) {
        if self.should_grow {
            self.tail.push_front(Tail::new(self.x, self.y));
            self.should_grow = false;
        }
        else {
            let back = self.tail.pop_back();
            if let Some(mut back) = back {
                back.x = self.x;
                back.y = self.y;
                self.tail.push_front(back);
            }
        }
        let (x, y) = self.direction.clone().into();
        (self.x, self.y) = (self.x + x, self.y + y);
    }

    #[inline]
    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn randomize_apple(&mut self) {
        loop {
            let rx = gen_range(0, CELL_AMOUNT_X - 1);
            let ry = gen_range(0, CELL_AMOUNT_Y - 1);

            if self.collision((rx, ry), self.apple.position()) {
                continue;
            }
            
            if self.collision((rx, ry), self.position()) {
                continue;
            }
            
            if self.body_collision((rx, ry)) {
                continue;
            }

            self.apple.x = rx;
            self.apple.y = ry;

            break;
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if self.direction.inverted() != direction {
            self.direction = direction;
        }
    }

    pub fn update(&mut self) {
        self.observe();
        self.think();
        self.slither();

        if self.collision(self.position(), self.apple.position()) {
            self.should_grow = true;
            self.score += 1;
            self.steps_left += 100;
            if self.replay {
                if let Some(apple) = self.apples.pop_front() {
                    self.apple = apple;
                }
                else {
                    self.randomize_apple();
                }
            }
            else {
                self.apples.push_back(self.apple.clone());
                self.randomize_apple();
            }
        }

        if self.wall_collision(self.position()) || self.body_collision(self.position()) {
            self.is_dead = true;
        }
        else {
            self.steps_left -= 1;
        }

        if self.steps_left == 0 {
            self.is_dead = true;
        }

        // Somewhat of a compromise for the replay
        if self.is_dead && self.apples.is_empty() {
            self.apples.push_back(self.apple.clone());
        }

        self.lifetime += 1;
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.x as f32 * CELL_SIZE, 
            self.y as f32 * CELL_SIZE, 
            CELL_SIZE, 
            CELL_SIZE, 
            GREEN
        );
        self.tail
            .iter()
            .for_each(|t| draw_rectangle(
                t.x as f32 * CELL_SIZE, 
                t.y as f32 * CELL_SIZE, 
                CELL_SIZE, 
                CELL_SIZE, 
                DARKGREEN
            )
        )
    }
    
    pub fn body_collision(&self, position: (i32, i32)) -> bool {
        if self.tail.iter().any(|t| t.x == position.0 && t.y == position.1) {
            return true;
        }

        false
    }

    pub fn collision(&self, position: (i32, i32), other: (i32, i32)) -> bool {
        if position.0 == other.0 && position.1 == other.1 {
            return true
        }

        false
    }

    pub fn wall_collision(&self, position: (i32, i32)) -> bool {
        if position.0 < 0 || position.1 < 0 || position.0 == CELL_AMOUNT_X || position.1 == CELL_AMOUNT_Y  {
            return true
        }

        false
    }

    pub fn think(&mut self) {
        let input = Array2::from_shape_vec((24, 1), self.vision.clone()).unwrap();
        let decision = self.brain.forward(input);

        let mut index = 0;
        let mut max: f32 = 0.0;

        for (i, f) in decision.iter().enumerate() {
            if *f > max {
                max = *f;
                index = i;
            }
        }

        match index {
            0 => self.set_direction(Direction::Up),
            1 => self.set_direction(Direction::Down),
            2 => self.set_direction(Direction::Left),
            3 => self.set_direction(Direction::Right),
            _ => {},
        }
    }

    pub fn observe(&mut self) {
        let directions = [
            (-1, 0), (-1, -1), 
            (0, -1), (1, -1), 
            (1, 0), (1, 1),
            (0, 1), (-1, 1),
        ];

        let mut vision = Vec::with_capacity(24);
        for direction in directions {
            let ray = self.cast_ray(direction);
            vision.push(ray.apple);
            vision.push(ray.body);
            vision.push(ray.wall);
        }

        self.vision = vision;
    }

    fn cast_ray(&self, direction: (i32, i32)) -> Ray {
        let mut ray = Ray::default();
        let mut position = (self.x + direction.0, self.y + direction.1);
        let mut distance = 1.0;
        let mut found_apple = false;
        let mut found_body = false;

        loop {
            if self.collision(position, self.apple.position()) && !found_apple {
                ray.apple = 1.0;
                found_apple = true;
            }
            if self.body_collision(position) && !found_body {
                ray.body = 1.0 / distance;
                found_body = true;
            }
            if self.wall_collision(position) {
                ray.wall = 1.0 / distance;
                break;
            }

            position.0 += direction.0;
            position.1 += direction.1;

            distance += 1.0;
        }

        ray
    }

    pub fn determine_fitness(&mut self) {
        let lifetime = self.lifetime as u64;
        let score = self.score as u64;
        if self.score < 10 {
            self.fitness = (lifetime * lifetime) * score.pow(2);
        } else {
            let mut fitness = lifetime * lifetime;
            fitness *= 2_u64.pow(10);
            fitness *= score - 9;
            self.fitness = fitness;
        }
    }
}