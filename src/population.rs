use std::collections::HashMap;
use rand::Rng;
use rand::seq::{SliceRandom, IteratorRandom};

use crate::helpers::request_word;
use crate::attributes::{MAX_AGE, LEGAL_AGE, MAX_FAMILY_SIZE, Gender, Sexuality, RelationshipType, MEETUP_PERIOD};
use crate::human::Human;
use crate::relationship::Relationship;

#[derive(Default)]
pub struct Population {
    elapsed_time: usize,
    alive_pop: HashMap<usize, Human>,
    dead_pop: HashMap<usize, Human>
}

impl Population {
    fn new_id(&self) -> usize {
        let mut key: usize = rand::random();
        while self.alive_pop.contains_key(&key) || self.dead_pop.contains_key(&key) {
            key = rand::random();
        }
        key
    }

    pub fn new(pop_size: usize) -> Population {
        let mut rng = rand::thread_rng();
        let mut population = Population::default();
        let mut remaining_pop = pop_size;

        while remaining_pop > 0 {
            let family_size = if remaining_pop > 1 { rng.gen_range(1..=Ord::min(MAX_FAMILY_SIZE, remaining_pop)) } else { 1 };
            remaining_pop -= family_size;
            population.new_family(family_size)
        }

        dbg!(population.alive_pop.len());
        population
    }

    fn new_family(&mut self, family_size: usize) {
        let mut rng = rand::thread_rng();
        let family_name = request_word();
        // Using first person as family root so that relationships can be created
        // Root is guaranteed to be at least 18 years old
        let family_root_id = self.new_id();
        let family_root = Human::new(family_root_id, None, Some(family_name.clone()), None, None, Some(rng.gen_range(LEGAL_AGE..=(MAX_AGE-LEGAL_AGE))), None, None);
        self.alive_pop.insert(family_root_id, family_root.clone());

        for _ in 0..family_size {
            let relation: RelationshipType = rand::random();

            let (age, spouse_gender, spouse_sexuality): (Option<usize>, Option<Gender>, Option<Sexuality>) = match relation {
                RelationshipType::Child => {
                    (Some(rng.gen_range(0..=(family_root.get_age() - LEGAL_AGE))), None, None)
                },
                RelationshipType::Parent => {
                    (Some(rng.gen_range((family_root.get_age() + LEGAL_AGE)..=MAX_AGE)), None, None)
                },
                RelationshipType::Spouse => {
                    let spouse = Human::get_valid_spouses(family_root.get_gender(), family_root.get_sexuality()).choose(&mut rng).unwrap();
                    (
                        Some(rng.gen_range(Ord::max(family_root.get_age() / 2 + 7 * 365, LEGAL_AGE)..((family_root.get_age() - 7 * 365) * 2))), Some(spouse.0), Some(spouse.1)
                    )
                },
                RelationshipType::Sibling => {
                    let lower_parent_age: Option<usize> = family_root
                        .get_relationships()
                        .iter()
                        .filter(|x| matches!(x.get_relationship_type(), RelationshipType::Parent))
                        .map(|x| self.alive_pop.get(&x.get_person_id()).unwrap().get_age())
                        .collect::<Vec<usize>>()
                        .iter()
                        .min()
                        .copied();

                    if let Some(age) = lower_parent_age {
                        (Some(rng.gen_range(0..(age - LEGAL_AGE))), None, None)
                    }
                    else {
                        (Some(rng.gen_range(0..=MAX_AGE)), None, None)
                    }
                }
            };

            let family_member_id = self.new_id();
            self.alive_pop.insert(family_member_id, Human::new(family_member_id, None, Some(family_name.clone()), spouse_gender, spouse_sexuality, age, None, None));
            self.create_relationship((family_root_id, family_member_id), relation);
        }
    }

    pub fn run_ticks(&mut self) {
        self.elapsed_time += 1;

        let dead_ids: Vec<usize> = self.alive_pop
            .iter()
            .filter(|person| !person.1.get_alive())
            .map(|person| *person.0)
            .collect();

        for id in dead_ids.iter() {
            let dead = self.alive_pop.remove_entry(id).unwrap();
            self.dead_pop.insert(dead.0, dead.1);
        }

        self.alive_pop.iter_mut().for_each(|person| person.1.tick());

        if self.elapsed_time % MEETUP_PERIOD == 0 {
            self.run_meetups();
        }
    }

    fn run_meetups(&mut self) {
        let mut rng = rand::thread_rng();

        let people = self.alive_pop
            .keys()
            .choose_multiple(&mut rng, 2)
            .clone();

        let person_1 = self.alive_pop.get(people[0]).unwrap();
        let person_2 = self.alive_pop.get(people[1]).unwrap();

        if person_1.get_family() != person_2.get_family() &&
            !person_1.has_spouse() &&
            !person_2.has_spouse() {
                let couple_threshold: usize = 3;
                let roll = rng.gen_range(0..=100);

                if roll <= couple_threshold {
                    println!("Couple formed: ({}, {})", person_1.get_full_name(), person_2.get_full_name());
                    self.create_relationship((person_1.get_id(), person_2.get_id()), RelationshipType::Spouse);
                }
            }
    }

    pub fn get_size(&self) -> usize {
        self.alive_pop.len() + self.dead_pop.len()
    }

    pub fn get_survival_rate(&self) -> usize {
        self.dead_pop.len() * 100 / self.get_size()
    }

    fn create_relationship(&mut self, indices: (usize, usize), relationship_1: RelationshipType) {
        let relationship_2 = match relationship_1 {
            RelationshipType::Parent => {RelationshipType::Child},
            RelationshipType::Child => {RelationshipType::Parent},
            RelationshipType::Spouse => {RelationshipType::Spouse},
            RelationshipType::Sibling => {RelationshipType::Sibling}
        };

        self.alive_pop.get_mut(&indices.0).unwrap().add_relationship(Relationship::new(relationship_1, indices.1));
        self.alive_pop.get_mut(&indices.1).unwrap().add_relationship(Relationship::new(relationship_2, indices.0));
    }

    // TODO: Review (including dead/alive)
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

    #[allow(dead_code)]
    pub fn print_population(&self) {
        println!("[--------- Alive ------------]");
        for person in self.alive_pop.iter() {
            println!("{}", person.1);
        }

        println!("[--------- Dead  ------------]");
        for person in self.dead_pop.iter() {
            println!("{}", person.1);
        }
    }
}

