use rand::{Rng, prelude::IteratorRandom};

use crate::Population;
use crate::generation::{
    people::{
        human::Human,
        attributes::RelationshipType,
    },
    relationship::Relationship,
    constants::*
};

impl Population {
    // TODO: Review; how to simplify this bunch of conditions?
    // Different families; no (living) spouses; valid ages
    fn compatible(&self, person_1: &Human, person_2: &Human) -> bool {
        let person_2_ages = person_2.get_valid_spouse_ages();

        if person_2_ages.is_none() { return false }
        let person_2_ages = person_2_ages.unwrap();

        person_1.get_family() != person_2.get_family() && // person 1 and person 2 are from
                                                          // different families
        person_1.get_valid_spouse_ages().is_some() &&     // person 1 is old enough
        person_1.get_age() >= person_2_ages.0 &&          // person 1 is not too young for person 2
        person_1.get_age() <= person_2_ages.1 &&          // person 1 is not too old for person 2
        self.has_spouses(person_1.get_id()) &&            // person 1 has no spouse
        self.has_spouses(person_2.get_id())               // person 2 has no spouse
    }

    fn roll_marriage(&mut self, person_1: &Human, person_2: &Human) {
        let couple_threshold: usize = 3;
        let roll = rng.gen_range(0..=100);

        if roll <= couple_threshold {
            println!(
                "[MARRIAGE]: Couple formed: {}, {} and {}, {}. [{} | {}]",
                person_1.get_full_name(),
                person_1.get_age_years(),
                person_2.get_full_name(),
                person_2.get_age_years(),
                roll,
                couple_threshold
            );
            self.create_relationship(
                Relationship::new(
                    RelationshipType::Spouse,
                    person_1.get_id(),
                    person_2.get_id()
                )
            );
        }
    }

    pub fn meetups(&mut self) {
        if !(
            self.get_living().len() > 1 &&
            self.elapsed_time() % MEETUP_PERIOD == 0
        ) { return }

        let mut rng = rand::thread_rng();

        for _ in self.get_living().len() {
            let people: Vec<&Human> = self.get_pop()
                .iter()
                .choose_multiple(&mut rng, 2)
                .iter()
                .map(|person| person.1)
                .collect();

            let person_1 = people[0];
            let person_2 = people[1];
            
            if self.compatible(person_1, person_2) {
                self.roll_marriage(person_1, person_2);
            }
        }
    }
}

