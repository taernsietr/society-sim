use std::collections::HashMap;
use angelspeech::generator::text_generator::TextGenerator;

use crate::generation::{
    relationship::Relationship,
    people::human::Human
};

#[derive(Default)]
pub struct Population {
    generators: Vec<TextGenerator>,
    pub elapsed_time: usize,
    last_id: usize,
    pub people: HashMap<usize, Human>,
    pub relationships: Vec<Relationship>
}

#[allow(dead_code)]
impl Population {
    pub fn get_generators(&self) -> &[TextGenerator] { &self.generators } 

    pub fn get_living(&mut self) -> Vec<usize> {
        self.people
            .iter()
            .filter(|person| person.1.alive)
            .map(|person| person.1.id)
            .collect::<Vec<usize>>()
   }

    pub fn next_id(&mut self) -> usize {
        self.last_id += 1;
        self.last_id.to_owned()
    }

    pub fn get_survival_rate(&self) -> usize {
        self.people.iter().filter(|person| person.1.alive).count() * 100 / self.people.len()
    }

    pub fn tick(&mut self) {
        self.elapsed_time += 1;
        self.people.iter_mut().for_each(|person| person.1.tick());
        self.meetups();
    //  self.children();
    }

    pub fn print_population(&self) {
        println!("\n[--------- Alive ------------]");
        for person in self.people.clone().into_iter().filter(|person| person.1.alive) {
            println!("{}", person.1);
        }

        println!("\n[--------- Dead  ------------]");
        for person in self.people.clone().into_iter().filter(|person| !person.1.alive) {
            println!("{}", person.1);
        }
    }

    pub fn json(&self) -> String {
        let people = &self.people.values().collect::<Vec<&Human>>();
        serde_json::to_string(people).unwrap()
    }
}


