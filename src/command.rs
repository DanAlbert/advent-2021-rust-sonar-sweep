#[derive(Debug, Eq, PartialEq)]
pub enum Action {
    FORWARD,
    UP,
    DOWN,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    pub action: Action,
    pub value: i32,
}

impl Command {
    fn new(action: Action, value: i32) -> Command {
        Command { action, value }
    }

    fn parse(line: &str) -> Result<Command, String> {
        let items: Vec<&str> = line.split_whitespace().collect();
        if items.len() != 2 {
            return Err(format!("Invalid command: \"{}\"", line));
        }

        let action_str = &items[0];
        let value_str = &items[1];
        let value = value_str
            .parse::<i32>()
            .map_err(|_| format!("Invalid command value: \"{}\"", value_str))?;

        let action = match action_str {
            &"up" => Ok(Action::UP),
            &"down" => Ok(Action::DOWN),
            &"forward" => Ok(Action::FORWARD),
            _ => Err(format!("Unknown action \"{}\"", action_str)),
        }?;

        Ok(Command::new(action, value))
    }

    pub fn parse_each<'a>(
        lines: impl IntoIterator<Item = &'a str>,
    ) -> Result<Vec<Command>, String> {
        lines.into_iter().map(|l| Command::parse(l)).collect()
    }
}

#[test]
fn test_parse() {
    assert_eq!(Ok(Command::new(Action::UP, 2)), Command::parse("up 2"));
    assert_eq!(Ok(Command::new(Action::DOWN, 0)), Command::parse("down 0"));
    assert_eq!(
        Ok(Command::new(Action::FORWARD, -1)),
        Command::parse("forward -1")
    );

    assert_eq!(
        Err(format!("Unknown action \"backward\"")),
        Command::parse("backward 1")
    );

    assert_eq!(Err(format!("Invalid command: \"\"")), Command::parse(""));
    assert_eq!(
        Err(format!("Invalid command: \"up 1 down 2\"")),
        Command::parse("up 1 down 2")
    );
    assert_eq!(
        Err(format!("Invalid command value: \"a\"")),
        Command::parse("up a")
    );
}

#[test]
fn test_parse_each() {
    assert_eq!(
        Ok(vec!(
            Command::new(Action::UP, 1),
            Command::new(Action::DOWN, 2)
        )),
        Command::parse_each(vec!("up 1", "down 2"))
    );

    assert_eq!(Ok(vec!()), Command::parse_each(vec!()));

    assert_eq!(
        Err(format!("Unknown action \"backward\"")),
        Command::parse_each(vec!("up 1", "backward 2"))
    );
}
