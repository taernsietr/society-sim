use rand::Rng;

use crate::generation::{
    population::Population,
    helpers::request_word,
    people::{
        human::Human,
        attributes::{Gender, Sexuality, RelationshipType},
    },
    constants::*
};

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

    fn new_id(&self) -> usize {
        let mut key: usize = rand::random();
        while self.get_pop().contains_key(&key) || self.dead_pop.contains_key(&key) {
            key = rand::random();
        }

        key
    }

    pub fn new_family(&mut self, family_size: usize) {
        let mut rng = rand::thread_rng();
        let family_name = request_word();
        // Using first person as family root so that relationships can be created
        // Root is guaranteed to be at least 18 years old
        let family_root_id = self.new_id();
        let family_root = Human::new(family_root_id, None, Some(family_name.clone()), None, None, Some(rng.gen_range(LEGAL_AGE..=(MAX_AGE-LEGAL_AGE))), None, None);
        self.get_pop().insert(family_root_id, family_root.clone());

        for _ in 0..family_size-1 {
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
                    let valid_spouse_ages = family_root.get_valid_spouse_ages().unwrap();
                    (
                        Some(rng.gen_range(Ord::max(valid_spouse_ages.0, LEGAL_AGE)..valid_spouse_ages.1)), Some(spouse.0), Some(spouse.1)
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
            self.get_pop().insert(family_member_id, Human::new(family_member_id, None, Some(family_name.clone()), spouse_gender, spouse_sexuality, age, None, None));
            self.create_relationship((family_root_id, family_member_id), relation);
        }
    }
}

