use mail_parser::{Message, HeaderValue};
use chrono::DateTime;
use std::path::Path;
use std::fs::{read_to_string, File, create_dir_all};
use std::io::Write;
use glob::glob;

fn main() {

    let glob_pattern = std::env::args.nth(1).expect("No pattern specified");
    let dir_path = std::env::args.nth(2).expect("No directory specified");
    let cli_args = CLI {
        pattern: glob_pattern,
        path: Path::new(&dir_path)
    };

    for entry in glob("data/*.eml").expect("No matching files") {
        match entry {
            Ok(path) => {
                let posix_path = Path::new(&path);
                let contents = read_to_string(posix_path)
                                    .unwrap();
                println!("{:?}", path.display());
                let parsed_email = Message::parse(contents.as_bytes())
                                                .unwrap();
                    let from = match parsed_email.get_from() {
                        HeaderValue::Address(mail_address) => mail_address.address
                                                                .as_ref()
                                                                .unwrap()
                                                                .to_lowercase(),
                        _ => panic!("Oops! could not make sense of this object")
                    };
                    let send_date = parsed_email.get_date().unwrap().to_iso8601();
                    let parsed_date = DateTime::parse_from_rfc3339(&send_date).unwrap();
                    let send_year_month = parsed_date.format("%Y-%m-%d");
                    let email_subject = parsed_email.get_subject().unwrap();
                    let file_fmt = vec![send_year_month.to_string(), from.to_string().to_owned(), email_subject.to_owned()];
                    let file_out = file_fmt.join("_");
                    println!("{:?}", file_out);
                    let out_path = vec![String::from("data"), String::from("output"), file_out].join("/");
                    save_file(out_path, contents.as_bytes())
            },
            Err(e) => println!("{:?}", e),
        }
    }
}

struct CLI {
    /*
    Data type for gathering cli arguments
     */
    pattern: String,
    path: std::path::Path,
}

struct ParsedEmail {
    /*
    To implement email parsing struct
    */
    from: String,
    date: DateTime,
    subject: String,
}

//Helper to save files and handle path creation dynamically
fn save_file(filepath: String, contents: &[u8]) {
    let path = Path::new(&filepath);
    if !path.exists() {
        let parent = path.parent().unwrap();
        match create_dir_all(parent) {
            Ok(res) => println!("Created directory successfully! {:?}", res),
            Err(why) => panic!("Error creating directories in path specified {:?}", why)
        }
    }

    let mut file = match File::create(&path) {
        Err(why) => panic!("Could not create file {:?}", path.display()),
        Ok(file) => file,
    };

    match file.write_all(contents) {
        Err(why) => panic!("Something went wrong writing your file {:?}", why),
        Ok(_) => println!("SUCCESS! Wrote file to {:?}", path.display()),
    }
}

