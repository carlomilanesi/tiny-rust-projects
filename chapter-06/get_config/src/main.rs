use config::Config;
use std::collections::{BTreeMap, HashMap};
use std::error::Error;

const BASE_SETTINGS_PATH: &str = "base.json5";
const USER_FIXED_SETTINGS_PATH: &str = "fixed.toml";
const USER_CHANGEABLE_SETTINGS_PATH: &str = "changeable.yaml";

struct Configuration {
    settings: HashMap<String, String>,
}

impl Configuration {
    fn load() -> Result<Self, Box<dyn Error>> {
        let builder = Config::builder()
            .set_default("1", "a")?
            .set_default("2", "b")?
            .set_default("3", "c")?
            .set_default("4", "d")?
            .set_default("5", "e")?
            .set_default("6", "f")?
            .set_default("7", "g")?
            .set_default("8", "h")?
            .add_source(config::File::with_name(BASE_SETTINGS_PATH))
            .add_source(config::File::with_name(USER_FIXED_SETTINGS_PATH).required(false))
            .add_source(config::File::with_name(USER_CHANGEABLE_SETTINGS_PATH).required(false))
            // Upper or lower, without ending underscore
            .add_source(config::Environment::default().prefix("USE_CONFIG"));
        Ok(Self {
            settings: builder
                .build()?
                .try_deserialize::<HashMap<String, String>>()?,
        })
    }

    fn print(&self) {
        println!("=== Settings ===");
        for setting in self.settings.iter().collect::<BTreeMap<_, _>>() {
            println!("{} = {}", setting.0, setting.1);
        }
        println!();
    }

    fn update(&mut self) {
        self.settings.insert("1".to_string(), "q".to_string());
        self.settings.insert("3".to_string(), "r".to_string());
        self.settings.insert("5".to_string(), "s".to_string());
        self.settings.insert("7".to_string(), "t".to_string());
    }

    fn save(&self) -> Result<(), Box<dyn Error>> {
        serde_yaml::to_writer(
            std::fs::File::create(USER_CHANGEABLE_SETTINGS_PATH)?,
            &self.settings.iter().collect::<BTreeMap<_, _>>(),
        )?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = std::fs::remove_file(USER_CHANGEABLE_SETTINGS_PATH);
    let mut settings1 = Configuration::load()?;
    settings1.print();
    settings1.update();
    settings1.save()?;
    let settings2 = Configuration::load()?;
    settings2.print();
    Ok(())
}
