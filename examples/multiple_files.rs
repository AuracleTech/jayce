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

    let biggest_filename_size = files
        .iter()
        .map(|(filename, _)| filename.len())
        .max()
        .unwrap_or_default();

    for (filename, source) in files.iter() {
        let mut jayce = Tokenizer::new(&source, &DUOS_RUST);

        let formatted_filename = format!("{:width$}", filename, width = biggest_filename_size);
        let formatted_count = format!("{:width$}", jayce.tokenize_all().len(), width = 8);

        println!("{}: {}", formatted_filename, formatted_count);
    }
}
