use color_eyre::eyre::Result;
use std::collections::HashMap;
use clap::ArgMatches;

use crate::env_variables::EnvVariables;
use crate::env_variables::Shell;

pub fn set(
    config_sets: &mut HashMap<String, Vec<EnvVariables>>,
    shell: Shell,
    matches: &ArgMatches,
) -> Result<()> {
    let target_set = matches.value_of("env-set").unwrap().to_owned();
    let var_set = {
        let set = config_sets
        .get_mut(&target_set);

        match set {
            Some(s) => s,
            None => {
                eprintln!("Set {} was not found", &target_set);
                std::process::exit(3);
            }
        }
    };

    let mut output: Box<dyn std::io::Write> = {
        if matches.is_present("stdout") {
            Box::new(std::io::stdout())
        } else {
            let path = matches.value_of("pipe").unwrap();
            let file = std::fs::File::create(path);

            if let Ok(f) = file {
                Box::new(std::io::BufWriter::new(f))
            } else {
                eprintln!("Failed to create File: {}", path);
                std::process::exit(4);
            }
        }
    };
    for item in var_set.iter_mut() {
        item.ask_user_input();
    }

    for item in var_set {
        item.print_variables(&shell, output.as_mut())
        .map_err(|e| {
            eprintln!("Could not print variables: {}", e);
            std::process::exit(5)
        }).ok();
    }

    Ok(())
}