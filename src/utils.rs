use color_eyre::eyre::{eyre, Result};

use std::collections::HashMap;
use std::io::BufRead;
use std::io::Write;

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

pub(crate) fn get_input<T: BufRead, W: Write>(
    prompt: &str,
    source: &mut T,
    destination: &mut W,
) -> Option<String> {
    destination
        .write_all(prompt.as_bytes())
        .map_err(|e| {
            eprintln!("Could not write to destination: {}", e);
            std::process::exit(5)
        })
        .ok();
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_get_input() {
        let mut prepared_input = BufReader::new("test\n".as_bytes());
        let mut prepared_empty_input = BufReader::new("\n".as_bytes());
        let mut catched_output = Vec::new();
        assert_eq!(
            get_input("test", &mut prepared_input, &mut catched_output),
            Some("test".to_owned())
        );
        catched_output.clear();
        assert_eq!(
            get_input("test", &mut prepared_empty_input, &mut catched_output),
            Some("".to_owned())
        );
    }

    #[test]
    fn test_fish_get_target_set() {
        let mut config = crate::configuration::get_config("resources/test.yaml");
        let target_set = get_target_set(&mut config.sets, "test-set");
        assert!(target_set.is_ok());
        assert_eq!(target_set.as_ref().unwrap().len(), 2);
        assert_eq!(
            target_set.as_ref().unwrap().get(0).unwrap().name,
            "TESTKEY".to_owned()
        );
    }

    #[test]
    fn test_bash_get_target_set() {
        let mut config = crate::configuration::get_config("resources/test_posix.yaml");
        let target_set = get_target_set(&mut config.sets, "test-set");
        assert!(target_set.is_ok());
        assert_eq!(target_set.as_ref().unwrap().len(), 2);
        assert_eq!(
            target_set.as_ref().unwrap().get(0).unwrap().name,
            "TESTKEY".to_owned()
        );
    }
}
