use std::fmt;
use rand::Rng;

use crate::helpers::request_word;
use crate::attributes::{MAX_INITIAL_AGE, MAX_AGE, Sexuality, Gender};
use crate::relationship::Relationship;

#[derive(Clone)]
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

#[allow(dead_code)]
impl Human {
    pub fn get_id(&self) -> usize { self.id }
    pub fn get_name(&self) -> String { self.name.clone() }
    pub fn get_age(&self) -> usize { self.age }
    pub fn get_alive(&self) -> bool { self.alive }
    pub fn get_relationships(&self) -> &[Relationship] { &self.relationships }

    pub fn add_relationship(&mut self, relationship: Relationship) { self.relationships.push(relationship); }

    // TODO: Refactor this to something that doesn't look like a monkey wrote
    pub fn get_valid_spouses(&self) -> &[(Gender, Sexuality)] {
        match (&self.gender, &self.sexuality) {
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

    fn get_formatted_age(&self) -> (usize, usize, usize) {
        let mut days = self.age;
        let years: usize = days / 365;
        days -= years * 365; 
        let months: usize = days / 30;
        days -= months * 30; 

        (years, months, days)
    }

    pub fn tick(&mut self) {
        if self.alive {
            self.check_death();
        }
    }

    fn check_death(&mut self) {
        let mut rng = rand::thread_rng(); 

        self.age += 1;

        let death_threshold = (self.age / MAX_AGE) as f32;
        let roll = rng.gen_range(0.0..=1.0);

        if roll <= death_threshold {
            self.alive = false;
            println!("{}, {}, has died. [{:.2} | {:.2}]", self.name, self.get_formatted_age().0, roll, death_threshold);
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

