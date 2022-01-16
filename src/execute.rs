use crate::env_variables::EnvVariables;
use crate::utils;
use color_eyre::eyre::{eyre, Result};
use std::io::BufRead;
use std::{env::set_var, process};

pub fn execute<T: BufRead>(
    var_set: &mut Vec<EnvVariables>,
    command: &str,
    source: &mut T,
) -> Result<String> {
    for var in var_set.iter_mut() {
        if let Some(s) = utils::get_input(
            format!(
                "#{}[{}]: ",
                var.name,
                var.value.as_ref().unwrap_or(&"".to_owned())
            )
            .as_str(),
            source,
        ) {
            var.value = Some(s);
        }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration;
    use std::io::BufReader;

    #[test]
    fn test_execute_fish_with_input() {
        let mut prepared_input = BufReader::new("test\n".as_bytes());
        let mut config = configuration::get_config("resources/test.yaml");
        let mut target_set = {
            match crate::utils::get_target_set(&mut config.sets, "another-set") {
                Ok(s) => s,
                Err(e) => panic!("{}", e),
            }
        };

        let output = execute(&mut target_set, "echo 1", &mut prepared_input).unwrap();

        assert_eq!(output, "1\n");
    }

    #[test]
    fn test_fish_env_var() {
        let mut prepared_input = BufReader::new("test\n".as_bytes());
        let mut config = configuration::get_config("resources/test.yaml");
        let mut target_set = {
            match crate::utils::get_target_set(&mut config.sets, "another-set") {
                Ok(s) => s,
                Err(e) => panic!("{}", e),
            }
        };

        let output = execute(
            &mut target_set,
            "bash resources/scripts/echo_anothertest.sh",
            &mut prepared_input,
        )
        .unwrap();

        assert_eq!(output, "test\n");
    }
}
