mod helpers;
mod attributes;
mod human;
mod population;
mod relationship;

use population::Population;
use helpers::Parameters;

fn main() {
    let params = Parameters { running_time: 365 * 10, initial_pop: 32 };
    let mut population = Population::new(params.initial_pop);

    for _ in 0..params.running_time {
        population.tick();
    }

    population.print_population();

    println!("[Initial population: {}]", params.initial_pop);
    println!("[Final population: {}]", population.get_size());
    println!("[Survival rate: {}%]", population.get_survival_rate());
}
