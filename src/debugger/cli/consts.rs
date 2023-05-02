use super::{Command, CommandAlias};

// TODO: Fill in help text

pub static COMMANDS: [Command; 4] = [
    Command {
        name: "continue",
        help: "continue command help",
        short_help: "continue command short help",
    },
    Command {
        name: "breakpoint",
        help: "breakpoint command help",
        short_help: "breakpoint command short help",
    },
    Command {
        name: "print",
        help: "print command help",
        short_help: "print command short help",
    },
    Command {
        name: "quit",
        help: "quit command help",
        short_help: "Stops current debugging session and exits etxdb.",
    },
];

pub static COMMAND_ALIASES: [CommandAlias; 1] = [CommandAlias {
    name: "c",
    aliased_to: "continue",
}];

pub mod quit_choice {
    pub const STOP_AND_QUIT: &str = "Yes, stop and quit";
    pub const QUIT: &str = "Yes, quit but don't stop";
    pub const ABORT: &str = "No, abort!";
}
