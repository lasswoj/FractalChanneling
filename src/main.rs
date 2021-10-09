use std::collections::HashMap;
use std::str::FromStr;
use serde_json::{Value};
// use confy::{Serialize, Deserialize};
use std::{fs, string};

mod data_ops;
mod interface;
use interface::cmdl::CMDActions;
use interface::base::ActionsTr;

use std::io::{stdin,stdout,Write};
struct Todo {
    // use rust built in HashMap to store key - val pairs
    map: HashMap<String, bool>,
}




struct ticket {
    raporter: Value,
    t_type: Value,
    asignee: Value,
    blocked_by: Value,
    name: Value,
    description: Value,
    child: Value,
    Priority: Value,
    Status: Value
}



use std::fs::File;
use std::io::Read;

fn json() {
    let file = File::open("text.json")
        .expect("file should open read only");
    let json: serde_json::Value = serde_json::from_reader(file)
        .expect("file should be proper JSON");
    let totalHits = json.get("totalHits")
        .expect("file should have totalHits key");
        print!("{}",totalHits)
}

impl Todo {
    fn insert(&mut self, key: String) {
        // insert a new item into our map.
        // we pass true as value
        self.map.insert(key, true);
    }

    // [rest of the code]
    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)
    }
    
    fn new() -> Result<Todo, std::io::Error> {
        let mut content = std::fs::read_to_string("db.txt")?;
        let map: HashMap<String, bool> = content
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect();
        Ok(Todo { map })
    }
}







fn run_app() -> Result<(), ()> {
    let action = CMDActions::get_action();
    Ok(())
}

fn main() {

    // std::process::exit(match run_app() {
    //     Ok(_) => 0,
    //     Err(err) => {
    //         eprintln!("error: {:?}", err);
    //         1
    //     }
    // });



    // let go = Actions::from_str(&*std::env::args().nth(1).unwrap()).unwrap();
    // read_action_input();

    // let action = match std::env::args().nth(1){
    //     Some(arg) => arg,
    //     None =>retinput("provide action")
    // };

    // let action = match std::env::args().nth(1){
    //     Some(arg) => arg,
    //     None =>
    // };

    // enum Actions {
    //     Add,
    //     Remove,
    //     Select,
    //     Put,
    //     Append,
    //     Read
    // }



    // let item = match std::env::args().nth(1){
    //     Some(arg) => arg,
    //     None =>retinput("provide items")
    // };
    
    // let item = std::env::args().nth(2).expect("Please specify an item");
    // let mut todo = Todo::new().unwrap();
    // if action == "add" {
    //     todo.insert(item);
    //     match todo.save() {
    //         Ok(_) => println!("todo saved"),
    //         Err(why) => println!("An error occurred: {}", why),
    //     }
    // } 
}
