use std::{
    path::PathBuf,
    fs::read_dir
};

use angelspeech::generator::text_generator::TextGenerator;

pub fn load_generators() -> Vec<TextGenerator> {
    let settings = PathBuf::from("/home/tsrodr/Run/angelspeech/settings");
    let setting_files = read_dir(settings.as_os_str()).unwrap();
    let mut generators = Vec::new();

    for file in setting_files {
        generators.push(TextGenerator::load_local(file.unwrap().path()));
    };

    generators
}

