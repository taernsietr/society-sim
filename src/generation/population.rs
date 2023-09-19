use std::collections::HashMap;

use crate::generation::{
    relationship::Relationship,
    people::{
        human::Human,
        attributes::RelationshipType,
    },
};

#[derive(Default)]
pub struct Population {
    elapsed_time: usize,
    alive_pop: HashMap<usize, Human>,
    dead_pop: HashMap<usize, Human>
}

impl Population {
    pub fn get_pop(&mut self) -> HashMap<usize, Human> { self.alive_pop }
    pub fn elapsed_time(&self) -> usize { self.elapsed_time }

    pub fn get_size(&self) -> usize {
        self.alive_pop.len() + self.dead_pop.len()
    }

    pub fn get_survival_rate(&self) -> usize {
        self.alive_pop.len() * 100 / self.get_size()
    }

    pub fn create_relationship(&mut self, indices: (usize, usize), relationship_1: RelationshipType) {
        let relationship_2 = match relationship_1 {
            RelationshipType::Parent => {RelationshipType::Child},
            RelationshipType::Child => {RelationshipType::Parent},
            RelationshipType::Spouse => {RelationshipType::Spouse},
            RelationshipType::Sibling => {RelationshipType::Sibling}
        };

        self.alive_pop.get_mut(&indices.0).unwrap().add_relationship(Relationship::new(relationship_1, indices.1));
        self.alive_pop.get_mut(&indices.1).unwrap().add_relationship(Relationship::new(relationship_2, indices.0));
    }

    pub fn tick(&mut self) {
        self.elapsed_time += 1;
        self.alive_pop.iter_mut().for_each(|person| person.1.tick());
        self.dead_cleanup();
        self.meetups();
    //    self.children();
    }

    fn dead_cleanup(&mut self) {
        let dead_ids: Vec<usize> = self.alive_pop
            .iter()
            .filter(|person| !person.1.get_alive())
            .map(|person| *person.0)
            .collect();

        for id in dead_ids.iter() {
            let dead = self.alive_pop.remove_entry(id).unwrap();
            self.dead_pop.insert(dead.0, dead.1);
        }
    }

    //fn children(&mut self) {
    //    // this means children will have the same birthday
    //    if !self.elapsed_time % CHILDBIRTH_PERIOD == 0 { return }

    //    let mut rng = rand::thread_rng();
    //    let mut children: Vec<Human> = Vec::new();
    //    
    //    let mut candidates: Vec<usize> = self.alive_pop
    //        .iter()
    //        .filter(|person| person.1.get_spouse().is_some())
    //        .filter(|person| person.1.get_age() <= FERTILE_AGE)
    //        .map(|person| person.1.get_id())
    //        .collect::<Vec<usize>>();
    //    candidates.sort();
    //    candidates.dedup();

    //    for couple in candidates {
    //        let parent_1 = self.alive_pop.get(&couple.0).unwrap().clone();
    //        let parent_2 = self.alive_pop.get(&couple.1).unwrap().clone();
    //        let family_name = parent_1.get_family();

    //        let childbirth_threshold: usize = 100;
    //        let roll = rng.gen_range(0..=100);

    //        if roll <= childbirth_threshold {
    //            let child = Human::new(self.new_id(), Some(request_word()), Some(family_name), None, None, Some(0), None, None);
    //            println!(
    //                "[BIRTH]: {}, {} has been born. [{} | {}]",
    //                child.get_family(),
    //                child.get_name(),
    //                roll,
    //                childbirth_threshold
    //            );
    //            self.create_relationship((parent_1.get_id(), child.get_id()), RelationshipType::Parent);
    //            self.create_relationship((parent_2.get_id(), child.get_id()), RelationshipType::Parent);
    //            children.push(child);
    //        }
    //    }
    //}

    #[allow(dead_code)]
    pub fn print_relationships(&self, person: &Human) {
        for relationship in person.get_relationships() {
            let mut relative = self.alive_pop.get(&relationship.get_person_id());
            if relative.is_none() { relative = self.dead_pop.get(&relationship.get_person_id()) }
            if relative.is_none() { return }

            println!(
                "{} is {}'s {}",
                person.get_name(),
                relative.unwrap().get_name(),
                relationship.get_relationship_type()
            );
        }
    }

    pub fn print_population(&self) {
        println!("\n[--------- Alive ------------]");
        for person in self.alive_pop.iter() {
            println!("{}", person.1);
        }

        println!("\n[--------- Dead  ------------]");
        for person in self.dead_pop.iter() {
            println!("{}", person.1);
        }
    }
}

