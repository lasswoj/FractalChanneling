pub mod acc_toml;
pub use acc_toml::Config;
pub use std::str::FromStr;
pub mod acc_json;
pub use acc_json::Ticket;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn toml_works() {

        let conf = Config::from_str(r#"
            [profile]
            name = 'xxxxxxxxxxxxxxxxx'
            password = 'yyyyyyyyyyyyyyyyy'
        "#).unwrap();
        assert_eq!(conf.profile.as_ref().unwrap().name, "xxxxxxxxxxxxxxxxx");
        assert_eq!(conf.profile.as_ref().unwrap().password, "yyyyyyyyyyyyyyyyy");
    }
    #[test]
    fn toml_works_from_file() {
        let conf = Config::from_file("test_resources/testconf.toml").unwrap();
        assert_eq!(conf.profile.as_ref().unwrap().name, "xxxxxxxxxxxxxxxxx");
        assert_eq!(conf.profile.as_ref().unwrap().password, "yyyyyyyyyyyyyyyyy");
    }
    #[test]
    fn json_works() {
        let v: Ticket = Ticket::from_str(r#"
        {
            "primary_key": 10,
            "unique_name": "json_reader",
            "description": "implement json ticket config reader 2.3",
            "story_points": 2,
            "child_tickets": [],
            "reporter": "lasswoj",
            "asignee": "lasswoj",
            "affected_modules": ["user_interface"],
            "affected_steps": ["*"],
            "relevant_changes": ["*"],
            "continued_in": 0,
            "blocked_by": [],
            "status":"pending",
            "priority": "high"
        }"#).unwrap();
        assert_eq!(v.primary_key, 10);
    }
    #[test]
    fn json_list_works() {
        let v  = Ticket::from_list_str(r#"
        [
            {
              "primary_key": 10,
              "unique_name": "json_reader",
              "description": "implement json ticket config reader 2.3",
              "story_points": 2,
              "child_tickets": [],
              "reporter": "lasswoj",
              "asignee": "lasswoj",
              "affected_modules": ["user_interface"],
              "affected_steps": ["*"],
              "relevant_changes": ["*"],
              "continued_in": 0,
              "blocked_by": [],
              "status":"pending",
              "priority": "high"
            },
            {
              "primary_key": 11,
              "unique_name": "local_server",
              "description": "implement server 2.5",
              "story_points": 2,
              "child_tickets": [],
              "reporter": "lasswoj",
              "asignee": "",
              "affected_modules": ["user_interface"],
              "affected_steps": [],
              "relevant_changes": ["*"],
              "continued_in": 0,
              "blocked_by": [],
              "status":"pending",
              "priority": "low"
            }]"#).unwrap();
        assert_eq!(v[0].primary_key, 10);
    }
    #[test]
    fn json_works_from_file() {
        let tickets = Ticket::from_list_file("test_resources/db.json").unwrap();
        assert_eq!(tickets[0].primary_key, 1);
        print!("{:#?} ", tickets)
        // assert_eq!(conf.profile.as_ref().unwrap().password, "yyyyyyyyyyyyyyyyy");
    }
}
