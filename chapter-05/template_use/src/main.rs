#[macro_use]
extern crate serde_derive;

use std::error::Error;
use tinytemplate::TinyTemplate;

#[derive(Serialize)]
struct PersonsData {
    persons: Vec<PersonData>,
}

#[derive(Serialize)]
struct PersonData {
    first_name: String,
    middle_initial: Option<char>,
    last_name: String,
    birth_year: u16,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut tt = TinyTemplate::new();
    tt.add_template("persons", include_str!("template.html"))?;
    let context = PersonsData {
        persons: vec![
            PersonData {
                first_name: "John".to_string(),
                middle_initial: None,
                last_name: "Doe".to_string(),
                birth_year: 1997,
            },
            PersonData {
                first_name: "Jane".to_string(),
                middle_initial: Some('A'),
                last_name: "Doe".to_string(),
                birth_year: 2004,
            },
            PersonData {
                first_name: "Marion".to_string(),
                middle_initial: Some('W'),
                last_name: "Smith".to_string(),
                birth_year: 1974,
            },
        ],
    };
    let rendered = tt.render("persons", &context)?;
    println!("{}", rendered);
    Ok(())
}
