use color_eyre::eyre::Result;
use read_input::prelude::*;

use serde_derive::{
    Serialize,
    Deserialize,
};

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
    pub fn ask_user_input(&mut self) -> &mut Self {
        let user_input = input::<String>()
        .msg(format!("#{}[{}]: ",
            self.name,
            self.value.as_ref().unwrap_or(&"".to_string())
        ))
        .get();

        if !user_input.is_empty() {
            self.value = Some(user_input)
        }

        self
    }

    pub fn print_variables<T> (&self, shell: &Shell, mut destination: T) -> Result<()>
    where T: std::io::Write
    {
        match shell {
            Shell::Fish => {
                destination.write(format!(
                    "set {} {}\n",
                    self.name.to_ascii_uppercase(),
                    self.value.clone().unwrap_or(String::new())
                ).as_bytes())?;
            },
            Shell::Posix => {
                destination.write(format!(
                    "{}={}\n",
                    self.name.to_uppercase(),
                    self.value.clone().unwrap()
                ).as_bytes())?;
            },
        }
        destination.flush()?;
        Ok(())
    }
}
