use std::{
    io::{self, Write},
    process,
};

pub fn javascript(code: &str, args: Option<String>) -> io::Result<()> {
    let mut cmd = process::Command::new("node");
    let mut child = cmd.arg("-");

    if let Some(args) = args {
        child = child.arg(args);
    }

    let mut child = child
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

pub fn python(code: &str, args: Option<String>) -> io::Result<()> {
    let mut cmd = process::Command::new("python");
    let mut child = cmd.arg("-");

    if let Some(args) = args {
        child = child.arg(args);
    }

    let mut child = child
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

pub fn bash(code: &str, args: Option<String>) -> io::Result<()> {
    let mut cmd = process::Command::new("bash");
    let mut child = cmd.arg("-s").arg("-");

    if let Some(args) = args {
        child = child.arg(args);
    }

    let mut child = child
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

pub fn ruby(_code: &str, _args: Option<String>) -> io::Result<()> {
    todo!("Ruby support is not yet implemented.");
}
