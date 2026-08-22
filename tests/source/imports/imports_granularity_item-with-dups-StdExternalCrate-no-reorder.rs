// rustfmt-imports_granularity: Item
// rustfmt-reorder_imports: false
// rustfmt-group_imports: StdExternalCrate

use crate::lexer;
use crate::lexer;
use crate::lexer::tokens::TokenData;
use crate::lexer::{tokens::TokenData};
use crate::lexer::self;
use crate::lexer;
use crate::lexer;
use crate::lexer::{self};
use crate::lexer::{self, tokens::TokenData};

use crate::{b::c, d::e, d::f};
#[cfg(unix)]
use crate::{b::c, d::e, d::f};
#[cfg(windows)]
use crate::{b::c, d::e, d::f};
// my comment
use crate::b;
