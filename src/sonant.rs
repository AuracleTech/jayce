#[derive(Debug)]
pub struct Sonant {
    pub name: String,
    pub regex: String,
}

impl Sonant {
    pub fn new(name: &str, regex: &str) -> Sonant {
        Sonant {
            name: name.to_string(),
            regex: regex.to_string(),
        }
    }
}
