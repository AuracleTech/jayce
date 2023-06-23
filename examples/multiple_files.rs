use jayce::{internal::DUOS_RUST, Tokenizer};

fn main() {
    let current_dir = std::env::current_dir()
        .expect("Unable to get current directory")
        .join("data");

    let mut files = Vec::new();

    if let Ok(entries) = std::fs::read_dir(current_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                if file_path.is_file() && file_path.extension().unwrap_or_default() == "rs" {
                    files.push((
                        file_path.file_name().unwrap().to_str().unwrap().to_owned(),
                        std::fs::read_to_string(&file_path).expect("Unable to read file"),
                    ));
                }
            }
        }
    }

    for (filename, source) in files.iter() {
        let mut jayce = Tokenizer::new(&source, &DUOS_RUST);
        let total_tokens = jayce.tokenize_all().len();
        println!("{} has {} tokens", filename, total_tokens);
    }
}
