mod language;
mod human;
mod housing;

use human::Human;

fn main() {
    // let person = Human::new_random();
    let mut population: Vec<Human> = vec![];
    for _ in 0..5 {
        population.push(Human::new_random());
    }

    let mut time = (0, 1);

    for _ in 0..35 {
        for index in 0..population.len() {
            population[index].tick();
        }
        if time.1 == 12 {
            time.0 += 1;
            time.1 = 1;
        } else {
            time.1 += 1;
        }
    }

    for (index, p) in population.into_iter().enumerate() {
        println!("{}. {}, {} - age: {:?}", index+1, p.name, p.family, p.age);
    }
}
