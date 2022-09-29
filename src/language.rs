use rand::{Rng, prelude::SliceRandom};

pub fn random_word() -> String {
    let c = vec!["p", "t", "k", "b", "d", "g", "ts", "dz", "tʃ", "dʒ", "f", "v", "θ", "ð", "s", "z", "ʃ", "ʒ", "x", "ɣ", "h", "r", "l", "y", "w", "m", "n"];
    let v = vec!["i", "a", "u"];
    let e = vec!["θ", "ð", "s", "ʃ", "z", "ʒ", "x", "ɣ", "n"]; 
    let patterns = vec!["VV", "VE", "CV", "CVE"];

    let mut rng = rand::thread_rng();
    let mut word = "".to_string();
    let word_length = rng.gen_range(1..=3);  

    for _ in 1..=word_length {
        let current = patterns.choose(&mut rng);
        for letter in current.unwrap().chars() {
            match letter {
                'C' => word.push_str(c.choose(&mut rng).unwrap()),
                'V' => word.push_str(v.choose(&mut rng).unwrap()),
                'E' => word.push_str(e.choose(&mut rng).unwrap()),
                _ => ()
            }
        }
    }
    word
}
