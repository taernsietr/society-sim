use rand::{Rng, prelude::SliceRandom};
use angelspeech::generator::text_generator::TextGenerator;

use crate::generation::{
    population::Population,
    people::{
        human::Human,
        attributes::{Gender, Sexuality, RelationshipType},
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
            population.new_family(family_size, &generators)
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
        self.add_person(
            Some(family_root_id),
            Some(language.random_length_word(1, 5)),
            Some(family_name.clone()),
            None,
            None,
            Some(rng.gen_range(LEGAL_AGE..=(MAX_AGE-LEGAL_AGE))),
            None
        );
        let family_root = &self.get_pop().get(&family_root_id).unwrap();

        for _ in 0..family_size-1 {
            let relation: RelationshipType = rand::random();

            let lowest_parent_age = self.lowest_parent_age(family_root_id);

            let (age, relative_gender, relative_sexuality): (Option<usize>, Option<Gender>, Option<Sexuality>) = match relation {
                RelationshipType::Child => {(
                        Some(rng.gen_range(0..=(family_root.get_age() - LEGAL_AGE))),
                        None,
                        None
                )},
                RelationshipType::Parent => {(
                        Some(rng.gen_range((family_root.get_age() + LEGAL_AGE)..=MAX_AGE)),
                        None,
                        None
                )},
                RelationshipType::Spouse => {
                    let spouse = Human::get_valid_spouses(
                        family_root.get_gender(),
                        family_root.get_sexuality()
                    ).choose(&mut rng).unwrap();

                    let valid_spouse_ages = family_root.get_valid_spouse_ages().unwrap();
                    (
                        Some(rng.gen_range(Ord::max(valid_spouse_ages.0, LEGAL_AGE)..valid_spouse_ages.1)),
                        Some(spouse.0),
                        Some(spouse.1)
                    )
                },
                RelationshipType::Sibling => {
                    let lower_parent_age: Option<usize> = self.get_relationships()
                        .iter()
                        .filter(
                            |relationship|
                            relationship.get_person_id(1) == family_root.get_id() &&
                            matches!(relationship.get_relationship_type(), RelationshipType::Parent)
                        )
                        .map(|relationship| self.get_pop().get(&relationship.get_person_id(0)).unwrap().get_age())
                        .collect::<Vec<usize>>()
                        .iter()
                        .min()
                        .copied();

                    if let Some(age) = lower_parent_age {(
                        Some(rng.gen_range(0..(age - LEGAL_AGE))),
                        None,
                        None
                    )}
                    else {(
                        Some(rng.gen_range(0..=MAX_AGE)),
                        None,
                        None
                    )}
                }
            };

            let relative_id = self.next_id();
            self.add_person(
                Some(relative_id),
                Some(language.random_length_word(1, 5)),
                Some(family_name.clone()),
                relative_gender,
                relative_sexuality,
                age,
                None
            );
            self.create_relationship(Relationship::new(relation, family_root_id, relative_id));
        }
    }

    fn lowest_parent_age(&self, id: usize) -> Option<usize> {
        self.get_relationships()
            .iter()
            .filter(
                |relationship|
                relationship.get_person_id(1) == id &&
                matches!(relationship.get_relationship_type(), RelationshipType::Parent)
            )
            .map(|relationship| self.get_pop().get(&relationship.get_person_id(0)).unwrap().get_age())
            .collect::<Vec<usize>>()
            .iter()
            .min()
            .clone()
    }


    fn request_word(&self) -> String {
        let mut rng = rand::thread_rng();
        let language = self.get_generators().choose(&mut rng).unwrap();
        language.random_length_word(1, 5)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_person(
        &mut self,
        id: Option<usize>,
        name: Option<String>,
        family: Option<String>,
        gender: Option<Gender>,
        sexuality: Option<Sexuality>,
        age: Option<usize>,
        phenotype: Option<usize>,
    ) {
        let mut rng = rand::thread_rng();
        let person = Human {
            id: id.unwrap_or_else(|| self.next_id()),
            name: name.unwrap_or_else(|| self.request_word()),
            family: family.unwrap_or_else(|| self.request_word()),
            gender: gender.unwrap_or_else(rand::random),
            sexuality: sexuality.unwrap_or_else(rand::random),
            age: age.unwrap_or_else(|| rng.gen_range(0..=MAX_INITIAL_AGE)),
            phenotype: phenotype.unwrap_or_else(|| rng.gen_range(0..=65535)),
            alive: true,
        };
        self.get_pop().insert(person.get_id(), person);
    }
}

