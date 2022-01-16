use color_eyre::eyre::{eyre, Result};

use std::collections::HashMap;
use std::io::BufRead;

use crate::env_variables::EnvVariables;

pub(crate) fn get_target_set<'a, 'b>(
    config_set: &'a mut HashMap<String, Vec<EnvVariables>>,
    target: &'b str,
) -> Result<&'a mut Vec<EnvVariables>> {
    match config_set.get_mut(target) {
        Some(s) => Ok(s),
        None => return Err(eyre!("Could not find target set for {}", target)),
    }
}

pub(crate) fn get_input<T: BufRead>(prompt: &str, source: &mut T) -> Option<String> {
    print!("{}", prompt);
    let mut user_input = String::new();
    match source.read_line(&mut user_input) {
        Ok(_) => {
            if user_input.is_empty() {
                None
            } else {
                Some(user_input.trim().to_owned())
            }
        }
        Err(_) => None,
    }
}
