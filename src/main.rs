use runt::compile::{build_cli_from_runtfile, match_command};
use runt::eval;
use runt::parser::parse;
use runt::runtfile::{Script, ABOUT};

use std::fs::File;
use std::io::{self, Read};

use clap::Command as ClapCommand;

fn read_runtfile() -> io::Result<String> {
    let mut file = File::open("Runtfile")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub(crate) fn run(runtfile_source: Option<String>) -> Result<(), io::Error> {
    if let Some(runtfile) = runtfile_source {
        let runtfile = parse(&runtfile);
        let cli = build_cli_from_runtfile(&runtfile);
        let matches = cli.get_matches();
        let (cmd, args) = match_command(&runtfile, &matches, runtfile.root_command());
        let cmd = cmd.unwrap();
        match cmd.script {
            Script::Javascript => eval::javascript(&cmd.code, args),
            Script::Python => eval::python(&cmd.code, args),
            Script::Bash => eval::bash(&cmd.code, args),
            Script::Ruby => eval::ruby(&cmd.code, args),
        }
    } else {
        let cli = ClapCommand::new("runt").about(ABOUT);
        let warning = format!(
            "{}\n👷 A Runtfile wasn't detected in the current directory. No commands loaded.",
            ABOUT
        );
        cli.about(warning).print_help()
    }
}

fn main() -> io::Result<()> {
    run(read_runtfile().ok())
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use runt::{compile, runtfile::Script};

    use dedent_macro::dedent;
    #[test]
    fn parses_markdown_command() {
        let runtfile = parse(dedent!(
            "
            # Chocolate

            ## Rain

            Some stay high while others feel the pain

            ```sh
            echo chocolate rain
            ```
        "
        ));

        // Test that we parse it correctly
        println!("{:?}", runtfile);
        assert_eq!(runtfile.root_command().name, "chocolate".to_string());
        let command = runtfile.find_command_by_name("rain");
        assert!(command.is_some());
        let command = command.unwrap();
        assert_eq!(command.script, Script::Bash);

        // Test that we generate the correct CLI
        let cli = compile::build_cli_from_runtfile(&runtfile);
        assert_eq!(cli.get_name(), "chocolate".to_string());

        let subcommand = cli.find_subcommand("rain");
        assert!(subcommand.is_some());
        let subcommand = subcommand.unwrap();

        let about = subcommand.get_about();
        assert!(about.is_some());
        let about = about.unwrap();
        assert!(about.to_string().contains("feel the pain"));
    }
}
