use std::fmt;
use rand::Rng;

use crate::helpers::request_word;
use crate::attributes::{Sexuality, Gender};
use crate::relationship::Relationship;

const MAX_INITIAL_AGE: u32 = 60 * 365;
const MAX_AGE: u32 = 110 * 365;

#[derive(Clone)]
pub struct Human {
    id: usize,
    name: String,
    family: String,
    gender: Gender,
    sexuality: Sexuality,
    age: u32,
    phenotype: u32,
    relationships: Vec<Relationship>,
    alive: bool,
}

impl fmt::Display for Human {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (years, months, _) = self.get_formatted_age();
        
        let age: String = match (years, months) {
            (0, 0) => "0 months old".to_string(),
            (0, 1) => "1 month old".to_string(),
            (1, 0) => "1 year old".to_string(),
            (1.., 0) => format!("{} years old", years),
            (0, 1..) => format!("{} months old", months),
            (1, 1..) => format!("1 year, {} months old", months),
            (1.., 1) => format!("{} years, 1 month old", years),
            (1.., 1..) => format!("{} years, {} months old", years, months),
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
    pub fn get_name(&self) -> &String { &self.name }
    pub fn get_alive(&self) -> bool { self.alive }
    pub fn get_relationships(&self) -> &[Relationship] { &self.relationships }

    pub fn add_relationship(&mut self, relationship: Relationship) { self.relationships.push(relationship); }

    pub fn tick(&mut self) {
        if self.alive {
            let mut rng = rand::thread_rng(); 

            self.age += 1;

            let dead = (self.age / MAX_AGE) as f32;
            let roll = rng.gen_range(0.0..=1.0);

            if roll < dead {
                self.alive = false;
                println!("{}, {}, has died. [{:.2} | {:.2}]", self.name, self.get_formatted_age().0, roll, dead);
            }
        }
    }

    fn get_formatted_age(&self) -> (u32, u32, u32) {
        let mut days = self.age;
        let years: u32 = days / 365;
        days -= years * 365; 
        let months: u32 = days / 30;
        days -= months * 30; 

        (years, months, days)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: usize,
        name: Option<String>,
        family: Option<String>,
        gender: Option<Gender>,
        sexuality: Option<Sexuality>,
        age: Option<u32>,
        phenotype: Option<u32>,
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

