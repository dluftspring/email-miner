
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::Path;

//Helper to save files and handle path creation dynamically
pub fn save_file(filepath: String, contents: &[u8]) {
    let path = Path::new(&filepath);
    if !path.exists() {
        let parent = path.parent().unwrap();
        match create_dir_all(parent) {
            Ok(res) => println!("Created directory successfully! {:?}", res),
            Err(why) => panic!("Error creating directories in path specified {:?}", why)
        }
    }

    let mut file = match File::create(&path) {
        Err(why) => panic!("Could not create file {:?}", why),
        Ok(file) => file,
    };

    match file.write_all(contents) {
        Err(why) => panic!("Something went wrong writing your file {:?}", why),
        Ok(_) => println!("SUCCESS! Wrote file to {:?}", path.display()),
    }
}

pub fn get_file_extension(file_expression: &str) -> String {
    let file_parts = file_expression.split(".");
    let extension = file_parts.last().unwrap();
    format!(".{}", extension.to_string())
}