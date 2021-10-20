use serde_json::{Error};
use serde::{Deserialize};
use std::fs;
use std::str::FromStr;


#[derive(Deserialize, Debug)]
pub struct Ticket{
    pub primary_key: i32,
    pub unique_name: String,
    pub description: String,
    pub story_points: i32,
    pub child_tickets: Option<Vec<Ticket>>,
    pub reporter: String,
    pub asignee: String,
    pub affected_modules: Vec<String>,
    pub affected_steps: Vec<String>,
    pub relevant_changes: Vec<String>,
    pub status: String,
}

impl FromStr for Ticket {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Error> {
        let data: Result<Ticket, Error> = serde_json::from_str(input);
        data
    }
}


impl Ticket{
    pub fn from_list_file(filename: &str)-> Result<Vec<Self>, Error>{
        let data = fs::read_to_string(filename).expect("Unable to read file");
        let ret = Ticket::from_list_str(&*data);
        return ret;
    }
    pub fn from_list_str(input: &str) -> Result<Vec<Self>, Error> {
        let data: Result<Vec<Ticket>, Error> = serde_json::from_str(input);
        data
    }
}
