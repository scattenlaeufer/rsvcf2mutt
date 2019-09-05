extern crate ical;
extern crate toml;
extern crate xdg;

use std::fs::{read_dir, read_to_string, File};
use std::io::BufReader;
use toml::Value;

#[derive(Debug)]
struct Email {
    email: String,
    parameter: String,
}

impl Email {
    fn new(email: String, parameter: String) -> Email {
        Email {
            email: email,
            parameter: parameter,
        }
    }
}

#[derive(Debug)]
struct Contact {
    name: String,
    emails: Vec<Email>,
}

impl Contact {
    fn new(name: String, email: Vec<Email>) -> Contact {
        Contact {
            name: name,
            emails: email,
        }
    }
}

fn main() {
    let config = read_to_string("config.toml")
        .unwrap()
        .parse::<Value>()
        .unwrap();
    let contact_file_list = read_dir(config["contact_path"].as_str().unwrap()).unwrap();
    let mut contacts: Vec<Contact> = Vec::new();
    for contact_file_name_result in contact_file_list {
        let contact_file_name = contact_file_name_result.unwrap();
        // TODO: Find a better way to skip on any file that doesn't end with .vcf
        if contact_file_name.file_name() == "displayname" {
            continue;
        }
        let buf = BufReader::new(File::open(contact_file_name.path()).unwrap());

        let reader = ical::VcardParser::new(buf);
        for contact_result in reader {
            let contact = contact_result.unwrap();
            let mut name = "".into();
            let mut emails: Vec<Email> = Vec::new();
            for prop in contact.properties {
                if prop.name == "FN" {
                    match prop.value {
                        Some(n) => name = n,
                        None => (),
                    }
                } else if prop.name == "EMAIL" {
                    let parameter = match prop.params {
                        Some(p) => p[0].1.join("_"),
                        None => "".into(),
                    };
                    emails.push(Email::new(prop.value.unwrap(), parameter));
                }
            }
            if !emails.is_empty() {
                contacts.push(Contact::new(name, emails));
            }
        }
    }
    println!("{:#?}", contacts);
    println!("{}", contacts.len());
}
