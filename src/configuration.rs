use crate::env_variables::{EnvVariables, Shell};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct ConfigFile {
    shell: Option<Shell>,
    sets: HashMap<String, Vec<EnvVariables>>,
}

#[derive(Debug, Serialize)]
pub struct Config {
    pub shell: Shell,
    pub sets: HashMap<String, Vec<EnvVariables>>,
}

pub fn get_config(path: &str) -> Config {
    let config = {
        let mut configger = config::Config::default();
        configger.merge(config::File::with_name(path)).unwrap();
        let read_config = configger.try_into::<ConfigFile>().unwrap();

        Config {
            shell: read_config.shell.unwrap_or(Shell::Posix),
            sets: read_config.sets,
        }
    };

    config
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_posix() {
        let config = get_config("resources/test_posix.yaml");
        matches!(config.shell, Shell::Posix);
        assert_eq!(config.sets.len(), 2);
        assert_eq!(config.sets["test-set"].len(), 2);
    }

    #[test]
    fn test_config_fish() {
        let config = get_config("resources/test.yaml");
        matches!(config.shell, Shell::Fish);
        assert_eq!(config.sets.len(), 2);
        assert_eq!(config.sets["test-set"].len(), 2);
    }
}
