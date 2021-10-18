use access_files::{Config, Ticket};

fn run_app() -> Result<(), ()> {
    let config = Config::from_file("cfg.toml").unwrap();
    let tickets = Ticket::from_list_file("resources/db1.json").unwrap();
    print!("{:#?}", config);
    print!("{:#?}", tickets);
    Ok(())
}

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}
