use std::fmt;
use rand::{Rng, distributions::{Distribution, Standard}};

pub const MAX_INITIAL_AGE: usize = 60 * 365;
pub const MAX_AGE: usize = 110 * 365;
pub const LEGAL_AGE: usize = 18 * 365;
pub const MAX_FAMILY_SIZE: usize = 5;

#[derive(Clone, Copy)]
pub enum Gender {
    CisMale,
    CisFemale,
    TransMale,
    TransFemale,
    NonBinary,
}

#[derive(Clone, Copy)]
pub enum Sexuality {
    Heterosexual,
    Homosexual,
    Pansexual,
}

#[derive(Clone, Copy)]
pub enum RelationshipType {
    Parent,
    Child,
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
            RelationshipType::Child => write!(f, "child"),
            RelationshipType::Sibling => write!(f, "sibling"),
            RelationshipType::Spouse => write!(f, "spouse")
        }
    }
}

