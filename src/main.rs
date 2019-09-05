extern crate ical;
extern crate toml;
extern crate xdg;

use std::fs::{read_dir, read_to_string, write, File};
use std::io::BufReader;
use toml::Value;

#[derive(Debug, Eq, PartialEq)]
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
    fn new(name: String, emails: Vec<Email>) -> Contact {
        let mut email_vec = emails;
        email_vec.sort_by(|a, b| a.parameter.cmp(&b.parameter));
        Contact {
            name: name,
            emails: email_vec,
        }
    }

    fn create_mutt_alias(&self) -> Vec<String> {
        let mut alias_vec: Vec<String> = Vec::new();
        for email in &self.emails {
            let param = match email.parameter.as_ref() {
                "" => "".into(),
                "pref" => "".into(),
                s => format!("_{}", s),
            };
            alias_vec.push(format!(
                "alias {}{} {} <{}>",
                self.name.to_lowercase().replace(" ", "_"),
                param,
                self.name,
                email.email
            ));
        }
        alias_vec
    }
}

fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("rsvcf2mutt").unwrap();
    let xdg_config_path = xdg_dirs.place_config_file("config.toml").unwrap();
    let config = read_to_string(xdg_config_path)
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
    contacts.sort_by(|a, b| a.name.cmp(&b.name));
    let mut alias_vec: Vec<String> = Vec::new();
    for contact in contacts {
        alias_vec.append(&mut contact.create_mutt_alias());
    }
    let alias_string = alias_vec.join("\n");
    let out_file_name = format!(
        "{}/rsvcf2mutt_addressbook.muttrc",
        config["mutt_config_path"].as_str().unwrap()
    );
    match write(out_file_name, alias_string) {
        Ok(_) => (),
        Err(e) => panic!("file error! {}", e),
    };
}
