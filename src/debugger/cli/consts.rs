use super::Command;
use lazy_static::lazy_static;

pub const COMMANDS: [Command; 4] = [
    Command {
        name: "continue",
        shorthand: Some("c"),
        help: "continue command help text", // TODO: Fill
    },
    Command {
        name: "breakpoint",
        shorthand: Some("b"),
        help: "breakpoint command help text", // TODO: Fill
    },
    Command {
        name: "print",
        shorthand: Some("p"),
        help: "print command help text", // TODO: Fill
    },
    Command {
        name: "quit",
        shorthand: Some("q"),
        help: "Stops current debugging session and exits etxdb.",
    },
];

lazy_static! {
    pub static ref VALID_COMMANDS: Vec<&'static str> = {
        COMMANDS
            .into_iter()
            .flat_map(|command| {
                let mut vec = vec![command.name];
                if command.shorthand.is_some() {
                    vec.push(command.shorthand.unwrap());
                }
                vec
            })
            .collect()
    };
}

pub const QUIT_YES_CHOICE: &str = "Yes, stop and quit";
pub const QUIT_NO_CHOICE: &str = "No, abort!";
