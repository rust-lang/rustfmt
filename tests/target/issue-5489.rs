// rustfmt-format_macro_matchers: true

// Original issue #5489 case - macro metavariable ($name) as attribute value.
use clap::Parser;

macro_rules! generate_the_struct {
    ($a:literal) => {
        #[derive(Parser)]
        pub struct Struct {
            #[clap(
                long = "--alpha-beta-gamma",
                env = "ALPHA_BETA_GAMMA",
                default_value = $a,
            )]
            alpha_beta_gamma: usize,
        }
    };
}

fn main() {
    generate_the_struct!("77777");
    let s = Struct {
        alpha_beta_gamma: 66666,
    };
    println!("{}", s.alpha_beta_gamma);
}

//
// Other case with macro metavariables ($name) as attribute value.
macro_rules! generate_the_struct {
    ($a:literal) => {
        #[derive(Parser)]
        pub struct Struct {
            #[clap(
                long = "--alpha-beta-gamma",
                env = "ALPHA_BETA_GAMMA",
                default_value = $a,
            )]
            alpha_beta_gamma: usize,
        }
    };
}

macro_rules! generate_the_struct {
    ($bbbbb:literal) => {
        #[derive(Parser)]
        pub struct Struct {
            #[clap(default_value($bbbbb))]
            aaaaa: str,
        }
    };
}

macro_rules! generate_the_struct {
    ($bbbbb:literal, $ccccc:literal) => {
        #[clap(default_value = $bbbbb, other_value = $ccccc,)]
        const aaaaa: str;
    };
}

macro_rules! generate_the_struct {
    ($bbbbb:literal, $ccccc:literal) => {
        #[clap(default_value = $bbbbb, other_value = $ccccc, lit = "Lit",)]
        const aaaaa: str;
    };
}

//
// Cases with COSNT as attribute value.
const CONSTVAR: usize = 8;
#[clap(default_value = CONSTVAR)]
const aaaaa: str;

macro_rules! generate_the_struct {
    ($a:literal) => {
        #[derive(Parser)]
        pub struct Struct {
            #[clap(long = "--alpha-beta-gamma", env = CONSTVAR,)]
            alpha_beta_gamma: usize,
        }
    };
}

macro_rules! generate_the_struct {
    ($a:literal) => {
        #[derive(Parser)]
        pub struct Struct {
            #[clap(long = "--alpha-beta-gamma", env = CONSTVAR, default_value = $a,)]
            alpha_beta_gamma: usize,
        }
    };
}

//
// Variations of #2470 from `macro_rules.rs`.
macro foo($type_name:ident, $docs:expr) {
    #[doc = "Lit"]
    pub struct $type_name;
}

macro foo($type_name:ident, $docs:expr) {
    #[doc = $docs]
    pub struct $type_name;
}

macro foo($type_name:ident, $docs:expr) {
    #[$docs]
    pub struct $type_name;
}

//
// Cases with no macro metavariable ($name) or CONST as attribute value
macro_rules! generate_the_struct {
    ($a:literal) => {
        #[derive(Parser)]
        pub struct Struct {
            #[clap(
                long = "--alpha-beta-gamma",
                env = "ALPHA_BETA_GAMMA",
                default_value = "Lit"
            )]
            alpha_beta_gamma: usize,
        }
    };
}

#[clap(default_value = "Lit")]
const aaaaa: str;
