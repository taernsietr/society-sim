use std::fmt;
use rand::Rng;

use crate::helpers::request_word;
use crate::relationship::Relationship;
use crate::attributes::{Sexuality, Gender, RelationshipType};
use crate::constants::*;

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

impl fmt::Display for Human {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (years, months, days) = self.get_formatted_age();
        
        let age: String = match (years, months, days) {
            (0, 0, 1) => "1 day old".to_string(),
            (0, 0, 1..) => format!("{} days old", days),
            (0, 1, _) => "1 month old".to_string(),
            (1, 0, _) => "1 year old".to_string(),
            (1.., 0, _) => format!("{} years old", years),
            (0, 1.., _) => format!("{} months old", months),
            (1, 1.., _) => format!("1 year, {} months old", months),
            (1.., 1, _) => format!("{} years, 1 month old", years),
            (1.., 1.., _) => format!("{} years, {} months old", years, months),
            (_, _, _) => unreachable!()
        };
        
        write!(
            f,
            "{}, {} - {}, {}, {} {} [{}]",
            self.family, self.name,
            match self.alive {
                true => "alive".to_string(),
                false => "dead".to_string(),
            },
            age,
            self.sexuality, self.gender, self.phenotype
        )
    }
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

    // TODO: Refactor this to something that doesn't look like a monkey wrote
    pub fn get_valid_spouses(gender: Gender, sexuality: Sexuality) -> &'static [(Gender, Sexuality)] {
        match (gender, sexuality) {
            (Gender::CisMale, Sexuality::Heterosexual) => &[
                (Gender::CisFemale, Sexuality::Heterosexual),
                (Gender::TransFemale, Sexuality::Heterosexual),
                (Gender::CisFemale, Sexuality::Pansexual),
                (Gender::TransFemale, Sexuality::Pansexual)
            ],
            (Gender::CisMale, Sexuality::Homosexual) => &[
                (Gender::CisMale, Sexuality::Homosexual),
                (Gender::TransMale, Sexuality::Homosexual),
                (Gender::CisMale, Sexuality::Pansexual),
                (Gender::TransMale, Sexuality::Pansexual)
            ],
            (Gender::CisFemale, Sexuality::Heterosexual) => &[
                (Gender::CisMale, Sexuality::Heterosexual),
                (Gender::TransMale, Sexuality::Heterosexual),
                (Gender::CisMale, Sexuality::Pansexual),
                (Gender::TransMale, Sexuality::Pansexual)
            ],
            (Gender::CisFemale, Sexuality::Homosexual) => &[
                (Gender::CisFemale, Sexuality::Homosexual),
                (Gender::TransFemale, Sexuality::Homosexual),
                (Gender::CisFemale, Sexuality::Pansexual),
                (Gender::TransFemale, Sexuality::Pansexual)
            ],
            (Gender::TransMale, Sexuality::Heterosexual) => &[
                (Gender::CisFemale, Sexuality::Heterosexual),
                (Gender::TransFemale, Sexuality::Heterosexual),
                (Gender::CisFemale, Sexuality::Pansexual),
                (Gender::TransFemale, Sexuality::Pansexual)
            ],
            (Gender::TransMale, Sexuality::Homosexual) => &[
                (Gender::CisMale, Sexuality::Homosexual),
                (Gender::TransMale, Sexuality::Homosexual),
                (Gender::CisMale, Sexuality::Pansexual),
                (Gender::TransMale, Sexuality::Pansexual)
            ],
            (Gender::TransFemale, Sexuality::Heterosexual) => &[
                (Gender::CisMale, Sexuality::Heterosexual),
                (Gender::TransMale, Sexuality::Heterosexual),
                (Gender::CisMale, Sexuality::Pansexual),
                (Gender::TransMale, Sexuality::Pansexual)
            ],
            (Gender::TransFemale, Sexuality::Homosexual) => &[
                (Gender::CisFemale, Sexuality::Homosexual),
                (Gender::TransFemale, Sexuality::Homosexual),
                (Gender::CisFemale, Sexuality::Pansexual),
                (Gender::TransFemale, Sexuality::Pansexual)
            ],
            (_, Sexuality::Pansexual) => &[
                (Gender::CisMale, Sexuality::Heterosexual),
                (Gender::CisMale, Sexuality::Homosexual),
                (Gender::CisFemale, Sexuality::Heterosexual),
                (Gender::CisFemale, Sexuality::Homosexual),
                (Gender::TransMale, Sexuality::Heterosexual),
                (Gender::TransMale, Sexuality::Homosexual),
                (Gender::TransFemale, Sexuality::Heterosexual),
                (Gender::TransFemale, Sexuality::Homosexual),
                (Gender::CisMale, Sexuality::Pansexual),
                (Gender::CisFemale, Sexuality::Pansexual),
                (Gender::TransMale, Sexuality::Pansexual),
                (Gender::TransFemale, Sexuality::Pansexual),
            ],
            (_, _) => unreachable!()
        }
    }

    pub fn get_valid_spouse_ages(&self) -> Option<(usize, usize)> {
        match self.age / 365 {
            0..=17 => { None },
            18..=22 => { Some((LEGAL_AGE, (self.age - (7 * 365)) * 2)) },
            23.. => { Some((self.age / 2 + (7 * 365), (self.age - (7 * 365)) * 2)) },
            _ => unreachable!()
        }
    }

    fn get_formatted_age(&self) -> (usize, usize, usize) {
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

