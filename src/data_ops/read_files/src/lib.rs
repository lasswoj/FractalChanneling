pub mod config;
pub use config::Config;
pub use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {

        let conf = Config::from_str(r#"
            [profile]
            name = 'xxxxxxxxxxxxxxxxx'
            password = 'yyyyyyyyyyyyyyyyy'
        "#).unwrap();
        assert_eq!(conf.profile.as_ref().unwrap().name, "xxxxxxxxxxxxxxxxx");
        assert_eq!(conf.profile.as_ref().unwrap().password, "yyyyyyyyyyyyyyyyy");
    }
    #[test]
    fn it_works_from_file() {
        let conf = Config::from_file("test resources/testconf.toml").unwrap();
        assert_eq!(conf.profile.as_ref().unwrap().name, "xxxxxxxxxxxxxxxxx");
        assert_eq!(conf.profile.as_ref().unwrap().password, "yyyyyyyyyyyyyyyyy");
    }
}
