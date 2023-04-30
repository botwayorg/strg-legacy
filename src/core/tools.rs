extern crate dirs;
use std::path::Path;

pub fn get_home_dir() -> String {
    let home_dir = dirs::home_dir().unwrap();

    home_dir.to_str().unwrap().to_string()
}

pub fn return_path(db: &String) -> String {
    let mut home_dir = get_home_dir();

    home_dir.push_str(&("/.".to_owned() + &db.to_string().to_owned()));

    let path = Path::new(&home_dir).to_string_lossy().to_string();

    path
}
