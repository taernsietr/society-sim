use std::fmt;
use rand::{Rng, distributions::{Distribution, Standard}};

#[derive(Clone, Copy, Default, Debug)]
pub enum Gender {
    #[default]
    CisMale,
    CisFemale,
    TransMale,
    TransFemale,
    NonBinary,
}

#[derive(Clone, Copy, Default, Debug)]
pub enum Sexuality {
    #[default]
    Heterosexual,
    Homosexual,
    Pansexual,
}

#[derive(Clone, Copy, Debug)]
pub enum RelationshipType {
    Parent,
    Child,
    Sibling,
    Spouse,
}

impl Distribution<Gender> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Gender {
        match rng.gen_range(1..100) {
            1..=30 => Gender::CisMale,
            31..=60 => Gender::CisFemale,
            61..=75 => Gender::TransMale,
            76..=90 => Gender::TransFemale,
            91..=100 => Gender::NonBinary,
            _ => unreachable!()
        }
    }
}

impl Distribution<Sexuality> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Sexuality {
        match rng.gen_range(1..100) {
            1..=50 => Sexuality::Heterosexual,
            51..=70 => Sexuality::Homosexual,
            71..=100 => Sexuality::Pansexual,
            _ => unreachable!()
        }
    }
}

impl Distribution<RelationshipType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RelationshipType {
        match rng.gen_range(0..3) {
            0 => RelationshipType::Parent,
            1 => RelationshipType::Child,
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
            RelationshipType::Sibling => write!(f, "sibling"),
            RelationshipType::Spouse => write!(f, "spouse")
        }
    }
}

