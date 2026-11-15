use std::{
    fs::File, io::{BufRead, BufReader, Error, ErrorKind}, path::PathBuf,
};

pub trait ResolveDay {
    fn resolve_part1(&self, file_name: &String) -> Result<String, Error>;
    fn resolve_part2(&self, file_name: &String) -> Result<String, Error>;
}

pub fn get_lines_from_file(file_name: &String) -> Result<Vec<String>, Error> {
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
pub fn get_file_path(day_name: &str, is_training: Option<bool>) -> Result<String, Error> {
    let name_file = match is_training {
        Some(true) => {day_name.to_string() + "Training"}
        None|Some(false) => day_name.to_string(),
    };
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join(name_file + ".txt")
        .into_os_string()
        .into_string()
        .map_err(|_| Error::new(ErrorKind::InvalidFilename,"impossible de creer le chemin de fichier"))
}
