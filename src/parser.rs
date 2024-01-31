use comrak::nodes::{AstNode, NodeValue};
use comrak::{parse_document, Arena as TypedArena, ComrakOptions};
use la_arena::{Arena, Idx};

use crate::runtfile::{Command, Runtfile, Script};

/// Parses the Runtfile source into a code-level representation.
/// This is then used further on to generate a clap CLI.
pub fn parse(input: &str) -> Runtfile {
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
                cmd.name = text.replace(' ', "-").to_lowercase();
            } else {
                cmd.description = text.clone();
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
                parent.subcommands.push(index);
                cmd_idx = index;
            }
        }
        NodeValue::CodeBlock(code) => {
            let cmd = runtfile.command_mut(cmd_idx);
            cmd.code = code.literal.clone();
            cmd.script = match code.info.clone().as_str() {
                "bash" => Script::Bash, // TODO: support other shells
                "python" => Script::Python,
                "javascript" | "js" => Script::Javascript,
                "ruby" => Script::Ruby,
                _ => Script::Bash,
            };
        }
        _ => (),
    });
    runtfile
}

fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &mut F)
where
    F: FnMut(&'a AstNode<'a>),
{
    f(node);
    for c in node.children() {
        iter_nodes(c, f);
    }
}
