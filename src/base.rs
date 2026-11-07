use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

/// Reads all lines from a specified file located in the 'data' subdirectory relative to the crate root.
///
/// # Arguments
///
/// * `file_name` - The name of the file to read (e.g., "Day1.txt").
///
/// # Returns
///
/// A `Result` containing a `Vec<String>` of lines if successful, or an `std::io::Error` if the file cannot be read.
pub fn get_lines_from_file(file_name: &String) -> Result<Vec<String>, std::io::Error> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join(file_name);
    println!("resolve: {}", path.display());
    let file = File::open(&path)?;
    let lecteur = BufReader::new(file);
    let mut list: Vec<String> = Vec::new();
    for ligne in lecteur.lines() {
        list.push(ligne?);
    }
    Ok(list)
}

pub enum FilesDataEnum {
    Day1Training,
    Day1,
    Day2,
    Day2Training,
}

impl FilesDataEnum {
    pub fn as_string(&self) -> String {
        match self {
            FilesDataEnum::Day1Training => "Day1Training.txt".to_string(),
            FilesDataEnum::Day1 => "Day1.txt".to_string(),
            FilesDataEnum::Day2 => "Day2.txt".to_string(),
            FilesDataEnum::Day2Training => "Day2Training.txt".to_string(),
        }
    }
}
