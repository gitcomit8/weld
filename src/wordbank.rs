use directories::ProjectDirs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::{fs, io};

const BUILTIN_WORDS: &str = include_str!("res/eff_words.txt");

pub struct WordBank {
    built_in: Vec<&'static str>,
    custom_path: PathBuf,
}

impl WordBank {
    pub fn new() -> Self {
        //Prepare built-in words
        let built_in: Vec<&str> = BUILTIN_WORDS
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect();

        //Determine path for custom words
        let proj_dirs =
            ProjectDirs::from("", "", "weld").expect("Could not determine home directory");
        let conf_dir = proj_dirs.config_dir();

        //Create dir if does not exist
        if !conf_dir.exists() {
            fs::create_dir_all(conf_dir).unwrap_or_else(|_| {});
        }

        let custom_path = conf_dir.join("custom_words.txt");

        Self {
            built_in,
            custom_path,
        }
    }

    //Get all words
    pub fn get_all_words(&self) -> Vec<String> {
        let mut all_words: Vec<String> = self.built_in.iter().map(|s| s.to_string()).collect();

        //Read custom file if exists
        if self.custom_path.exists() {
            if let Ok(content) = fs::read_to_string(&self.custom_path) {
                for line in content.lines() {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        all_words.push(trimmed.to_string());
                    }
                }
            }
        }
        all_words
    }

    //Add custom word to file
    pub fn add_custom_word(&self, word: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.custom_path)?;

        writeln!(file, "{}", word)?;
        Ok(())
    }
}
