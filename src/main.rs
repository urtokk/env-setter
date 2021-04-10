use clap::{
    crate_authors,
    crate_description,
    crate_name,
    crate_version,
    App,
    Arg
};

use read_input::prelude::*;

use std::{borrow::BorrowMut, collections::HashMap};
use serde_derive::{
    Serialize,
    Deserialize
};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    shell: Shell,
    sets: HashMap<String,Vec<EnvVariables>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EnvVariables {
    name: String,
    value: Option<String>,

}

#[derive(Debug, Serialize, Deserialize)]
enum Shell {
    Fish,
    Posix,
}

fn main() {
    let matches = App::new(crate_name!())
                        .version(crate_version!())
                        .author(crate_authors!())
                        .about(crate_description!())
                        .arg(Arg::with_name("config")
                            .short("c")
                            .long("config")
                            .help("path to configfile")
                            .default_value(".config/env-setter.yaml")
                            .index(2))
                        .arg(Arg::with_name("env-set")
                            .short("e")
                            .long("env-set")
                            .index(1)
                            .help("Env set to use"))
                        .get_matches();
    let configfile = matches.value_of("config").unwrap();
    let env_set = matches.value_of("env-set").unwrap().to_owned();

    let config = {
        let mut configger = config::Config::default();
        configger.merge(config::File::with_name(configfile)).unwrap();
        configger.try_into::<Config>().unwrap()
    };

    let var_set = config
                    .sets
                    .get_key_value(&env_set);
    let mut var_output: Vec<EnvVariables> = Vec::new();

    match var_set {
        Some((set, vec_var)) => {
            println!("using variable set {}", set);
            for var in vec_var {
                get_user_input(var, var_output.borrow_mut());
            }
            print_variables(&var_output, config.shell);
        },
        None => println!("No variable set named {} found", env_set)
    }
}

fn get_user_input(e_var: &EnvVariables, o_var: &mut Vec<EnvVariables>) {
    let value = {
        let user_input = input::<String>()
        .msg(format!("{}[{}]: ",
            &e_var.name,
            &e_var.value.as_ref().unwrap_or(&"".to_string())
        ))
        .get();

        if user_input.is_empty() {
            None
        } else {
            Some(user_input)
        }
    };

    match value {
        Some(v) => {
            o_var.push( EnvVariables {
                name: e_var.name.clone(),
                value: Some(v),
            })
        },
        None => {
            if let Some(v) = e_var.value.clone() {
                o_var.push( EnvVariables {
                    name: e_var.name.clone(),
                    value: Some(v),
                })
            } else {
                o_var.push( EnvVariables {
                    name: e_var.name.clone(),
                    value: Some("".to_owned()),
                })
            }
        }
    }
}

fn print_variables (var_list: &Vec<EnvVariables>, shell: Shell) {
    match shell {
        Shell::Fish => {
            for var in var_list {
                println!(
                    "set {} {}",
                    var.name.to_ascii_uppercase(),
                    var.value.clone().unwrap());
            }
        },
        Shell::Posix => {
            for var in var_list {
                println!(
                    "{}={}",
                    var.name.to_uppercase(),
                    var.value.clone().unwrap()
                )
            }
        },
    }
}
