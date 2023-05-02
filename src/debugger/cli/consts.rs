use lazy_static::lazy_static;

use super::{commands, Command};

// TODO: Fill in help text

lazy_static! {
    pub static ref COMMANDS: Vec<Command<'static>> = vec![
        Command {
            name: "continue",
            help: "continue command help",
            short_help: "continue command short help",
            handler: commands::continue_command,
        },
        Command {
            name: "breakpoint",
            help: "breakpoint command help",
            short_help: "breakpoint command short help",
            handler: commands::breakpoint_command,
        },
        Command {
            name: "print",
            help: "print command help",
            short_help: "print command short help",
            handler: commands::print_command,
        },
        Command {
            name: "quit",
            help: "quit command help",
            short_help: "Stops current debugging session and exits etxdb.",
            handler: commands::quit_command,
        },
    ];
    // uncomment when it becomes needed
    // pub static ref COMMAND_ALIASES: Vec<CommandAlias<'static>> = vec![CommandAlias {
    //     name: "c",
    //     aliased_to: "continue",
    // }];
}

pub mod quit_choice {
    pub const STOP_AND_QUIT: &str = "Yes, stop and quit";
    pub const QUIT: &str = "Yes, quit but don't stop";
    pub const ABORT: &str = "No, abort!";
}
