use rand::Rng;

use crate::helpers::request_word;
use crate::attributes::RelationshipType;
use crate::human::Human;
use crate::relationship::Relationship;

const MAX_FAMILY_SIZE: usize = 5;

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
        let family_root_id = self.population.len();
        let family_name = request_word();

        self.population.push(Human::new(family_root_id, None, Some(family_name.clone()), None, None, None, None, None));
        for person in family_root_id+1..family_root_id+size {
            let relation: RelationshipType = rand::random();
            self.population.push(Human::new(person, None, Some(family_name.clone()), None, None, None, None, None));
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

