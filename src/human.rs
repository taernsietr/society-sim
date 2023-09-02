use std::fmt;
use rand::{Rng, distributions::{Distribution, Standard}};

use crate::helpers::request_word;

const MAX_INITIAL_AGE: u32 = 60 * 365;
const MAX_AGE: u32 = 110 * 365;

#[derive(Clone)]
pub enum Gender {
    CisMale,
    CisFemale,
    TransMale,
    TransFemale,
    NonBinary,
}

#[derive(Clone)]
pub enum Sexuality {
    Heterosexual,
    Homosexual,
    Pansexual,
}

#[derive(Clone)]
pub enum RelationshipType {
    Parent,
    Offspring,
    Sibling,
    Spouse,
}

impl Distribution<Gender> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Gender {
        match rng.gen_range(0..5) {
            0 => Gender::CisMale,
            1 => Gender::CisFemale,
            2 => Gender::TransMale,
            3 => Gender::TransFemale,
            _ => Gender::NonBinary
        }
    }
}

impl Distribution<Sexuality> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Sexuality {
        match rng.gen_range(0..3) {
            0 => Sexuality::Heterosexual,
            1 => Sexuality::Homosexual,
            _ => Sexuality::Pansexual
        }
    }
}

impl Distribution<RelationshipType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RelationshipType {
        match rng.gen_range(0..3) {
            0 => RelationshipType::Parent,
            1 => RelationshipType::Offspring,
            2 => RelationshipType::Sibling,
            _ => RelationshipType::Spouse,
        }
    }
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Gender::CisMale => write!(f, "Cisgender Male"),
            Gender::CisFemale => write!(f, "Cisgender Female"),
            Gender::TransMale => write!(f, "Transgender Male"),
            Gender::TransFemale => write!(f, "Transgender Female"),
            Gender::NonBinary => write!(f, "Non-Binary")
        }
    }
}

impl fmt::Display for Sexuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sexuality::Heterosexual => write!(f, "Heterosexual"),
            Sexuality::Homosexual => write!(f, "Homosexual"),
            Sexuality::Pansexual => write!(f, "Pansexual"),
        }
    }
}

impl fmt::Display for RelationshipType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RelationshipType::Parent => write!(f, "parent"),
            RelationshipType::Offspring => write!(f, "offspring"),
            RelationshipType::Sibling => write!(f, "sibling"),
            RelationshipType::Spouse => write!(f, "spouse")
        }
    }
}

#[derive(Clone)]
pub struct Relationship {
    relation: RelationshipType,
    person: usize,
}

pub struct Population {
    pub population: Vec<Human>
}

impl Population {
    pub fn new(pop_size: usize) -> Population {
        let mut population: Vec<Human> = Vec::new();

        for id in 0..pop_size {
            population.push(Human::new(id, None, None, None, None, None, None, None));
        }

        Population::create_relationship(&mut population, (0, 1), RelationshipType::Parent);
        
        Population { population }
    }

    pub fn create_relationship(population: &mut [Human], indices: (usize, usize), relationship: RelationshipType) {
        let (relationship_1, relationship_2) = match relationship {
            RelationshipType::Parent => {(
                Relationship { relation: RelationshipType::Parent, person: indices.1 },
                Relationship { relation: RelationshipType::Offspring, person: indices.0 }
            )},
            RelationshipType::Offspring => {(
                Relationship { relation: RelationshipType::Offspring, person: indices.1 },
                Relationship { relation: RelationshipType::Parent, person: indices.0 }
            )},
            RelationshipType::Spouse => {(
                Relationship { relation: RelationshipType::Spouse, person: indices.1 },
                Relationship { relation: RelationshipType::Spouse, person: indices.0 }
            )},
            RelationshipType::Sibling => {(
                Relationship { relation: RelationshipType::Sibling, person: indices.1 },
                Relationship { relation: RelationshipType::Sibling, person: indices.0 }
            )}
        };
        
        population[indices.0].relationships.push(relationship_1);
        population[indices.1].relationships.push(relationship_2);
    }
    
    pub fn get_relationships(&self, id: usize) {
        for relationship in &self.population[id].relationships {
            println!("{} is {}'s {}",
                self.population[relationship.person].get_name(),
                self.population[id].get_name(),
                relationship.relation
            );
        }
    }
}

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
    pub fn get_family(&self) -> &String { &self.family }
    pub fn get_gender(&self) -> &Gender { &self.gender }
    pub fn get_sexuality(&self) -> &Sexuality { &self.sexuality }
    pub fn get_age(&self) -> &u32 { &self.age }
    pub fn get_phenotype(&mut self) -> &u32 { &self.phenotype }
    pub fn get_relationships(&self) -> &[Relationship] { &self.relationships }
    pub fn get_alive(&self) -> bool { self.alive }

    pub fn set_name(&mut self, value: String) { self.name = value }
    pub fn set_family(&mut self, value: String) { self.family = value }
    pub fn set_gender(&mut self, value: Gender) { self.gender = value }
    pub fn set_sexuality(&mut self, value: Sexuality) { self.sexuality = value }
    pub fn set_age(&mut self, value: u32) { self.age = value }
    pub fn set_phenotype(&mut self, value: u32) { self.phenotype = value }
    pub fn set_relationships(&mut self, value: Vec::<Relationship>) { self.relationships = value }
    pub fn set_alive(&mut self, value: bool) { self.alive = value }

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

