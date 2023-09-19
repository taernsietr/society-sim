use crate::generation::{
    people::{
        human::Human,
        attributes::{Sexuality, Gender}
    },
    constants::LEGAL_AGE
};

impl Human {
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
        let age = self.get_age();
        match age / 365 {
            0..=17 => { None },
            18..=22 => { Some((LEGAL_AGE, (age - (7 * 365)) * 2)) },
            23.. => { Some((age / 2 + (7 * 365), (age - (7 * 365)) * 2)) },
            _ => unreachable!()
        }
    }
}

