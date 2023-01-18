# runt

Runt is a polyglot task runner designed for managing menial tasks.
It automatically generates a CLI based off of a Runtfile in your
current working directory. This Runtfile is a simple markdown file
that contains a list of tasks and their descriptions. It even works
with nested commands. Check out the example [Runtfile](./Runtfile)
in this repo!

## Usage

To use runt, pass the name of a task to the runt command.

```bash
runt [task]
```
