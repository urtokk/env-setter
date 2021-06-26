use color_eyre::eyre::{
    Result,
    eyre
};
use std::{
    process,
    env::set_var
};
use crate::env_variables::EnvVariables;


pub fn execute(var_set: &Vec<EnvVariables>, command: &str) -> Result<String> {
    for var in var_set.iter() {
        set_var(&var.name, var.value.as_ref().unwrap_or(&" ".to_owned()));
    }

    let mut command_iter = command.split(" ");
    let executable = match command_iter.nth(0) {
        Some(s) => s,
        None => return Err(eyre!("No command specified"))
    };

    let mut execute_command = process::Command::new(executable);

    for s in command_iter {
        execute_command.arg(s);
    }

    let output = execute_command.output()?;

    Ok(String::from_utf8(output.stdout)?)
}