mod helpers;
mod attributes;
mod constants;
mod human;
mod population;
mod relationship;

use population::Population;

fn main() {
    let running_time = 365 * 10;
    let initial_pop = 32 ;
    let mut population = Population::new(initial_pop);

    for _ in 0..running_time {
        population.tick();
    }

    population.print_population();

    println!("[Initial population: {}]", initial_pop);
    println!("[Final population: {}]", population.get_size());
    println!("[Survival rate: {}%]", population.get_survival_rate());
}
