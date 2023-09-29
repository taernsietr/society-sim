mod generation;
mod helpers;

use generation::population::Population;
use helpers::load_generators;

fn main() {
    let running_time = 365 * 10;
    let initial_pop = 32;
    let generators = load_generators();
    let mut population = Population::new(initial_pop, &generators);

    for _ in 0..running_time {
        population.tick();
    }

    population.print_population();

    println!("[Initial population: {}]", initial_pop);
    println!("[Final population: {}]", population.get_size());
    println!("[Survival rate: {}%]", population.get_survival_rate());
}
