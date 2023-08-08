use jayce::{internal::DUOS_RUST, Tokenizer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let mut total_token_count = 0;

    for (filename, source) in files.iter() {
        let mut tokenizer = Tokenizer::new(&source, &DUOS_RUST);

        let tokens_for_file = tokenizer.tokenize_all()?.len();

        println!("File {} has {} tokens", filename, tokens_for_file);

        total_token_count += tokens_for_file;
    }

    println!("Total token count : {}", total_token_count);

    Ok(())
}
