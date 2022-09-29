use crate::language::*;
use rand::{Rng, distributions::{Distribution, Standard}};

pub enum Gender {
    CisMale,
    CisFemale,
    TransMale,
    TransFemale,
    NonBinary,
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

pub enum Sexuality {
    Heterosexual,
    Homosexual,
    Pansexual,
}

impl Distribution<Sexuality> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Sexuality {
        match rng.gen_range(0..5) {
            0 => Sexuality::Heterosexual,
            1 => Sexuality::Homosexual,
            _ => Sexuality::Pansexual
        }
    }
}

#[allow(dead_code)]
pub struct Human {
    pub name: String,
    pub family: String,
    gender: Gender,
    sexuality: Sexuality,
    age: u8,
    social_score: u8,
    phenotype: u16,
}

impl Human {

    #[allow(dead_code)]
    pub fn new(name: String, family: String, gender: Gender, sexuality: Sexuality, age: u8, social_score: u8, phenotype: u16) -> Human {
        Human {
            name,
            family,
            gender,
            sexuality,
            age,
            social_score,
            phenotype,
        }
    }

    #[allow(dead_code)]
    pub fn new_random() -> Human {
        let mut rng = rand::thread_rng();
        let g: Gender = rand::random();
        let s: Sexuality = rand::random();
        Human {
            name: random_word(),
            family: random_word(),
            gender: g,
            sexuality: s,
            age: rng.gen_range(1..65),
            social_score: rng.gen_range(0..255),
            phenotype: rng.gen_range(0..65535),
        }
    }

    #[allow(dead_code)]
    pub fn new_random_from_family(family: String) -> Human {
        let mut rng = rand::thread_rng();
        let g: Gender = rand::random();
        let s: Sexuality = rand::random();
        Human {
            name: random_word(),
            family,
            gender: g,
            sexuality: s,
            age: rng.gen_range(1..65),
            social_score: rng.gen_range(0..255),
            phenotype: rng.gen_range(0..65535),
        }
    }
}

