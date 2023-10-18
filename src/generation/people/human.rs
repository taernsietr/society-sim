use rand::Rng;

use crate::generation::{
    people::attributes::{Sexuality, Gender},
    constants::*
};

use angelspeech::generator::text_generator::TextGenerator;

#[derive(Clone, Default, Debug)]
pub struct HumanBuilder {
    id: Option<usize>,
    name: Option<String>,
    family: Option<String>,
    gender: Option<Gender>,
    sexuality: Option<Sexuality>,
    age: Option<usize>,
    phenotype: Option<usize>,
    alive: Option<bool>
}

#[allow(dead_code)]
impl HumanBuilder {
    pub fn new() -> HumanBuilder {
        HumanBuilder::default()
    }

    pub fn id(&mut self, id: usize) -> &mut Self {
        self.id = id.into();
        self
    }

    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into().into();
        self
    }

    pub fn family(&mut self, family: impl Into<String>) -> &mut Self {
        self.family = family.into().into();
        self
    }

    pub fn gender(&mut self, gender: Gender) -> &mut Self {
        self.gender = gender.into();
        self
    }

    pub fn sexuality(&mut self, sexuality: Sexuality) -> &mut Self {
        self.sexuality = sexuality.into();
        self
    }

    pub fn age(&mut self, age: usize) -> &mut Self {
        self.age = age.into();
        self
    }

    pub fn phenotype(&mut self, phenotype: usize) -> &mut Self {
        self.phenotype = phenotype.into();
        self
    }

    pub fn alive(&mut self, alive: bool) -> &mut Self {
        self.alive = alive.into();
        self
    }

    pub fn random_name(&mut self, language: &TextGenerator) -> &mut Self {
        self.name = language.random_length_word(1, 5).into();
        self
    }

    pub fn random_legal_age(&mut self) -> &mut Self {
        let mut rng = rand::thread_rng();
        self.age = rng.gen_range(LEGAL_AGE..=(MAX_AGE - LEGAL_AGE)).into();
        self
    }

    pub fn random_child_age(&mut self, relative_age: usize) -> &mut Self {
        let mut rng = rand::thread_rng();
        self.age = rng.gen_range(0..=(relative_age - LEGAL_AGE)).into();
        self
    }

    pub fn random_parent_age(&mut self, relative_age: usize) -> &mut Self {
        let mut rng = rand::thread_rng();
        self.age = rng.gen_range((relative_age + LEGAL_AGE)..=MAX_AGE).into();
        self
    }

    pub fn random_spouse_age(&mut self, (min, max): (usize, usize)) -> &mut Self {
        let mut rng = rand::thread_rng();
        self.age = rng.gen_range(Ord::max(min, LEGAL_AGE)..max).into();
        self
    }

    pub fn build(&mut self, language: &TextGenerator) -> Human {
        let mut rng = rand::thread_rng();
        Human {
            id: self.id.unwrap(),
            name: self.name.clone().unwrap_or_else(|| language.random_length_word(1, 5)),
            family: self.family.clone().unwrap_or_else(|| language.random_length_word(1, 5)),
            gender: self.gender.unwrap_or_else(rand::random),
            sexuality: self.sexuality.unwrap_or_else(rand::random),
            age: self.age.unwrap_or_else(|| rng.gen_range(0..=MAX_INITIAL_AGE)),
            phenotype: self.phenotype.unwrap_or_else(|| rng.gen_range(0..=65535)),
            alive: self.alive.unwrap_or_default()
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct Human {
    pub id: usize,
    pub name: String,
    pub family: String,
    pub gender: Gender,
    pub sexuality: Sexuality,
    pub age: usize,
    pub phenotype: usize,
    pub alive: bool,
}

#[allow(dead_code)]
impl Human {
    pub fn get_id(&self) -> usize { self.id }
    pub fn get_name(&self) -> &str { self.name.as_ref() }
    pub fn get_family(&self) -> &str { self.family.as_ref() }
    pub fn get_gender(&self) -> Gender { self.gender }
    pub fn get_sexuality(&self) -> Sexuality { self.sexuality }
    pub fn get_age(&self) -> usize { self.age }
    pub fn get_alive(&self) -> bool { self.alive }
    pub fn get_phenotype(&self) -> usize { self.phenotype }

    pub fn get_full_name(&self) -> String { format!("{} {}", self.name, self.family) }
    pub fn get_age_years(&self) -> usize { self.age / 365 }

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

        //let death_threshold = 1 + self.age * 5 / MAX_AGE;
        //let roll = rng.gen_range(0..=100);

        if rng.gen_bool(0.9/1.0) {
        //if roll <= death_threshold {
            self.alive = false;
            println!(
                //"[DEATH]: {}, {}, {}, has died. [{:.2} | {:.2}]",
                "[DEATH]: {}, {}, {}, has died.",
                self.family,
                self.name,
                self.get_formatted_age().0,
                //roll,
                //death_threshold
            );
        }
    }
}

