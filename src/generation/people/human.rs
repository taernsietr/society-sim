use rand::Rng;

use crate::generation::{
    people::attributes::{Sexuality, Gender},
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
    pub fn get_phenotype(&self) -> usize { self.phenotype }

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
        name: String,
        family: String,
        gender: Gender,
        sexuality: Sexuality,
        age: usize,
        phenotype: usize,
    ) -> Human {
        Human {
            id,
            name,
            family,
            gender,
            sexuality,
            age,
            phenotype,
            alive: true,
        }
    }
}

