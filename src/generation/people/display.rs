use std::fmt;

use crate::generation::people::human::Human;

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
            self.get_family(), self.get_name(),
            match self.get_alive() {
                true => "alive".to_string(),
                false => "dead".to_string(),
            },
            age,
            self.get_sexuality(), self.get_gender(), self.get_phenotype()
        )
    }
}

