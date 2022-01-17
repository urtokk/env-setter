use color_eyre::eyre::Result;

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

    #[test]
    fn test_fish_env_variables_print() {
        let env_variables = EnvVariables {
            name: "TEST".to_string(),
            value: Some("test".to_string()),
        };

        let mut destination = Vec::new();
        env_variables.print_variables(&Shell::Fish, &mut destination).unwrap();
        assert_eq!(
            String::from_utf8_lossy(&destination),
            "set TEST test\n"
        );
    }

    #[test]
    fn test_posix_env_variables_print() {
        let env_variables = EnvVariables {
            name: "TEST".to_string(),
            value: Some("test".to_string()),
        };

        let mut destination = Vec::new();
        env_variables.print_variables(&Shell::Posix, &mut destination).unwrap();
        assert_eq!(
            String::from_utf8_lossy(&destination),
            "TEST=test\n"
        );
    }
}
