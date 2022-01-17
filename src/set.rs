use color_eyre::eyre::Result;

use crate::env_variables::EnvVariables;
use crate::env_variables::Shell;
use crate::utils;

pub fn set<R, W>(
    var_set: &mut Vec<EnvVariables>,
    shell: Shell,
    source: &mut R,
    target: &mut W,
) -> Result<()>
where
    R: std::io::BufRead,
    W: std::io::Write,
{
    for item in var_set.iter_mut() {
        if let Some(s) = utils::get_input(
            format!(
                "#{}[{}]: ",
                item.name,
                item.value.as_ref().unwrap_or(&"".to_owned())
            )
            .as_str(),
            source,
            target,
        ) {
            if !s.is_empty() {
                item.value = Some(s);
            }
        }
    }

    for item in var_set {
        item.print_variables(&shell, target.by_ref())
            .map_err(|e| {
                eprintln!("Could not print variables: {}", e);
                std::process::exit(5)
            })
            .ok();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_set_fish() {
        let mut var_set = vec![
            EnvVariables {
                name: "TEST_VAR".to_owned(),
                value: Some("changeme".to_owned()),
            },
            EnvVariables {
                name: "TEST_VAR2".to_owned(),
                value: Some("test2".to_owned()),
            },
        ];

        let mut prepared_input = BufReader::new("test\n\n".as_bytes());
        let prepared_output = "set TEST_VAR test\nset TEST_VAR2 test2\n".to_owned();
        let mut catched_output = Vec::new();

        set(
            &mut var_set,
            Shell::Fish,
            &mut prepared_input,
            &mut catched_output,
        )
        .unwrap();

        assert_eq!(
            String::from_utf8_lossy(&catched_output)
                .matches(prepared_output.as_str())
                .into_iter()
                .count(),
            1
        );
    }

    #[test]
    fn test_set_posix() {
        let mut var_set = vec![
            EnvVariables {
                name: "TEST_VAR".to_owned(),
                value: Some("changeme".to_owned()),
            },
            EnvVariables {
                name: "TEST_VAR2".to_owned(),
                value: Some("test2".to_owned()),
            },
        ];

        let mut prepared_input = BufReader::new("test\n\n".as_bytes());
        let prepared_output = "TEST_VAR=test\nTEST_VAR2=test2\n".to_owned();
        let mut catched_output = Vec::new();

        set(
            &mut var_set,
            Shell::Posix,
            &mut prepared_input,
            &mut catched_output,
        )
        .unwrap();

        assert_eq!(
            String::from_utf8_lossy(&catched_output)
                .matches(prepared_output.as_str())
                .into_iter()
                .count(),
            1
        );
    }
}
