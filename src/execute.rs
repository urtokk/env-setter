use crate::env_variables::EnvVariables;
use color_eyre::eyre::{eyre, Result};
use std::{env::set_var, process};

pub fn execute(var_set: &mut Vec<EnvVariables>, command: &str) -> Result<String> {
    for var in var_set.iter_mut() {
        var.ask_user_input();
        set_var(&var.name, var.value.as_ref().unwrap_or(&" ".to_owned()));
    }

    let mut command_iter = command.split(' ');
    let executable = match command_iter.next() {
        Some(s) => s,
        None => return Err(eyre!("No command specified")),
    };

    let mut execute_command = process::Command::new(executable);

    for s in command_iter {
        execute_command.arg(s);
    }

    let output = execute_command.output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(eyre!(
            "Command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}
