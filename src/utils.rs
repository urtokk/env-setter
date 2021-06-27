use color_eyre::eyre::{
    Result,
    eyre
};

use std::collections::HashMap;

use crate::env_variables::EnvVariables;

pub fn get_target_set<'a, 'b>(config_set: &'a mut HashMap<String, Vec<EnvVariables>>, target: &'b str) -> Result<&'a mut Vec<EnvVariables>> {
    match config_set.get_mut(target) {
        Some(s) => Ok(s),
        None => return Err(eyre!("Could not find target set for {}", target)),
    }
}