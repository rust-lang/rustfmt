// Test that the default import granularity merges imports at crate level

use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub use serde::Serialize;
pub use serde::Deserialize;

use clap::Args;
use clap::Parser;