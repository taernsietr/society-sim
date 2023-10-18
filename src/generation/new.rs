use rand::{Rng, prelude::SliceRandom};
use angelspeech::generator::text_generator::TextGenerator;

use crate::generation::{
    population::Population,
    people::{
        human::{HumanBuilder, Human},
        attributes::RelationshipType,
    },
    relationship::Relationship,
    constants::*
};

impl Population {
    pub fn new(pop_size: usize, generators: &[TextGenerator]) -> Population {
        let mut rng = rand::thread_rng();
        let mut population = Population::default();
        let mut remaining_pop = pop_size;

        while remaining_pop > 0 {
            let family_size = if remaining_pop > 1 {
                rng.gen_range(1..=Ord::min(MAX_FAMILY_SIZE, remaining_pop))
            } else { 1 };

            remaining_pop -= family_size;
            population.new_family(family_size, generators)
        }
        population
    }

    pub fn new_family(&mut self, family_size: usize, generators: &[TextGenerator]) {
        let mut rng = rand::thread_rng();
        let language = &generators.choose(&mut rng).unwrap();
        let family_name = language.random_length_word(1, 5);

        // Using first person as family root so that relationships can be created
        // Root is guaranteed to be at least 18 years old
        let family_root_id = self.next_id();
        let mut family_root = HumanBuilder::new();
        family_root
            .id(family_root_id)
            .random_name(language)
            .family(family_name.clone())
            .random_legal_age();
        let family_root = family_root.build(language);

        for _ in 0..family_size-1 {
            let relation: RelationshipType = rand::random();
            let lowest_parent_age = self.lowest_parent_age(family_root_id);
            let relative_id = self.next_id();
            let mut relative = HumanBuilder::new();

            match relation {
                RelationshipType::Child => { relative.random_child_age(family_root.age) },
                RelationshipType::Parent => { relative.random_parent_age(family_root.age) },
                RelationshipType::Spouse => {
                    let spouse_ages = family_root.get_valid_spouse_ages().unwrap();
                    let (spouse_gender, spouse_sexuality) = Human::get_valid_spouses(
                        family_root.gender,
                        family_root.sexuality
                    ).choose(&mut rng).unwrap();

                    relative
                        .random_spouse_age(spouse_ages)
                        .gender(*spouse_gender)
                        .sexuality(*spouse_sexuality)
                },
                RelationshipType::Sibling => {
                    if let Some(age) = lowest_parent_age { relative.age(age) }
                    else { relative.age(rng.gen_range(0..=MAX_AGE)) }
                }
            };

            relative
                .id(relative_id)
                .random_name(language)
                .family(family_name.clone());

            self.add(relative.build(language));
            self.relationships.push(Relationship::new(relation, family_root_id, relative_id));
        }
        self.add(family_root);
    }

    fn lowest_parent_age(&self, id: usize) -> Option<usize> {
        self.relationships
            .iter()
            .filter(
                |relationship|
                relationship.get_person_id(1) == id &&
                matches!(relationship.kind, RelationshipType::Parent)
            )
            .map(|relationship| self.people.get(&relationship.get_person_id(0)).unwrap().age)
            .collect::<Vec<usize>>()
            .iter()
            .min()
            .copied()
    }

    pub fn add(&mut self, person: Human) {
        self.people.insert(person.id, person);
    }
}

