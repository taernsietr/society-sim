use std::collections::HashMap;
use angelspeech::generator::text_generator::TextGenerator;

use crate::generation::{
    relationship::Relationship,
    people::human::Human
};

#[derive(Default)]
pub struct Population {
    generators: Vec<TextGenerator>,
    elapsed_time: usize,
    people: HashMap<usize, Human>,
    relationships: Vec<Relationship>
}

impl Population {
    pub fn elapsed_time(&self) -> usize { self.elapsed_time }
    pub fn get_generators(&self) -> &[TextGenerator] { &self.generators } 
    pub fn get_size(&self) -> usize { self.people.len() }
    pub fn get_relationships(&self) -> Vec<Relationship> { self.relationships }
    pub fn get_relationships_mut(&mut self) -> Vec<Relationship> { self.relationships }
    pub fn get_pop(&mut self) -> HashMap<usize, Human> { self.people }
    pub fn get_living(&mut self) -> Vec<usize> {
        self.people
            .iter()
            .filter(|person| person.1.get_alive())
            .map(|person| person.1.get_id())
            .collect::<Vec<usize>>()
   }

    pub fn get_survival_rate(&self) -> usize {
        self.people.iter().filter(|person| person.1.get_alive()).count() * 100 / self.people.len()
    }

    pub fn tick(&mut self) {
        self.elapsed_time += 1;
        self.people.iter_mut().for_each(|person| person.1.tick());
    //  self.meetups();
    //    self.children();
    }

    pub fn print_population(&self) {
        println!("\n[--------- Alive ------------]");
        for person in self.people.into_iter().filter(|person| person.1.get_alive()) {
            println!("{}", person.1);
        }

        println!("\n[--------- Dead  ------------]");
        for person in self.people.into_iter().filter(|person| !person.1.get_alive()) {
            println!("{}", person.1);
        }
    }
}

