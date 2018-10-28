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
}

#[derive(Serialize)]
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
            edition: Some(String::from("Edition2018")),
        }
    }
    pub fn check(&mut self, choice: bool) {
        self.check = Some(choice);
    }
    pub fn emit(&mut self, choice: String) {
        self.emit_mode = Some(choice);
    }
    pub fn backup(&mut self, choice: bool) {
        self.make_backup = Some(choice);
    }
    pub fn color(&mut self, choice: String) {
        self.color = Some(choice);
    }
    pub fn file_lines(&mut self, choice: String) {
        self.file_lines = Some(choice);
    }
    pub fn skip_children(&mut self, choice: bool) {
        self.skip_children = Some(choice);
    }
    pub fn error_on_unformatted(&mut self, choice: bool) {
        self.error_on_unformatted = Some(choice);
    }
}

fn is_nightly() -> bool {
    option_env!("CFG_RELEASE_CHANNEL").map_or(false, |c| c == "nightly" || c == "dev")
}

fn make_interactive_options<'a>() -> Vec<InteractiveConfig<'a>> {
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

    if is_nightly() {
        inter_opts.push(InteractiveConfig::init(
            Configuration::FileLines,
            "What lines to you want to restrict formatting to ({\"file\":\"lib.rs\",\"range\":[7,13]})? ",
        ));
        inter_opts.push(InteractiveConfig::init(
            Configuration::ErrorOnUnformatted,
            "Do you want rustfmt to fail if comments or string litererals are beyond the wax-width? ",
        ));
        inter_opts.push(InteractiveConfig::init(
            Configuration::SkipChildren,
            "Do you want rustfmt to skip child modules? ",
        ));
    }

    inter_opts
}

pub fn interactive_config() -> Result<i32, failure::Error> {
    use self::Configuration::{
        Backup, Check, Color, ConfigPath, Emit, ErrorOnUnformatted, FileLines, SkipChildren,
    };
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let options = make_interactive_options();
    let handle = stdin.lock();
    let mut configuration_path = String::new();
    let mut fin_choices = ConfigChoice::new();

    let mut line = handle.lines();
    for mut option in options {
        print!("{}", option.init_text);
        stdout.flush()?;

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
                    }
                }
            }
            None => (),
        };
    }

    let mut file = if configuration_path != "" {
        File::create(format!("{}/rustfmt.toml", configuration_path))
            .expect("Could not create configuration file")
    } else {
        File::create("rustfmt.toml").expect("Could not create configuration file")
    };

    let toml = toml::to_string(&fin_choices).expect("Unable to serialize configuration");
    file.write(toml.as_bytes())
        .expect("Not able to write to file");

    Ok(0)
}
