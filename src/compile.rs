use crate::runtfile::{Command, Runtfile};
use clap::{Arg, ArgMatches, Command as ClapCommand};

pub fn build_cli_from_runtfile(runtfile: &Runtfile) -> ClapCommand {
    let root_cmd = runtfile.root_command();
    let mut cli = ClapCommand::new(root_cmd.name.clone()).about(root_cmd.description.clone());
    cli = cli.name(root_cmd.name.clone());

    for c in root_cmd.subcommands.iter() {
        cli = cli.subcommand(build_command(runtfile, runtfile.command(*c)));
    }
    cli
}

fn build_command(runtfile: &Runtfile, cmd: &Command) -> ClapCommand {
    let mut subcommand = ClapCommand::new(cmd.name.clone()).about(cmd.description.clone());
    if !cmd.code.is_empty() {
        subcommand = subcommand
            .arg(Arg::new("--").help("Arguments coming after this are passed to the script."));
    }

    for subcommand_index in cmd.subcommands.iter() {
        subcommand =
            subcommand.subcommand(build_command(runtfile, runtfile.command(*subcommand_index)));
    }
    subcommand
}

pub fn match_command(
    runtfile: &Runtfile,
    matches: &ArgMatches,
    cmd: &Command,
) -> (Option<Command>, Option<String>) {
    let (cmd, sub_matches) = if cmd.name == "runt" {
        (cmd, matches)
    } else if let Some(sub_matches) = matches.subcommand_matches(cmd.name.as_str()) {
        (cmd, sub_matches)
    } else {
        return (None, None);
    };

    // check if it has any subcommands, and if it does, return the first one that matches
    for subcommand_index in cmd.subcommands.iter() {
        let m = match_command(runtfile, sub_matches, runtfile.command(*subcommand_index));
        if m.0.is_some() {
            return m;
        }
    }
    return (
        Some(cmd.clone()),
        sub_matches
            .try_get_one::<String>("--")
            .ok()
            .flatten()
            .cloned(),
    );
}
