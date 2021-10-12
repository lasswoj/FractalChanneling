use read_files::Config;




fn run_app() -> Result<(), ()> {
    let config = Config::from_file("cfg.toml").unwrap();
    print!("{:#?}",config);
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
