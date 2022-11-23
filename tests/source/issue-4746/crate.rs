// rustfmt-imports_granularity: Crate

// Long imports with reordering - from original issue
use foo::bar;
use foo::{foo, baz};
use abaadfsasdfdsfdfas::aasdffjsioejr::abc::sdsdf::sdfsdfsdf::sdfsdfdsf::{
    asdfasdefasdasdfsdfdfasdf::asdfasdasedfafasdfasdf,
};

use foo::bar;
use foo::{foo, baz};
use abaadfsasdfdsfdfas::aasdffjsioejr::abc::sdsdf::sdfsdfsdf::sdfsdfds::{
    asdfasdefasdasdfsdfdfasdf::asdfasdasedfafasdfasdf,
};

// Long imports with different sizes - based on original issue
use z123::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::asdf;
use a;

use z123::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::a;
use a;

use z123::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567;
use a;

use z123::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z12345678::z1234567;
use a;

use z123::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567890123456::z1234567;
use a;

use z123::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z12345678901234567::z1234567;
use a;

use z123::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::asdf;
use a;

// Very long (two lines) imports
use z123::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567;
use a;

use z123::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::{foo, baz};
use a;

use z123::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::foo;
use z123::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::baz;
use a;

use z123::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::{
foo::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567,
baz::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567};
use a;

use z123::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::{
foo::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567,
baz::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567};
use a;

use z123::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::{
z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::foo,
z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::baz};
use a;

use z123::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::foo::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567;
use z123::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::baz::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567;
use a;

use z123::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::foo;
use z123::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::z1234567::baz;
use a;
