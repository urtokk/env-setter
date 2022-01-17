use color_eyre::eyre::Result;
use read_input::prelude::*;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Shell {
    Fish,
    Posix,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvVariables {
    pub name: String,
    pub value: Option<String>,
}

impl EnvVariables {
    pub fn print_variables<T>(&self, shell: &Shell, mut destination: T) -> Result<()>
    where
        T: std::io::Write,
    {
        match shell {
            Shell::Fish => {
                destination.write_all(
                    format!(
                        "set {} {}\n",
                        self.name.to_ascii_uppercase(),
                        self.value.clone().unwrap_or_default(),
                    )
                    .as_bytes(),
                )?;
            }
            Shell::Posix => {
                destination.write_all(
                    format!(
                        "{}={}\n",
                        self.name.to_uppercase(),
                        self.value.clone().unwrap()
                    )
                    .as_bytes(),
                )?;
            }
        }
        destination.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration;
    use crate::utils;
    use std::io::BufReader;

    #[test]
    fn test_fish_envVariables_load() {
        let mut prepared_input = BufReader::new("test\n".as_bytes());
        let mut config = configuration::get_config("resources/test.yaml");
        let mut target_set = {
            match crate::utils::get_target_set(&mut config.sets, "another-set") {
                Ok(s) => s,
                Err(e) => panic!("{}", e),
            }
        };

        assert_eq!(target_set.len(), 1);
        assert!(target_set[0].name == "ANOTHERTEST");
        assert!(target_set[0].value.is_none());
    }
}
