// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This module contains types and functions to support interactive configuration.

use super::file_lines::FileLines;
use std::fs::File;
use std::io::{self, BufRead, Write};

pub struct InteractiveConfig<'a> {
    pub config: Configuration,
    pub init_text: &'a str,
    pub choice: Option<bool>,
    pub choice_text: String,
}

impl<'a> InteractiveConfig<'a> {
    pub fn update_choice(&mut self, choice: bool, text: String) {
        self.choice = Some(choice);
        self.choice_text = text;
    }
    pub fn init(conf: Configuration, text: &str) -> InteractiveConfig {
        InteractiveConfig {
            config: conf,
            init_text: text,
            choice: None,
            choice_text: String::new(),
        }
    }
}

/// Rustfmt configuration options.
pub enum Configuration {
    Check,
    /// Options for emitting files [files|stdout|coverage|checkstyle]
    Emit,
    Backup,
    ConfigPath,
    /// Colored output [always|never|auto]
    Color,
    FileLines,
    SkipChildren,
    ErrorOnUnformatted,
    /// Rustfmt edition [2015|2018]
    Edition,
}

#[derive(Serialize, Debug)]
pub struct ConfigChoice {
    check: Option<bool>,
    emit_mode: Option<String>,
    make_backup: Option<bool>,
    color: Option<String>,
    file_lines: Option<String>,
    skip_children: Option<bool>,
    error_on_unformatted: Option<bool>,
    edition: Option<String>,
}

impl ConfigChoice {
    pub fn new() -> ConfigChoice {
        ConfigChoice {
            check: None,
            emit_mode: None,
            make_backup: None,
            color: None,
            file_lines: None,
            skip_children: None,
            error_on_unformatted: None,
            edition: None,
        }
    }
    pub fn check(&mut self, choice: bool) {
        self.check = Some(choice);
    }
    pub fn emit(&mut self, choice: String) {
        match choice.as_ref() {
            "files" | "stdout" | "coverage" | "checkstype" => self.emit_mode = Some(choice),
            _ => println!("Invalid option for emitting files \"{}\":", choice),
        }
    }
    pub fn backup(&mut self, choice: bool) {
        self.make_backup = Some(choice);
    }
    pub fn color(&mut self, choice: String) {
        match choice.as_ref() {
            "always" | "never" | "auto" => self.color = Some(choice),
            _ => println!("Invalid option for rustfmt color \"{}\":", choice),
        }
    }
    pub fn file_lines(&mut self, choice: String) {
        let json: Result<FileLines, serde_json::Error> = serde_json::from_str(choice.as_str());

        match json {
            Ok(_) => self.file_lines = Some(choice),
            Err(_) => println!("Invalid option for file lines selection \"{}\":", choice),
        }
    }
    pub fn skip_children(&mut self, choice: bool) {
        self.skip_children = Some(choice);
    }
    pub fn error_on_unformatted(&mut self, choice: bool) {
        self.error_on_unformatted = Some(choice);
    }
    pub fn edition(&mut self, choice: String) {
        match choice.as_ref() {
            "2015" | "2018" => self.edition = Some(choice),
            "" | "\n" => self.edition = Some(String::from("2018")),
            _ => println!("Invalid rustfmt edition \"{}\":", choice),
        }
    }
}

fn is_nightly() -> bool {
    option_env!("CFG_RELEASE_CHANNEL").map_or(false, |c| c == "nightly" || c == "dev")
}

/// Generates the options for interactive mode.
fn make_interactive_options<'a>(nightly: bool) -> Vec<InteractiveConfig<'a>> {
    let mut inter_opts: Vec<InteractiveConfig> = Vec::new();

    inter_opts.push(InteractiveConfig::init(
        Configuration::Check,
        "Do you want rustfmt to only check your files? ",
    ));
    inter_opts.push(InteractiveConfig::init(
        Configuration::Emit,
        "Do you want to emit any data (files, stdout)? ",
    ));
    inter_opts.push(InteractiveConfig::init(
        Configuration::Backup,
        "Do you want to backup any modified files? ",
    ));
    inter_opts.push(InteractiveConfig::init(
        Configuration::ConfigPath,
        "Where would you like your rustfmt configuration saved? ",
    ));
    inter_opts.push(InteractiveConfig::init(
        Configuration::Color,
        "What coloring mode would you like to use (always, never, auto)? ",
    ));
    inter_opts.push(InteractiveConfig::init(
        Configuration::Edition,
        "Which rustfmt edition do you want to use (2015, 2018)? ",
    ));

    if nightly {
        inter_opts.push(InteractiveConfig::init(
            Configuration::FileLines,
            "What lines to you want to restrict formatting to? ",
        ));
        inter_opts.push(InteractiveConfig::init(
            Configuration::ErrorOnUnformatted,
            "Do you want rustfmt to fail if comments are beyond the wax-width? ",
        ));
        inter_opts.push(InteractiveConfig::init(
            Configuration::SkipChildren,
            "Do you want rustfmt to skip child modules? ",
        ));
    }

    inter_opts
}

/// Prompts the user for the given configuration options then returns the choices.
fn prompt_for_config<R, W>(
    reader: R,
    mut writer: W,
    options: Vec<InteractiveConfig>,
) -> (ConfigChoice, String)
where
    R: BufRead,
    W: Write,
{
    use self::Configuration::{
        Backup, Check, Color, ConfigPath, Edition, Emit, ErrorOnUnformatted, FileLines,
        SkipChildren,
    };

    let mut configuration_path = String::new();
    let mut fin_choices = ConfigChoice::new();

    let mut line = reader.lines();
    for mut option in options {
        write!(&mut writer, "{}", option.init_text);

        writer.flush().expect("Unable to flush stdout");

        let next_line = line.next();
        let user_choice = next_line.unwrap().unwrap();

        match user_choice.to_lowercase().as_ref() {
            "" | "no" | "n" | " " | "\n" => option.update_choice(false, String::new()),
            "yes" | "y" => option.update_choice(true, String::new()),
            _ => option.update_choice(true, user_choice),
        };

        match option.choice {
            Some(ch) => {
                if ch {
                    match option.config {
                        Backup => fin_choices.backup(true),
                        Check => fin_choices.check(true),
                        Color => fin_choices.color(option.choice_text),
                        ConfigPath => configuration_path = option.choice_text,
                        ErrorOnUnformatted => fin_choices.error_on_unformatted(true),
                        Emit => fin_choices.emit(option.choice_text),
                        FileLines => fin_choices.file_lines(option.choice_text),
                        SkipChildren => fin_choices.skip_children(true),
                        Edition => fin_choices.edition(option.choice_text),
                    }
                }
            }
            None => (),
        };
    }

    (fin_choices, configuration_path)
}

/// Main entry point for interactive mode.
pub fn interactive_config() -> Result<i32, failure::Error> {
    let stdin = io::stdin();
    let handle = &mut stdin.lock() as &mut BufRead;

    let options = make_interactive_options(is_nightly());
    let (choices, config_path) = prompt_for_config(handle, io::stdout(), options);

    println!("\nGenerating rustfmt.toml ...");
    let mut file = if config_path != "" {
        File::create(format!("{}/rustfmt.toml", config_path))
            .expect("Could not create configuration file")
    } else {
        File::create("rustfmt.toml").expect("Could not create configuration file")
    };

    let toml = toml::to_string(&choices).expect("Unable to serialize configuration");
    file.write(toml.as_bytes())
        .expect("Not able to write to file");

    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_normal_config() {
        let options = make_interactive_options(false);

        // format: check \n emit \n make_backup \n config_path \n color
        let input = b"no\nfiles\nyes\n./\nauto\n2018";
        let mut _output = Vec::new();
        let (choices, config_path) = prompt_for_config(&input[..], &mut _output, options);
        println!("Choices: {:?}", choices);

        let config = ConfigChoice {
            check: None,
            emit_mode: Some(String::from("files")),
            make_backup: Some(true),
            color: Some(String::from("auto")),
            file_lines: None,
            skip_children: None,
            error_on_unformatted: None,
            edition: Some(String::from("2018")),
        };

        assert_eq!(config_path, "./");
        assert_eq!(choices.check, config.check);
        assert_eq!(choices.emit_mode, config.emit_mode);
        assert_eq!(choices.make_backup, config.make_backup);
        assert_eq!(choices.color, config.color);
    }

    #[test]
    fn test_nightly_config() {
        let options = make_interactive_options(true);

        // format: check, emit, make_backup, config_path, color, edition,
        // file_lines, skip_children, error_on_unformatted, edition
        let input = b"yes\nconfig\nyes\n./\nauto\n2015\nno\nno\nyes";
        let mut _output = Vec::new();
        let (choices, config_path) = prompt_for_config(&input[..], &mut _output, options);
        println!("Choices: {:?}", choices);

        let config = ConfigChoice {
            check: Some(true),
            emit_mode: None,
            make_backup: Some(true),
            color: Some(String::from("auto")),
            file_lines: None,
            skip_children: Some(true),
            error_on_unformatted: None,
            edition: Some(String::from("2015")),
        };

        assert_eq!(config_path, "./");
        assert_eq!(choices.check, config.check);
        assert_eq!(choices.emit_mode, config.emit_mode);
        assert_eq!(choices.make_backup, config.make_backup);
        assert_eq!(choices.color, config.color);
        assert_eq!(choices.file_lines, config.file_lines);
        assert_eq!(choices.skip_children, config.skip_children);
        assert_eq!(choices.error_on_unformatted, config.error_on_unformatted);
        assert_eq!(choices.edition, config.edition);
    }
}
