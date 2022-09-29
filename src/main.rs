mod language;
mod human;

use crate::human::Human;

fn main() {
    // let person = Human::new_random();
    let mut person: Vec<Human> = vec![];
    for _ in 0..5 {
        person.push(Human::new_random());
    }

    for (index, p) in person.into_iter().enumerate() {
        println!("{}. {}, {}", index+1, p.name, p.family);
    }
}
