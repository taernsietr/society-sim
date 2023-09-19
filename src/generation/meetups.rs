use rand::Rng;

use crate::Population;

use crate::generation::{
    people::{
        human::Human,
        attributes::RelationshipType,
    },
    constants::*
};

impl Population {
    // this is only running once
    pub fn meetups(&mut self) {
        if !(self.get_pop().len() > 1 && self.elapsed_time % MEETUP_PERIOD == 0) { return }
        let mut rng = rand::thread_rng();

        let people: Vec<&Human> = self.get_pop()
            .iter()
            .choose_multiple(&mut rng, 2)
            .iter()
            .map(|person| person.1)
            .collect();

        let person_1 = people[0];
        let person_2 = people[1];
        
        let person_2_ages = person_2.get_valid_spouse_ages();
        if person_2_ages.is_none() { return }
        let person_2_ages = person_2_ages.unwrap();

        // TODO: Review; how to simplify this bunch of conditions?
        // Different families; no (living) spouses; valid ages
        if person_1.get_family() != person_2.get_family() &&
            person_1.get_valid_spouse_ages().is_some() &&
            person_1.get_age() >= person_2_ages.0 &&
            person_1.get_age() <= person_2_ages.1 &&
            person_1.get_spouse().is_none() && 
            person_2.get_spouse().is_none() {

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
                self.create_relationship((person_1.get_id(), person_2.get_id()), RelationshipType::Spouse);
            }
        }
    }
}

