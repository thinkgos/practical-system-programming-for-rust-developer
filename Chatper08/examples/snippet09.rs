use std::error;
use std::io::prelude::*;
use std::process::{Command, Stdio};

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut ps_output = String::new();
    // Spawn the `ps` command
    Command::new("ps")
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .unwrap()
        .read_to_string(&mut ps_output)?;

    print!("ps output from child process is:\n{}", ps_output);
    Ok(())
}
