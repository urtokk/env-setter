use std::io::Write;

use color_eyre::eyre::Result;

pub fn init_config(path: &str) -> Result<()> {
    let config = crate::configuration::Config {
        shell: crate::env_variables::Shell::Posix,
        sets: {
            let mut sets = std::collections::HashMap::new();
            let mut value_list = Vec::new();
            let test_val = crate::env_variables::EnvVariables {
                name: "TestKey".to_owned(),
                value: Some("TestValue".to_owned())
            };
            value_list.push(test_val);
            sets.insert("TestSet".to_owned(), value_list);
            sets

        },
    };

    let mut configfile = std::fs::File::create(path)?;
    let content = serde_yaml::to_string(&config)?;
    configfile.write(content.as_bytes())?;
    configfile.flush()?;
    Ok(())
}