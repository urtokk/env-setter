use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};

mod configuration;
mod env_variables;
mod execute;
mod init;
mod set;
mod utils;

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
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .help("path to configfile")
                .default_value(default_config.as_str()),
        )
        .subcommand(SubCommand::with_name("list"))
        .subcommand(
            SubCommand::with_name("set")
                .about("set a variable set")
                .arg(
                    Arg::with_name("env-set")
                        .index(1)
                        .help("Env set to use")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("pipe")
                        .short("f")
                        .long("file")
                        .default_value("/tmp/set-env")
                        .help("output file with env set"),
                )
                .arg(
                    Arg::with_name("stdout")
                        .short("s")
                        .long("stdout")
                        .help("print variable commands to stdout"),
                ),
        )
        .subcommand(SubCommand::with_name("init").about("Setup a easy example config"))
        .subcommand(
            SubCommand::with_name("execute")
                .about("execute provided command with target settings")
                .arg(
                    Arg::with_name("target")
                        .short("t")
                        .long("target")
                        .takes_value(true)
                        .required(true)
                        .help("Target set to use"),
                )
                .arg(
                    Arg::with_name("command")
                        .short("c")
                        .long("command")
                        .takes_value(true)
                        .required(true)
                        .help("command to be executed"),
                ),
        )
        .get_matches();

    let configfile = matches.value_of("config").unwrap();

    match matches.subcommand_name() {
        Some("set") => {
            let mut config = configuration::get_config(configfile);
            let matches = matches.subcommand_matches("set").unwrap();
            set::set(&mut config.sets, config.shell, &matches)
                .map_err(|e| {
                    eprintln!("Error setting environment: {}", e);
                    std::process::exit(6)
                })
                .ok();
        }
        Some("list") => {
            let config = configuration::get_config(configfile);
            for item in &config.sets {
                println!("{}", item.0);
            }
        }
        Some("init") => {
            init::init_config(configfile).ok();
        }
        Some("execute") => {
            let mut config = configuration::get_config(configfile);
            let matches = matches.subcommand_matches("execute").unwrap();
            let target_set = {
                let target = matches.value_of("target").unwrap();
                match utils::get_target_set(&mut config.sets, target) {
                    Ok(target) => target,
                    Err(e) => {
                        eprintln!("Could not determine target set: {}", e);
                        std::process::exit(7)
                    }
                }
            };
            let command = matches.value_of("command").unwrap();
            match execute::execute(target_set, command) {
                Ok(output) => println!("{}", output),
                Err(e) => {
                    eprintln!("Could not execute command: {}", e);
                    std::process::exit(8)
                }
            }
        }
        Some(s) => {
            eprintln!("Subcommand {} not supported", s);
            println!("{}", matches.usage());
            std::process::exit(2);
        }
        None => {
            println!("{}", matches.usage());
            std::process::exit(1);
        }
    }
}
