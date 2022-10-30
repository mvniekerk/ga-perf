// use rand::{seq::SliceRandom, Rng};

extern crate core;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;

// tuneable parameters
// TODO: extract these to somewhere nicer
const NUM_POINTS: usize = 256;
const MAX_POPULATION: usize = 1000;
const NUM_GENERATIONS: i32 = 50;
const NUM_SURVIVORS: usize = 100;

type Point = (f64, f64);
type Population = [[usize; NUM_POINTS]; MAX_POPULATION];
type World = [Point; NUM_POINTS];

fn main() {
    // set up world
    let world: World = core::array::from_fn(|_| (fastrand::f64(), fastrand::f64()));

    let start = std::time::Instant::now();
    // let mut rng = StepRng::new(2, 13);
    // let mut rng = StepRng::new(2, 13);
    let mut rng = rand::thread_rng();

    // set up the initial population
    let mut population: Population = core::array::from_fn(|_| new_random_agent(&mut rng));

    for _ in 0..NUM_GENERATIONS {
        eval_and_sort(&mut population, &world);

        // determine the set of survivors
        let survivors = &population[0..NUM_SURVIVORS as usize];

        population = core::array::from_fn(|_| new_agent_from_parents(survivors, &mut rng));
    }

    eval_and_sort(&mut population, &world);

    let finish = std::time::Instant::now();
    let elapsed = finish.duration_since(start);

    println!("Best score: {:?}", eval(&population[0], &world));

    println!("Elapsed: {:?}", elapsed);
}

fn new_random_agent(rng: &mut ThreadRng) -> [usize; NUM_POINTS]
{
    let mut agent: [usize; NUM_POINTS] = core::array::from_fn(|i| i);
    agent.shuffle(rng);
    agent
}

fn new_agent_from_parents(parents: &[[usize; NUM_POINTS]], mut rng: &mut ThreadRng) -> [usize; NUM_POINTS] {
    let parent = parents.choose(&mut rng).unwrap();

    let mut agent = *parent;
    agent.swap(
        rng.gen_range(0..NUM_POINTS) as usize,
        rng.gen_range(0..NUM_POINTS) as usize,
    );
    agent
}

fn distance(a: &Point, b: &Point) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

fn eval(agent: &[usize], world: &World) -> f64 {
    (0..NUM_POINTS)
        .zip((1..NUM_POINTS).chain(0..1))
        .map(|(i, j)| {
            distance(
                &world[agent[i]],
                &world[agent[j]],
            )
        })
        .sum()
}

fn eval_and_sort(pop: &mut Population, world: &World) {
    pop.sort_by(|a, b| eval(a, world).partial_cmp(&eval(b, world)).unwrap());
}
