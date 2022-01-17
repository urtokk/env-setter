use clap::ArgMatches;
use color_eyre::eyre::Result;

use crate::env_variables::EnvVariables;
use crate::env_variables::Shell;
use crate::utils;

pub fn set<R,W>(
    var_set: &mut Vec<EnvVariables>,
    shell: Shell,
    source: &mut R,
    target: &mut W,
) -> Result<()>
where
    R: std::io::BufRead,
    W: std::io::Write,
 {
    let target_set = matches.value_of("env-set").unwrap().to_owned();
    let var_set = {
        let set = config_sets.get_mut(&target_set);

        match set {
            Some(s) => s,
            None => {
                eprintln!("Set {} was not found", &target_set);
                std::process::exit(3);
            }
        }
    };

    for item in var_set.iter_mut() {
        if let Some(s) = utils::get_input(
            format!(
                "#{}[{}]: ",
                item.name,
                item.value.as_ref().unwrap_or(&"".to_owned())
            )
            .as_str(),
            source,
        ) {
            item.value = Some(s);
        }
    }

    for item in var_set {
        item.print_variables(&shell, &mut target)
            .map_err(|e| {
                eprintln!("Could not print variables: {}", e);
                std::process::exit(5)
            })
            .ok();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
