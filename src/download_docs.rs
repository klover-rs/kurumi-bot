use std::{fs, io::Read};

pub fn get_docs(which: &str) -> Result<String, std::io::Error> {
    let current_dir = std::env::current_dir()?;

    let current_dir = current_dir.join("docs");

    let file_path = current_dir.join(which);

    let mut file = fs::File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}
