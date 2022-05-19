use mail_parser::HeaderValue;
use std::path::Path;
use std::option::Option;
use chrono::DateTime;
use mail_parser::DateTime as MailDateTime;

pub struct CLI {
    /*
    Data type for gathering cli arguments
     */
    pub pattern: String,
    pub path: String,
}

impl CLI {
    pub fn glob_pattern(self) -> String {
        // Returns the glob pattern
        let posix_path = Path::new(&self.path).join(self.pattern);
        posix_path.to_str().unwrap().to_string()
    }
}

pub struct ParsedEmail<'a> {
    /*
    Email parsing data type that outputs a file name
    */
    pub from: &'a HeaderValue<'a>,
    pub date: Option<&'a MailDateTime>,
    pub subject: String,
    pub file_type: String,
}

impl ParsedEmail<'_> {
    fn sent_from(&self) -> String {
        let mail_address = match self.from {
            HeaderValue::Address(mail_address) => mail_address.address
                                                    .as_ref()
                                                    .unwrap(),
            _ => panic!("Oops! could not extract the sender's email address")
        };
        mail_address.to_lowercase()
    }

    fn sent_at(&self) -> String {
        let dt = self.date.unwrap().to_iso8601();
        DateTime::parse_from_rfc3339(&dt).unwrap().to_string()
    }

    pub fn make_file_name(&self) -> String {
        let dt = self.sent_at();
        let sender = self.sent_from();
        let subject_line = &self.subject;
        //Have to clone because we can't initialize a vec with mismatched typing
        let file_fmt = vec![dt, sender, subject_line.clone()];
        let file_out = file_fmt.join("_");
        format!("{}{}", file_out, self.file_type)
    }
}