use mail_parser::Message;
use std::path::Path;
use std::fs::read_to_string;
use glob::glob;
mod helpers;
use helpers::save_file;
mod types;
use types::{CLI, ParsedEmail};

fn main() {

    let glob_pattern = std::env::args().nth(1).expect("No pattern specified");
    let dir_path = std::env::args().nth(2).expect("No directory specified");
    let cli_args = CLI {
        pattern: glob_pattern,
        path: dir_path,
    };

    for entry in glob(&cli_args.glob_pattern()).expect("No matching files") {
        match entry {
            Ok(path) => {
                let posix_path = Path::new(&path);
                let contents = read_to_string(posix_path)
                                    .unwrap();
                println!("Working on file... {:?}", path.display());
                let message = Message::parse(contents.as_bytes()).unwrap();
                let parsed_email = ParsedEmail {
                    from: message.get_from(),
                    date: message.get_date(),
                    subject: message.get_subject().unwrap().to_string(),
                };
                let filepath = parsed_email.make_file_name();
                save_file(filepath, contents.as_bytes())
            },
            Err(e) => println!("{:?}", e),
        }
    }
}

