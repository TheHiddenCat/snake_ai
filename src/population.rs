use rand::Rng;

use crate::snake::Snake;


#[derive(Debug)]
pub struct Population {
    pub snakes: Vec<Snake>,
    pub size: usize, // 500
    pub generation: usize,
    pub transfer_count: usize, // 75
    pub mutation_odds: f32, // 30
    pub crossover: usize // 250
}

impl Population {
    pub fn new(size: usize) -> Self {
        let snakes = vec![Snake::default(); size];
        Population {
            size,
            snakes,
            generation: 1,
            transfer_count: 50,
            mutation_odds: 0.05,
            crossover: 100,
        }
    }

    pub fn train(&mut self) -> usize {
        let mut alive = 0;
        for snake in self.snakes.iter_mut() {
            if !snake.is_dead {
                snake.update();
                alive += 1
            }
        }
        alive
    }

    pub fn evaluate(&mut self) {
        self.snakes.iter_mut().for_each(|i| i.determine_fitness());
    }

    pub fn sort(&mut self) {
        self.snakes.sort_by_key(|s| std::cmp::Reverse(s.fitness));
    }

    pub fn best(&self) -> Snake {
        let best_snake = self.snakes.first().unwrap();
        let mut apples = best_snake.apples.clone();
        let apple = apples.pop_front().unwrap();
        Snake {
            brain: best_snake.brain.clone(),
            fitness: best_snake.fitness,
            apples,
            apple,
            replay: true,
            ..Default::default()
        }
    }

    pub fn up_generation(&mut self) {
        let mut rng = rand::thread_rng();
        let mut next_individuals = Vec::with_capacity(self.size);

        for i in 0..self.transfer_count {
            let snake = &self.snakes[i];
            let brain = snake.brain.clone();
            next_individuals.push(Snake { 
                brain,
                ..Default::default()
            });
        }
        
        for _ in 0..self.size-self.transfer_count {
            let mother = &self.snakes[rng.gen_range(0..self.crossover)];
            let father = &self.snakes[rng.gen_range(0..self.crossover)];
            let mut brain = mother.brain.crossover(&father.brain);
            brain.mutate(self.mutation_odds);
            let child = Snake { 
                brain,
                ..Default::default()
            };
            next_individuals.push(child);
        }
        
        self.generation += 1;
        self.snakes = next_individuals;
    }
}