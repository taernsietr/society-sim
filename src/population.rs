use rand::Rng;
use rand::seq::SliceRandom;

use crate::helpers::request_word;
use crate::attributes::{MAX_AGE, LEGAL_AGE, MAX_FAMILY_SIZE, Gender, Sexuality, RelationshipType};
use crate::human::Human;
use crate::relationship::Relationship;

#[derive(Default)]
pub struct Population {
    alive_pop: Vec<Human>,
    dead_pop: Vec<Human>
}

impl Population {
    pub fn new(pop_size: usize) -> Population {
        let mut rng = rand::thread_rng();
        let mut population = Population::default();
        let mut remaining_pop = pop_size;

        while remaining_pop > 0 {
            let family_size = if remaining_pop > 1 { rng.gen_range(1..=Ord::min(MAX_FAMILY_SIZE, remaining_pop)) } else { 1 };
            remaining_pop -= family_size;
            population.new_family(family_size)
        }

        population
    }

    fn new_family(&mut self, size: usize) {
        let mut rng = rand::thread_rng();
        let family_root_id = self.get_size();
        let family_name = request_word();

        // Using first person as family root so that relationships can be created
        // Root is guaranteed to be at least 18 years old
        self.alive_pop.push(Human::new(family_root_id, None, Some(family_name.clone()), None, None, Some(rng.gen_range(LEGAL_AGE..=(MAX_AGE-LEGAL_AGE))), None, None));
        let family_root_age = self.alive_pop[family_root_id].get_age();

        for person in family_root_id+1..family_root_id+size {
            let relation: RelationshipType = rand::random();

            let (age, spouse_gender, spouse_sexuality): (Option<usize>, Option<Gender>, Option<Sexuality>) = match relation {
                RelationshipType::Child => {
                    (Some(rng.gen_range(0..=(family_root_age - LEGAL_AGE))), None, None)
                },
                RelationshipType::Parent => {
                    (Some(rng.gen_range((family_root_age + LEGAL_AGE)..=MAX_AGE)), None, None)
                },
                RelationshipType::Spouse => {
                    let spouse = Human::get_valid_spouses(self.alive_pop[family_root_id].get_gender(), self.alive_pop[family_root_id].get_sexuality()).choose(&mut rng).unwrap();
                    (
                        Some(rng.gen_range(Ord::max(family_root_age / 2 + 7 * 365, LEGAL_AGE)..((family_root_age - 7 * 365) * 2))), Some(spouse.0), Some(spouse.1)
                    )
                },
                RelationshipType::Sibling => {
                    let lower_parent_age: Option<usize> = self.alive_pop[family_root_id]
                        .get_relationships()
                        .iter()
                        .filter(|x| matches!(x.get_relationship_type(), RelationshipType::Parent))
                        .map(|x| self.alive_pop[x.get_person_id()].get_age())
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

            self.alive_pop.push(Human::new(person, None, Some(family_name.clone()), spouse_gender, spouse_sexuality, age, None, None));
            Population::create_relationship(&mut self.alive_pop, (family_root_id, person), relation);
        }
    }

    pub fn run_ticks(&mut self) {
        self.alive_pop.iter_mut().for_each(|person| person.tick());
        //self.run_meetups();
    }

    fn run_meetups(&mut self) {
        todo!()
    }

    fn get_size(&self) -> usize {
        self.alive_pop.len() + self.dead_pop.len()
    }
    
    pub fn get_survival_rate(&self) -> usize {
        self.dead_pop.len() * 100 / self.get_size()
    }

    fn create_relationship(population: &mut [Human], indices: (usize, usize), relationship: RelationshipType) {
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

    // TODO: Review (including dead/alive)
    #[allow(dead_code)]
    pub fn print_relationships(&self, person_id: usize) {
        for relationship in self.alive_pop[person_id].get_relationships() {
            println!("{} is {}'s {}",
                self.alive_pop[relationship.get_person_id()].get_name(),
                self.alive_pop[person_id].get_name(),
                relationship.get_relationship_type()
            );
        }

        for relationship in self.dead_pop[person_id].get_relationships() {
            println!("{} was {}'s {}",
                self.alive_pop[relationship.get_person_id()].get_name(),
                self.alive_pop[person_id].get_name(),
                relationship.get_relationship_type()
            );
        }
    }
}

