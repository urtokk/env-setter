use clap::{
    crate_authors,
    crate_description,
    crate_name,
    crate_version,
    App,
    Arg,
    SubCommand,
};

use std::collections::HashMap;
use serde_derive::{
    Serialize,
    Deserialize
};

mod env_variables;

use env_variables::{
    EnvVariables,
    Shell
};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    shell: Option<Shell>,
    sets: HashMap<String,Vec<EnvVariables>>,
}

fn main() {
    let default_config = {
        let home = std::env::var("HOME").unwrap_or(String::from("~"));
        let path_config = format!("{}/.config/env-setter.yaml", home);
        path_config
    };

    let matches = App::new(crate_name!())
                        .version(crate_version!())
                        .author(crate_authors!())
                        .about(crate_description!())
                        .arg(Arg::with_name("config")
                            .short("c")
                            .long("config")
                            .help("path to configfile")
                            .default_value(default_config.as_str()))
                        .subcommand(SubCommand::with_name("list"))
                        .subcommand(SubCommand::with_name("set")
                                .about("set a variable set")
                                .arg(Arg::with_name("env-set")
                                .index(1)
                                .help("Env set to use")
                                .takes_value(true))
                            .arg(Arg::with_name("pipe")
                                .short("f")
                                .long("file")
                                .default_value("/tmp/set-env")
                                .help("output file with env set"))
                            .arg(Arg::with_name("stdout")
                                .short("s")
                                .long("stdout")
                                .help("print variable commands to stdout")))
                        .get_matches();
    let configfile = matches.value_of("config").unwrap();

    let mut config = {
        let mut configger = config::Config::default();
        configger.merge(config::File::with_name(configfile)).unwrap();
        configger.try_into::<Config>().unwrap()
    };

    if config.shell.is_none() {
        config.shell = Some(Shell::Posix);
    }

    match matches.subcommand_name() {
        Some("set") => {
            let matches = matches.subcommand_matches("set").unwrap();
            let env_set = matches.value_of("env-set").unwrap().to_owned();
            let var_set = {
                let set = config
                .sets
                .get_mut(&env_set);

                match set {
                    Some(s) => s,
                    None => {
                        eprintln!("Set {} was not found", env_set);
                        std::process::exit(2);
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
                        std::process::exit(3);
                    }
                }
            };
            for item in var_set.iter_mut() {
                item.ask_user_input();
            }

            for item in var_set {
                item.print_variables(&config.shell.as_ref().unwrap(), output.as_mut())
                .map_err(|e| {
                    eprintln!("Could not print variables: {}", e);
                    std::process::exit(4)
                }).ok();
            }
        },
        Some("list") => {
            for item in config.sets {
                println!("{}", item.0);
            }
        },
        Some(s) => {
            eprintln!("Subcommand {} not supported", s);
            println!("{}", matches.usage());
            std::process::exit(5);
        }
        None => {
            println!("{}",matches.usage());
            std::process::exit(1);
        }
    }
}
