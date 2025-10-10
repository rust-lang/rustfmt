use std::{fmt::Display, io::Write as _, path::PathBuf};

use crate::Config;

use super::{EditorConfig, maybe_unset::UnsetBehaviour};

#[derive(Clone, Debug)]
pub enum EditorConfigSerializationTarget {
    Stdout,
    Directory(PathBuf),
    File(PathBuf),
}

impl From<Option<PathBuf>> for EditorConfigSerializationTarget {
    fn from(value: Option<PathBuf>) -> Self {
        match value {
            Some(path) => match path.is_dir() {
                true => Self::Directory(path),
                false => Self::File(path),
            },
            None => Self::Stdout,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EditorConfigSerializationError {
    #[error("{0} is not a Directory")]
    NotADirectory(PathBuf),
    #[error("{0}")]
    IO(#[from] std::io::Error),
}

fn write_to_dir<S: Display>(
    dir: PathBuf,
    s: &S,
) -> std::result::Result<(), EditorConfigSerializationError> {
    match dir.is_dir() {
        true => {
            let mut config = dir;
            config.push(".editorconfig");
            append_to_file(&config, s).map_err(|err| err.into())
        }
        false => Err(EditorConfigSerializationError::NotADirectory(dir)),
    }
}

fn append_to_file<S: Display>(p: &PathBuf, s: &S) -> std::io::Result<()> {
    write!(
        std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(p)?,
        "{}",
        s,
    )
}

pub struct EditorConfigSerializer {
    config: EditorConfig,
    unset_behaviour: UnsetBehaviour,
}

impl EditorConfigSerializer {
    pub fn new(config: EditorConfig, unset_behaviour: UnsetBehaviour) -> Self {
        Self {
            config,
            unset_behaviour,
        }
    }
    pub fn unset_omit(&mut self) {
        self.unset_behaviour = UnsetBehaviour::Omit;
    }
    pub fn unset_emit(&mut self) {
        self.unset_behaviour = UnsetBehaviour::Emit;
    }

    pub fn write_to_target(
        self,
        target: &mut EditorConfigSerializationTarget,
    ) -> std::result::Result<(), EditorConfigSerializationError> {
        let s = self.to_string();
        match target {
            EditorConfigSerializationTarget::Stdout => {
                write!(&mut std::io::stdout().lock(), "{s}").map_err(|err| err.into())
            }
            EditorConfigSerializationTarget::Directory(path_buf) => {
                write_to_dir(path_buf.clone(), &s)
            }
            EditorConfigSerializationTarget::File(file) => {
                append_to_file(file, &s).map_err(|err| err.into())
            }
        }
    }
}

impl From<EditorConfig> for EditorConfigSerializer {
    fn from(value: EditorConfig) -> Self {
        Self::new(value, Default::default())
    }
}

impl<'a> From<&'a Config> for EditorConfigSerializer {
    fn from(value: &'a Config) -> Self {
        EditorConfig::from(value).into()
    }
}

impl From<Config> for EditorConfigSerializer {
    fn from(value: Config) -> Self {
        EditorConfig::from(value).into()
    }
}

impl Display for EditorConfigSerializer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.config.write_to(f, self.unset_behaviour)
    }
}
