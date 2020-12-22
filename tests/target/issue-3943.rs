// Tests for original #3943 issue
use ::Foo;
use ::foo;
use ::foo::Bar1;
use ::foo::{Bar2, Baz2};
use ::{Bar3, Baz3};

use ::Foo;
use ::foo;
use ::foo::Bar;
use ::foo::{Bar, Baz};
use ::{Bar, Baz};

use ::Foo;
use ::foo;
use ::foo::Bar;
use ::foo::{Bar, Baz};
use ::{Bar, Baz};

// Additional tests for signle item `{}` handling
use super::auxvec;
use crate::detect::{cache, Feature};
use ::AAAA;
use ::BBBB;
use aaaa::BBBB;
use bbbbb::AAAA;

// Tests with comments and "as"
use a::{/* pre-comment */ item};
use a::{item /* post-comment */};
use a::{/* pre-comment */ item /* post-comment */};
