use super::Command;
use lazy_static::lazy_static;

lazy_static! {

pub static ref COMMANDS: [Command<'static>; 4] = [
    Command {
        name: "continue",
        help: "continue command help text",
        aliases: vec!["c"],
    },
    Command {
        name: "breakpoint",
        aliases: vec!["b"],
        help: "breakpoint command help text", // TODO: Fill
    },
    Command {
        name: "print",
        aliases: vec!["p"],
        help: "print command help text", // TODO: Fill
    },
    Command {
        name: "quit",
        aliases: vec!["q"],
        help: "Stops current debugging session and exits etxdb.",
    },
];
    pub static ref VALID_COMMANDS: Vec<&'static str> = {
        COMMANDS
            .iter()
            .flat_map(|command| {
                let mut vec = vec![command.name];
                let mut aliases = command.aliases.clone();
                vec.append(&mut aliases);
                vec
            })
            .collect()
    };
}

pub const QUIT_STOP_YES_CHOICE: &str = "Yes, stop and quit";
pub const QUIT_YES_CHOICE: &str = "Yes, quit but don't stop";
pub const QUIT_NO_CHOICE: &str = "No, abort!";
