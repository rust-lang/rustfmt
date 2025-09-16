// Test that the default import granularity merges imports at crate level

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs::File,
    io::Read,
    path::Path,
};

pub use serde::{Deserialize, Serialize};

use clap::{Args, Parser};