use std::fs::File;
use std::io::{self, Write, Read};
use std::process;

use clap::{ArgMatches, Command as ClapCommand};
use comrak::nodes::{AstNode, NodeValue};
use comrak::{parse_document, Arena as TypedArena, ComrakOptions};
use la_arena::{Arena, Idx};

fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &mut F)
where
    F: FnMut(&'a AstNode<'a>),
{
    f(node);
    for c in node.children() {
        iter_nodes(c, f);
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Command {
    name: String,
    description: String,
    script: Script,
    code: String,
    level: u32,
    subcommands: Vec<Idx<Command>>,
}

#[derive(Debug, Clone, PartialEq)]
struct Runtfile {
    commands: Arena<Command>,
}

impl Runtfile {
    fn add_command(&mut self, command: Command) -> Idx<Command> {
        self.commands.alloc(command)
    }

    fn find_parent(&self, level: u32) -> Option<Idx<Command>> {
        for (index, command) in self.commands.iter() {
            if command.level == level - 1 {
                return Some(index);
            }
        }
        None
    }

    fn command(&self, index: Idx<Command>) -> &Command {
        &self.commands[index]
    }

    fn command_mut(&mut self, index: Idx<Command>) -> &mut Command {
        &mut self.commands[index]
    }

    fn root_command(&self) -> &Command {
        &self.commands.iter().next().unwrap().1
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Script {
    Bash,
    Python,
    Javascript,
    Ruby,
}

/**
 *
 */
fn parse(input: &str) -> Runtfile {
    let mut runtfile = Runtfile {
        commands: Arena::new(),
    };

    // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
    let arena = TypedArena::new();

    let root = parse_document(&arena, input, &ComrakOptions::default());

    let root_cmd = runtfile.add_command(Command {
        name: String::from(""),
        description: String::from(""),
        script: Script::Bash,
        code: String::from(""),
        level: 1,
        subcommands: vec![],
    });

    let mut cmd_idx: Idx<Command> = root_cmd;

    iter_nodes(root, &mut |node| match &node.data.borrow().value {
        NodeValue::Text(text) => {
            let cmd = runtfile.command_mut(cmd_idx);

            if cmd.name.is_empty() {
                cmd.name = String::from_utf8(text.clone())
                    .unwrap()
                    .replace(" ", "-")
                    .to_lowercase();
            } else {
                cmd.description = String::from_utf8(text.clone()).unwrap();
            }
        }
        NodeValue::Heading(heading) => {
            if let Some(parent_index) = runtfile.find_parent(heading.level) {
                let index = runtfile.add_command(Command {
                    name: String::from(""),
                    description: String::from(""),
                    script: Script::Bash,
                    code: String::from(""),
                    level: heading.level,
                    subcommands: vec![],
                });
                let parent = runtfile.command_mut(parent_index);
                parent.subcommands.push(index.clone());
                cmd_idx = index;
            }
        }
        NodeValue::CodeBlock(code) => {
            let cmd = runtfile.command_mut(cmd_idx);
            cmd.code = String::from_utf8(code.literal.clone()).unwrap();
            cmd.script = match String::from_utf8(code.info.clone()).unwrap().as_str() {
                "bash" => Script::Bash,
                "python" => Script::Python,
                "javascript" => Script::Javascript,
                "ruby" => Script::Ruby,
                _ => Script::Bash,
            };
        }
        _ => (),
    });
    runtfile
}

fn build_cli_from_runtfile(runtfile: &Runtfile) -> ClapCommand {
    let mut cli = ClapCommand::new("runt");
    cli = cli.subcommand(build_command(runtfile, runtfile.root_command()));
    cli
}

fn build_command(runtfile: &Runtfile, cmd: &Command) -> ClapCommand {
    let mut subcommand = ClapCommand::new(cmd.name.clone());

    subcommand = subcommand.about(cmd.description.clone());
    for subcommand_index in cmd.subcommands.iter() {
        subcommand =
            subcommand.subcommand(build_command(runtfile, runtfile.command(*subcommand_index)));
    }
    subcommand
}

fn match_command(runtfile: &Runtfile, matches: &ArgMatches, cmd: &Command) -> Option<Command> {
    if let Some(subcommand) = matches.subcommand_matches(cmd.name.as_str()) {
        // check if it has any subcommands, and if it does, return the first one that matches
        for subcommand_index in cmd.subcommands.iter() {
            let m = match_command(runtfile, subcommand, runtfile.command(*subcommand_index));
            if m.is_some() {
                return m;
            }
        }
        return Some(cmd.clone());
    } else {
        return None;
    }
}

fn run_javascript(code: &str) -> io::Result<()> {
    let mut child = process::Command::new("node")
        .arg("-")
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(code.as_bytes())?;
    }

    let output = child.wait_with_output()?;
    println!("{}", String::from_utf8(output.stdout).unwrap());
    Ok(())
}

fn run_python(code: &str) -> io::Result<()> {
    let mut child = process::Command::new("python")
        .arg("-")
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(code.as_bytes())?;
    }
    let output = child.wait_with_output()?;
    println!("{}", String::from_utf8(output.stdout).unwrap());
    Ok(())
}

fn run_bash(code: &str) -> io::Result<()> {
    let mut child = process::Command::new("bash")
        .arg("-")
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(code.as_bytes())?;
    }
    let output = child.wait_with_output()?;
    println!("{}", String::from_utf8(output.stdout).unwrap());
    Ok(())
}

fn run_ruby(_code: &str) -> io::Result<()> {
    todo!("run ruby code");
}

fn read_runtfile() -> io::Result<String> {
    let mut file = File::open("Runtfile")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> io::Result<()> {
    let runtfile = parse(&read_runtfile()?);
    let cli = build_cli_from_runtfile(&runtfile);

    let matches = cli.get_matches();

    let cmd = match_command(&runtfile, &matches, &runtfile.root_command()).unwrap();

    match cmd.script {
        Script::Javascript => run_javascript(&cmd.code),
        Script::Python => run_python(&cmd.code),
        Script::Bash => run_bash(&cmd.code),
        Script::Ruby => run_ruby(&cmd.code),
    }
}
