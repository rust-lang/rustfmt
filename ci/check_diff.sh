#!/bin/bash

set -e

# https://github.com/rust-lang/rustfmt/issues/5675
export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib:$LD_LIBRARY_PATH

function print_usage() {
    echo "usage check_diff REMOTE_REPO FEATURE_BRANCH [COMMIT_HASH] [OPTIONAL_RUSTFMT_CONFIGS]"
}

if [ $# -le 1 ]; then
    print_usage
    exit 1
fi

REMOTE_REPO=$1
FEATURE_BRANCH=$2
OPTIONAL_COMMIT_HASH=$3
OPTIONAL_RUSTFMT_CONFIGS=$4

# OUTPUT array used to collect all the status of running diffs on various repos
STATUSES=()

# Clone a git repository and cd into it.
#
# Parameters:
# $1: git clone url
# $2: directory where the repo should be cloned
function clone_repo() {
    GIT_TERMINAL_PROMPT=0 git clone --quiet $1 --depth 1 $2 && cd $2
}

# Initialize Git submoduels for the repo.
#
# Parameters
# $1: list of directories to initialize
function init_submodules() {
    git submodule update --init $1
}

# Run rusfmt with the --check flag to see if a diff is produced.
#
# Parameters:
# $1: Path to a rustfmt binary
# $2: Output file path for the diff
# $3: Any additional configuration options to pass to rustfmt
#
# Globlas:
# $OPTIONAL_RUSTFMT_CONFIGS: Optional configs passed to the script from $4
function create_diff() {
    local config;
    if [ -z "$3" ]; then
        config="--config=error_on_line_overflow=false,error_on_unformatted=false"
    else
        config="--config=error_on_line_overflow=false,error_on_unformatted=false,$OPTIONAL_RUSTFMT_CONFIGS"
    fi

    for i in `find . | grep "\.rs$"`
    do
        $1 --unstable-features --skip-children --check --color=always $config $i >> $2 2>/dev/null
    done
}

# Run the master rustfmt binary and the feature branch binary in the current directory and compare the diffs
#
# Parameters
# $1: Name of the repository (used for logging)
#
# Globlas:
# $RUSFMT_BIN: Path to the rustfmt master binary. Created when running `compile_rustfmt`
# $FEATURE_BIN: Path to the rustfmt feature binary. Created when running `compile_rustfmt`
# $OPTIONAL_RUSTFMT_CONFIGS: Optional configs passed to the script from $4
# $OUTPUT_DIR: Path to an output directory for storing the diff files. Set in `main`
function check_diff() {
    echo "running rustfmt (master) on $1"
    create_diff $RUSFMT_BIN rustfmt_diff.txt

    echo "running rustfmt (feature) on $1"
    create_diff $FEATURE_BIN feature_diff.txt $OPTIONAL_RUSTFMT_CONFIGS

    echo "checking diff"
    local diff;
    # we don't add color to the diff since we added color when running rustfmt --check.
    # tail -n + 6 removes the git diff header info
    # cut -c 2- removes the leading diff characters("+","-"," ") from running git diff.
    # Again, the diff output we care about was already added when we ran rustfmt --check
    diff=$(
        git --no-pager diff --color=never \
        --unified=0 --no-index rustfmt_diff.txt feature_diff.txt 2>&1 | tail -n +6 | cut -c 2-
    )

    # COPY diffs into output dir
    mkdir $OUTPUT_DIR/$1
    echo "Copying diffs to $OUTPUT_DIR/$1"
    cp rustfmt_diff.txt $OUTPUT_DIR/$1/rustfmt_diff.txt
    cp feature_diff.txt $OUTPUT_DIR/$1/feature_diff.txt

    if [ -z "$diff" ]; then
        echo "no diff detected between rustfmt and the feture branch"
        return 0
    else
        echo "Copying diffs between rustfmt and feature branch to $OUTPUT_DIR/$1/diff.txt"
        echo "$diff" >> $OUTPUT_DIR/$1/diff.txt
        echo "$diff"
        return 1
    fi
}

# Compiles and produces two rustfmt binaries.
# One for the current master, and another for the feature branch
#
# Parameters:
# $1: Directory where rustfmt will be cloned
#
# Globlas:
# $REMOTE_REPO: Clone URL to the rustfmt fork that we want to test
# $FEATURE_BRANCH: Name of the feature branch
# $OPTIONAL_COMMIT_HASH: Optional commit hash that will be checked out if provided
function compile_rustfmt() {
    RUSTFMT_REPO="https://github.com/rust-lang/rustfmt.git"
    clone_repo $RUSTFMT_REPO $1
    git remote add feature $REMOTE_REPO
    git fetch feature $FEATURE_BRANCH

    CARGO_VERSON=$(cargo --version)
    echo -e "\ncompiling with $CARGO_VERSON\n"

    echo "Building rustfmt from src"
    cargo build -q --release --bin rustfmt && cp target/release/rustfmt $1/rustfmt

    if [ -z "$OPTIONAL_COMMIT_HASH" ] || [ "$FEATURE_BRANCH" = "$OPTIONAL_COMMIT_HASH" ]; then
        git switch $FEATURE_BRANCH
    else
        git switch $OPTIONAL_COMMIT_HASH --detach
    fi

    echo "Building feature rustfmt from src"
    cargo build -q --release --bin rustfmt && cp target/release/rustfmt $1/feature_rustfmt

    RUSFMT_BIN=$1/rustfmt
    RUSTFMT_VERSION=$($RUSFMT_BIN --version)
    echo -e "\nRUSFMT_BIN $RUSTFMT_VERSION\n"

    FEATURE_BIN=$1/feature_rustfmt
    FEATURE_VERSION=$($FEATURE_BIN --version)
    echo -e "FEATURE_BIN $FEATURE_VERSION\n"
}

# Check the diff for running rustfmt and the feature branch on all the .rs files in the repo.
#
# Parameters
# $1: Clone URL for the repo
# $2: Name of the repo (mostly used for logging)
# $3: Path to any submodules that should be initialized
function check_repo() {
    WORKDIR=$(pwd)
    REPO_URL=$1
    REPO_NAME=$2
    SUBMODULES=$3

    local tmp_dir;
    tmp_dir=$(mktemp -d -t $REPO_NAME-XXXXXXXX)
    clone_repo $REPO_URL $tmp_dir

    if [ ! -z "$SUBMODULES" ]; then
        init_submodules $SUBMODULES
    fi


    # rustfmt --check returns 1 if a diff was found
    # Also check_diff returns 1 if there was a diff between master rustfmt and the feature branch
    # so we want to ignore the exit status check
    set +e
    check_diff $REPO_NAME
    # append the status of running `check_diff` to the STATUSES array
    STATUSES+=($?)
    set -e

    echo -e "removing tmp_dir $tmp_dir\n\n"
    rm -rf $tmp_dir
    cd $WORKDIR
}

function write_readme() {
    rustfmt_diff=\`rustfmt_diff.txt\`
    feature_diff=\`feature_diff.txt\`
    diff_file=\`diff.txt\`
    diff_files=\`*_diff.txt\`

    if [ -n "$OPTIONAL_RUSTFMT_CONFIGS" ]; then
        OPTIONAL_CONFIG_DETAILS="* diff check optional configs: \`$OPTIONAL_RUSTFMT_CONFIGS\`"
    fi

    cat > README.md << EOL
# Diff Check

## Summary

The Diff Check Job is used to validate rustfmts backwards compatability guarantees
by running the latest rustfmt from [rust-lang/rustfmt](https://github.com/rust-lang/rustfmt) and
comparing the formatting results against a fork or feature branch of rustfmt --
often before deciding to merge those changes into rustfmt via a pull request.

**cargo details**
* version: \`$CARGO_VERSON\`

**rustfmt details**
* version: \`$RUSTFMT_VERSION\`

**fork details**
* repo url: $REMOTE_REPO
* feature branch: \`$FEATURE_BRANCH\`
* version: \`$FEATURE_VERSION\`
$OPTIONAL_CONFIG_DETAILS

## How to interpret results

Diffs created by running the rustfmt binary are reported in $rustfmt_diff, and
diffs created by running the forked rustfmt binary are stored in $feature_diff.
The presence of $rustfmt_diff and $feature_diff are not indicative of any errors.
Some of the real world projects that rustfmt is tested against may not use rustfmt at all.
All the $diff_files files show is that using rustfmt on a given project would change some formatting.

If a $diff_file file is present for a given project then that indicates a failure to
uphold rustfmts backwards compatability guarantees. Given the same input both binaries produced different outputs.
The $diff_file shows the difference in formatting output between both binaries.

## How to inspect diff-check results

First unzip the the diff-check archive

\`\`\`
unzip diff-check.zip -d diff-check
\`\`\`

If the diff-check job completes successfully that means that both the rustfmt binary and the forked rustfmt binary
agree upon formatting changes. However, if the job fails because both binaries produced different formatting, you
can inspect the differences by running:

\`\`\`
for file in \$(find diff-check -type f -name diff.txt); cat \$file
\`\`\`

If you're curious you can inspect formatting changes produced when running rustfmt by running:

\`\`\`
for file in \$(find diff-check -type f -name rustfmt_diff.txt); cat \$file
\`\`\`

Similarly, you can inspect formatting changes produced when running the forked rustfmt binary by running:

\`\`\`
for file in \$(find diff-check -type f -name feature_diff.txt); cat \$file
\`\`\`
EOL
}

# Zip up all the diff changes detected by the script
#
# Globlas:
# $OUTPUT_DIR: Output directory where all `*diif.txt` files are written to. Set in `main`.
# $CURRENT_DIR: The directory where the script was run from. Set in `main`.
function zip_up_diffs() {
    cd $OUTPUT_DIR
    write_readme

    # Just to clean things up we'll make sure to remove empty files and directories
    find . -type f -empty -delete
    find . -type d -empty -delete
    zip -q -r $CURRENT_DIR/diff-check .
}

function main() {
    CURRENT_DIR=$(pwd)
    tmp_dir=$(mktemp -d -t rustfmt-XXXXXXXX)
    echo Created tmp_dir $tmp_dir

    compile_rustfmt $tmp_dir
    OUTPUT_DIR=$(mktemp -d -t diff-output-XXX)

    # run checks
    check_repo "https://github.com/rust-lang/rust.git" rust-lang-rust
    check_repo "https://github.com/rust-lang/cargo.git" cargo
    check_repo "https://github.com/rust-lang/miri.git" miri
    check_repo "https://github.com/rust-lang/rust-analyzer.git" rust-analyzer
    check_repo "https://github.com/bitflags/bitflags.git" bitflags
    check_repo "https://github.com/rust-lang/log.git" log
    check_repo "https://github.com/rust-lang/mdBook.git" mdBook
    check_repo "https://github.com/rust-lang/packed_simd.git" packed_simd
    check_repo "https://github.com/rust-lang/rust-semverver.git" check_repo
    check_repo "https://github.com/Stebalien/tempfile.git" tempfile
    check_repo "https://github.com/rust-lang/futures-rs.git" futures-rs
    check_repo "https://github.com/dtolnay/anyhow.git" anyhow
    check_repo "https://github.com/dtolnay/thiserror.git" thiserror
    check_repo "https://github.com/dtolnay/syn.git" syn
    check_repo "https://github.com/serde-rs/serde.git" serde
    check_repo "https://github.com/rust-lang/rustlings.git" rustlings
    check_repo "https://github.com/rust-lang/rustup.git" rustup
    check_repo "https://github.com/SergioBenitez/Rocket.git" Rocket
    check_repo "https://github.com/rustls/rustls.git" rustls
    check_repo "https://github.com/rust-lang/rust-bindgen.git" rust-bindgen
    check_repo "https://github.com/hyperium/hyper.git" hyper
    check_repo "https://github.com/actix/actix.git" actix
    check_repo "https://github.com/denoland/deno.git" denoland_deno

    zip_up_diffs

    # cleanup temp dir
    echo removing tmp_dir $tmp_dir and $OUTPUT_DIR
    rm -rf $tmp_dir
    rm -rf $OUTPUT_DIR

    # figure out the exit code
    for status in ${STATUSES[@]}
    do
        if [ $status -eq 1 ]; then
            echo "formatting diff found 💔"
            return 1
        fi
    done

    echo "no diff found 😊"
}

main
