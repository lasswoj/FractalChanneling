use serde::Deserialize;
use std::fs;
use std::str::FromStr;
use toml::de::Error;

#[derive(Deserialize, Debug)]
pub struct Profile {
    pub name: String,
    pub password: String,
}

pub type ParseError = Error;
// pub type ParseError = Infallible;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub profile: Option<Profile>,
}

impl FromStr for Config {
    type Err = ParseError;
    fn from_str(input: &str) -> Result<Self, ParseError> {
        return toml::from_str(input);
    }
}

impl Config {
    pub fn from_file(filename: &str) -> Result<Self, ParseError> {
        let data = fs::read_to_string(filename).expect("Unable to read file");
        let ret = Config::from_str(&*data);
        return ret;
    }
}
