use rand::Rng;

use crate::generation::{
    helpers::request_word,
    relationship::Relationship,
    people::attributes::{Sexuality, Gender, RelationshipType},
    constants::*
};

#[derive(Clone, Default, Debug)]
pub struct Human {
    id: usize,
    name: String,
    family: String,
    gender: Gender,
    sexuality: Sexuality,
    age: usize,
    phenotype: usize,
    relationships: Vec<Relationship>,
    alive: bool,
}

impl Human {
    pub fn get_id(&self) -> usize { self.id }
    pub fn get_name(&self) -> String { self.name.clone() }
    pub fn get_family(&self) -> String { self.family.clone() }
    pub fn get_full_name(&self) -> String { format!("{} {}", self.name, self.family) }
    pub fn get_gender(&self) -> Gender { self.gender }
    pub fn get_sexuality(&self) -> Sexuality { self.sexuality }
    pub fn get_age(&self) -> usize { self.age }
    pub fn get_age_years(&self) -> usize { self.age / 365 }
    pub fn get_alive(&self) -> bool { self.alive }
    pub fn get_relationships(&self) -> &[Relationship] { &self.relationships }
    pub fn get_phenotype(&self) -> usize { self.phenotype }

    pub fn add_relationship(&mut self, relationship: Relationship) { self.relationships.push(relationship); }

    pub fn get_spouse(&self) -> Option<usize> {
        let lookup: Vec<usize> = self.relationships
            .iter()
            .filter(|relationship| matches!(relationship.get_relationship_type(), RelationshipType::Spouse))
            .map(|relationship| relationship.get_person_id())
            .collect();
        if lookup.is_empty() { None }
        else { Some(lookup[0]) }
    }

    pub fn get_formatted_age(&self) -> (usize, usize, usize) {
        let mut days = self.age;
        let years: usize = days / 365;
        days -= years * 365; 
        let months: usize = days / 30;
        days -= months * 30; 

        (years, months, days)
    }

    pub fn tick(&mut self) {
        self.age += 1;

        if self.alive && self.age % 365 == 0 {
            self.check_death();
        }
    }

    fn check_death(&mut self) {
        let mut rng = rand::thread_rng(); 

        let death_threshold = 1 + self.age * 5 / MAX_AGE;
        let roll = rng.gen_range(0..=100);

        if roll <= death_threshold {
            self.alive = false;
            println!("[DEATH]: {}, {}, {}, has died. [{:.2} | {:.2}]", self.family, self.name, self.get_formatted_age().0, roll, death_threshold);
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: usize,
        name: Option<String>,
        family: Option<String>,
        gender: Option<Gender>,
        sexuality: Option<Sexuality>,
        age: Option<usize>,
        phenotype: Option<usize>,
        relationships: Option<Vec<Relationship>>,
    ) -> Human {
        let mut rng = rand::thread_rng();

        Human {
            id,
            name: name.unwrap_or_else(request_word),
            family: family.unwrap_or_else(request_word),
            gender: gender.unwrap_or_else(rand::random),
            sexuality: sexuality.unwrap_or_else(rand::random),
            age: age.unwrap_or_else(|| rng.gen_range(0..=MAX_INITIAL_AGE)),
            phenotype: phenotype.unwrap_or_else(|| rng.gen_range(0..=65535)),
            relationships: relationships.unwrap_or_default(),
            alive: true,
        }
    }
}

