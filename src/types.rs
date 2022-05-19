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
    pub fn glob_pattern(self) -> Option<&'static str> {
        // Returns the glob pattern
        return Path::new(&self.path)           
                            .join(self.pattern)
                            .to_str();  
    }
}

pub struct ParsedEmail<'a> {
    /*
    Email parsing data type that outputs a file name
    */
    from: &'a HeaderValue<'a>,
    date: Option<&'a MailDateTime>,
    subject: String,
}

impl ParsedEmail<'_> {
    fn sent_from(&self) -> String {
        let mail_address = match self.from {
            HeaderValue::Address(mail_address) => mail_address.address
                                                    .as_ref()
                                                    .unwrap()
                                                    .to_lowercase(),
            _ => panic!("Oops! could not extract the sender's email address")
        };
        return mail_address.to_lowercase();
    }

    fn sent_at(&self) -> String {
        let dt = self.date.unwrap().to_iso8601();
        return DateTime::parse_from_rfc3339(&dt).unwrap().to_string();
    }

    pub fn make_file_name(&self) -> String {
        let dt = self.sent_at();
        let sender = self.sent_from();
        let subject_line = &self.subject;
        //Have to clone because we can't initialize a vec with mismatched typing
        let file_fmt = vec![dt, sender, subject_line.clone()];
        let file_out = file_fmt.join("_");
        return file_out;
    }
}