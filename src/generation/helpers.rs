pub fn request_word() -> String {
    let url = "http://0.0.0.0:8080/generators/text?generator=kai&min=1&max=4&text_length=1";
    reqwest::blocking::get(url).unwrap().text().unwrap_or_default()
}

