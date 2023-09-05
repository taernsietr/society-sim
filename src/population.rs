use rand::Rng;
use rand::seq::SliceRandom;

use crate::helpers::request_word;
use crate::attributes::{MAX_AGE, LEGAL_AGE, MAX_FAMILY_SIZE, RelationshipType};
use crate::human::Human;
use crate::relationship::Relationship;

pub struct Population {
    pub population: Vec<Human>
}

impl Population {
    pub fn new(pop_size: usize) -> Population {
        let mut population = Population { population: Vec::<Human>::new() };
        let mut rng = rand::thread_rng();
        let mut remaining_pop = pop_size;

        while remaining_pop > 0 {
            let family_size = rng.gen_range(1..=Ord::min(MAX_FAMILY_SIZE, remaining_pop));
            remaining_pop -= family_size;
            population.new_family(family_size)
        }

        population
    }

    fn new_family(&mut self, size: usize) {
        let mut members: Vec<Human> = Vec::new();
        let mut rng = rand::thread_rng();
        let family_root_id = self.population.len();
        let family_name = request_word();

        // Using first person as family root
        self.population.push(Human::new(family_root_id, None, Some(family_name.clone()), None, None, Some(rng.gen_range(LEGAL_AGE..=MAX_AGE)), None, None));
        let family_root_age = self.population[family_root_id].get_age();

        for person in family_root_id+1..family_root_id+size {
            let relation: RelationshipType = rand::random();

            match relation {
                RelationshipType::Child => {
                    let age = rng.gen_range(0..=(family_root_age - LEGAL_AGE));
                    self.population.push(Human::new(person, None, Some(family_name.clone()), None, None, Some(age), None, None));
                    Population::create_relationship(&mut self.population, (family_root_id, person), relation);
                },
                RelationshipType::Parent => {
                    let age = rng.gen_range((family_root_age + LEGAL_AGE)..=MAX_AGE);
                    self.population.push(Human::new(person, None, Some(family_name.clone()), None, None, Some(age), None, None));
                    Population::create_relationship(&mut self.population, (family_root_id, person), relation);
                },
                RelationshipType::Spouse => {
                    let age = rng.gen_range(Ord::max(family_root_age / 2 + 7 * 365, LEGAL_AGE)..((family_root_age - 7 * 365) * 2));
                    let valid_spouses = self.population[family_root_id].get_valid_spouses().choose(&mut rng);
                    self.population.push(Human::new(person, None, Some(family_name.clone()), Some(valid_spouses.unwrap().0), Some(valid_spouses.unwrap().1), Some(age), None, None));
                    Population::create_relationship(&mut self.population, (family_root_id, person), relation);
                },
                RelationshipType::Sibling => {
                    let lower_parent_age = self.population[family_root_id]
                        .get_relationships()
                        .iter()
                        .filter(|x| matches!(x.get_relationship_type(), RelationshipType::Parent))
                        .map(|x| self.population[x.get_person_id()].get_age())
                        .collect::<Vec<usize>>()
                        .iter()
                        .min()
                        .copied();

                    let age = match lower_parent_age {
                        Some(age) => { Some(rng.gen_range(0..(age - LEGAL_AGE))) }
                        None => None
                    };
                    
                    self.population.push(Human::new(person, None, Some(family_name.clone()), None, None, age, None, None));
                }
            }
            Population::create_relationship(&mut self.population, (family_root_id, person), relation);
        }
        self.population.append(&mut members);
    }

    pub fn create_relationship(population: &mut [Human], indices: (usize, usize), relationship: RelationshipType) {
        let (relationship_1, relationship_2) = match relationship {
            RelationshipType::Parent => {(
                Relationship::new(RelationshipType::Parent, indices.1),
                Relationship::new(RelationshipType::Child, indices.0)
            )},
            RelationshipType::Child => {(
                Relationship::new(RelationshipType::Child, indices.1),
                Relationship::new(RelationshipType::Parent, indices.0)
            )},
            RelationshipType::Spouse => {(
                Relationship::new(RelationshipType::Spouse, indices.1),
                Relationship::new(RelationshipType::Spouse, indices.0)
            )},
            RelationshipType::Sibling => {(
                Relationship::new(RelationshipType::Sibling, indices.1),
                Relationship::new(RelationshipType::Sibling, indices.0)
            )}
        };
            
        population[indices.0].add_relationship(relationship_1);
        population[indices.1].add_relationship(relationship_2);
    }

    // TODO: Review
    #[allow(dead_code)]
    pub fn get_relationships(&self, id: usize) {
        for relationship in self.population[id].get_relationships() {
            println!("{} is {}'s {}",
                self.population[relationship.get_person_id()].get_name(),
                self.population[id].get_name(),
                relationship.get_relationship_type()
            );
        }
    }
}

