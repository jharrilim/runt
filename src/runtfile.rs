use la_arena::{Arena, Idx};

pub const ABOUT: &str = "Run commands defined in a Runtfile.";

#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub script: Script,
    pub code: String,
    pub level: u8,
    pub subcommands: Vec<Idx<Command>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Runtfile {
    pub commands: Arena<Command>,
}

impl Runtfile {
    pub fn add_command(&mut self, command: Command) -> Idx<Command> {
        self.commands.alloc(command)
    }

    pub fn find_parent(&self, level: u8) -> Option<Idx<Command>> {
        for (index, command) in self.commands.iter() {
            if command.level == level - 1 {
                return Some(index);
            }
        }
        None
    }

    pub fn command(&self, index: Idx<Command>) -> &Command {
        &self.commands[index]
    }

    pub fn command_mut(&mut self, index: Idx<Command>) -> &mut Command {
        &mut self.commands[index]
    }

    pub fn find_command_by_name(&self, name: &str) -> Option<&Command> {
        self.commands.iter().find(|c| c.1.name == name).map(|c| c.1)
    }

    pub fn root_command(&self) -> &Command {
        self.commands.iter().next().unwrap().1
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Script {
    Bash,
    Python,
    Javascript,
    Ruby,
}
