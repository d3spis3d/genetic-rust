use std::fmt;

use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use rand::distributions::{Distribution, Uniform};

pub struct City {
    x: f64,
    y: f64,
}

impl City {
    pub fn new(x: f64, y: f64) -> City {
        City { x, y }
    }
}

#[derive(Clone)]
pub struct Path {
    fitness: f64,
    order: Vec<usize>
}

impl Path {
    pub fn breed(&self, other: &Path, city_list: &Vec<City>) -> Path {
        let order = Path::crossover_order(&self.order, &other.order);
        let fitness = Path::calculate_fitness(&order, city_list);

        Path { fitness, order }
    }

    fn crossover_order(mother: &Vec<usize>, father: &Vec<usize>) -> Vec<usize> {
        let mut rng = thread_rng();
        let crossover_point = Uniform::new(0, mother.len()).sample(&mut rng);

        let mother_dna = &mother[0..crossover_point];
        let mut father_dna: Vec<usize> = father.iter().filter_map(|d| {
            if !mother_dna.contains(d) {
                return Some(*d)
            }
            None
        }).collect();

        let mut child = Vec::new();
        child.extend_from_slice(mother_dna);
        child.append(&mut father_dna);

        child
    }

    pub fn mutate(&mut self, city_list: &Vec<City>) {
        let mut rng = thread_rng();
        let point_one = Uniform::new(0, self.order.len()).sample(&mut rng);
        let point_two = Uniform::new(0, self.order.len()).sample(&mut rng);

        self.order.swap(point_one, point_two);
        self.fitness = Path::calculate_fitness(&self.order, &city_list);
    }

    pub fn calculate_fitness(path: &Vec<usize>, city_list: &Vec<City>) -> f64 {
        let path_length = city_list.len();
        let mut cost = 0.0;
        for i in 0..path_length - 1 {
            let a = &city_list[path[i]];
            let b = &city_list[path[i + 1]];
            cost = cost + ((a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0)).sqrt();
        }

        1.0 / cost
    }
}

impl fmt::Display for Path {
    fn fmt(& self, f: &mut fmt::Formatter) -> fmt::Result {
        let string_order: Vec<String> = self.order.iter().map(|i| i.to_string()).collect();
        write!(f, "Fitness: {}, Path: {}", self.fitness, string_order.join("->"))
    }
}

pub struct Simulation {
    population: Vec<Path>,
    city_list: Vec<City>,
    max_iterations: usize,
    crossover_rate: f64,
    mutation_rate: f64,
    survival_rate: f64,
}

impl Simulation {
    pub fn new(
        initial_population: Vec<Path>,
        city_list: Vec<City>,
        max_iterations: usize,
        crossover_rate: f64,
        mutation_rate: f64,
        survival_rate: f64,
    ) -> Simulation {
        Simulation {
            population: initial_population,
            city_list,
            max_iterations,
            crossover_rate,
            mutation_rate,
            survival_rate,
        }
    }

    pub fn run(&mut self) -> () {
        let mut fittest = self.find_fittest();
        println!("starting iterations");
        // print_generation(&self.population);

        for _ in 0..self.max_iterations {
            self.generate_next_generation();

            let challenger = self.find_fittest();

            // println!("challenger {} Fittest {}", challenger.fitness, fittest.fitness);
            if challenger.fitness > fittest.fitness {
                fittest = challenger;
            }

            // println!("{}", fittest);
            // print_generation(&self.population);
        }
    }

    fn find_fittest(&self) -> Path {
        let mut fittest = &self.population[0];

        for i in 1..self.population.len() {
            let p = &self.population[i];
            if p.fitness > fittest.fitness {
                fittest = p;
            }
        }

        return fittest.clone();
    }

    fn generate_next_generation(&mut self) {
        self.population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

        let breeding_count = (self.population.len() as f64 * self.crossover_rate) as usize;
        let surviving_parent_count = (breeding_count as f64 * self.survival_rate) as usize;

        let mut breeding_population = Vec::new();
        breeding_population.extend_from_slice(&self.population[0..breeding_count]);

        let mut offspring = Vec::new();

        let mut rng = thread_rng();
        let pcnt_range = Uniform::new(0, breeding_population.len());
        for i in 0..self.population.len() - surviving_parent_count - 2 {
            let rs = pcnt_range.sample(&mut rng);
            offspring.push(
                breeding_population[i % breeding_population.len()].breed(
                    &breeding_population[rs],
                    &self.city_list
                )
            );
        }

        let mut next_generation = Vec::new();
        next_generation.extend_from_slice(&self.population[0..surviving_parent_count]);
        next_generation.append(&mut offspring);
        next_generation.extend_from_slice(&self.population[self.population.len() - 2..self.population.len()]);

        assert!(next_generation.len() == self.population.len());

        for p in 0..next_generation.len() {
            if thread_rng().gen_bool(self.mutation_rate) {
                next_generation[p].mutate(&self.city_list);
            }
        }

        self.population = next_generation;
    }
}

fn initial_population(city_list: &Vec<City>, population_count: usize) -> Vec<Path> {
    let base_list: Vec<usize> = (0..city_list.len()).collect();
    let mut population = Vec::new();

    for _ in 0..population_count {
        let mut p = base_list.clone();
        let mut rng = thread_rng();
        p.shuffle(&mut rng);
        let fitness = Path::calculate_fitness(&p, city_list);

        population.push(Path { fitness, order: p });
    }

    population
}

fn print_generation(generation: &Vec<Path>) {
    println!("Generation");
    for p in 0..10 {
        println!("{}", generation[p]);
    }
}

fn main() {
    let cities = vec![
        City::new(1.0, 3.0),
        City::new(1.0, 2.0),
        City::new(1.0, 1.0),
        City::new(4.0, 3.0),
        City::new(2.0, 1.0),
        City::new(3.0, 3.0),
        City::new(3.0, 2.0),
        City::new(3.0, 1.0),
        City::new(4.0, 4.0),
    ];

    let init = initial_population(&cities, 100);

    let mut sim = Simulation::new(init, cities, 100, 0.8, 0.001, 0.2);
    sim.run();
}
