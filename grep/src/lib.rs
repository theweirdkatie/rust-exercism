use anyhow::Error;
use std::{fs, str::from_utf8};

#[derive(Debug)]
pub struct Flags {
    line_nums: bool,
    file_names: bool,
    match_lines: bool,
    case_ins: bool,
    invert: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Self {
            line_nums: flags.contains(&"-n"),
            file_names: flags.contains(&"-l"),
            match_lines: flags.contains(&"-x"),
            case_ins: flags.contains(&"-i"),
            invert: flags.contains(&"-v"),
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut result: Vec<String> = vec![];
    let multiple_files = files.len() > 1;
    for file in files {
        let contents = fs::read(file)?;
        let mut lines = from_utf8(&contents)?
            .lines()
            .enumerate()
            .filter(
                |(_, line)| flags.invert ^ match (flags.match_lines, flags.case_ins) {
                    (true, false) => line == &pattern,
                    (true, true) => line.to_lowercase() == pattern.to_lowercase(),
                    (false, false) => line.contains(&pattern),
                    (false, true) => line.to_lowercase().contains(&pattern.to_lowercase()),
                }
            )
            .map(|(i, line)| {
                format!("{}{}{line}", {
                    if multiple_files {
                        format!("{}:", file.to_string())
                    } else { "".to_string() }
                },{
                    if flags.line_nums {
                        format!("{}:", i + 1)
                    } else { "".to_string() }
                })
            })
            .collect::<Vec<String>>();

        if flags.file_names && lines.len() > 0 {
            result.push(file.to_string());
        } else {
            result.append(&mut lines);
        }
    }
    Ok(result)
}
