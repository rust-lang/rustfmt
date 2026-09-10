#!/bin/bash

# Install the latest nightly rust toolchain
# We want to perform a subtree-push from the latest nightly rust-lang/rust -> rustfmt
# In order to do so, make sure we've go the latest nightly toolchain installed
function install_latest_nightly() {
    rustup update nightly --no-self-update
}

# Follows the steps outlined in the Clippy docs to get a patched version of git-subtree that works
# with larger repos.
# This is necessary to push commits from rust-lang/rust -> rustfmt
# https://doc.rust-lang.org/nightly/clippy/development/infrastructure/sync.html#patching-git-subtree-to-work-with-big-repos
function get_patched_subtree() {
    local CLONE_DIR=$1
    local PATCHED_GIT_SUBTREE_FORK="https://github.com/tqc/git.git"
    local PATCHED_BRANCH="tqc/subtree"

    GIT_TERMINAL_PROMPT=0 git clone --branch $PATCHED_BRANCH --single-branch -q --depth 1 $PATCHED_GIT_SUBTREE_FORK $CLONE_DIR

    local SUBTREE_SCRIPT_PATH="contrib/subtree/git-subtree.sh"
    local FULL_SUBTREE_SCRIPT_PATH="$CLONE_DIR/$SUBTREE_SCRIPT_PATH"

    echo "Patching git-subtree using fork:$PATCHED_GIT_SUBTREE_FORK branch:$PATCHED_BRANCH"

    sudo cp --backup $FULL_SUBTREE_SCRIPT_PATH /usr/lib/git-core/git-subtree
    sudo chmod --reference=/usr/lib/git-core/git-subtree~ /usr/lib/git-core/git-subtree
    sudo chown --reference=/usr/lib/git-core/git-subtree~ /usr/lib/git-core/git-subtree
}

# Extract various details from rustc's verbose version output e.g `rustc -Vv`
function parse_rustc_verbose_version_info() {
    local RUSTC_VERBOSE_VERSION_INFO=$1
    # valid values are: `binary`, `commit-hash`, `commit-date`, `host`, `release`, `LLVM version`
    local INFO_KEY=$2
    echo $(cut -d ' ' -f2  <<< $(echo "$RUSTC_VERBOSE_VERSION_INFO=" | grep "$INFO_KEY:"))
}

# Parses the `commit-hash` from rustc verbose version output e.g `rustc -Vv`
function get_commit_hash() {
    local RUSTC_VERBOSE_VERSION_INFO=$1
    echo $(parse_rustc_verbose_version_info "$RUSTC_VERBOSE_VERSION_INFO" "commit-hash")
}

# Parses the `commit-date` from rustc verbose version output e.g `rustc -Vv`
function get_commit_date() {
    local RUSTC_VERBOSE_VERSION_INFO==$1
    echo $(parse_rustc_verbose_version_info "$RUSTC_VERBOSE_VERSION_INFO" "commit-date")
}

# Parses the `release` from rustc verbose version output e.g `rustc -Vv`
function get_release_number() {
    local RUSTC_VERBOSE_VERSION_INFO==$1
    echo $(parse_rustc_verbose_version_info "$RUSTC_VERBOSE_VERSION_INFO" "release")
}

# The nightly toolchain always has a commit date that is 1 day behind.
# This will help us get the correct release date for the toolchain
function toolchain_date() {
    # Should be a date string in the form YYYY-MM-DD.
    # We should get this date using `get_commit_date`
    local DATE=$1
    echo $(date --rfc-3339=date --date="$DATE+1day")
}

# Sets the new toolchain version in rustfmt's `rust-toolchain` file
function bump_rust_toolchain_version() {
    local CURRENT_TOOLCHAIN=$1
    local LATEST_TOOLCHAIN=$2
    local TOOLCHAIN_FILE="rust-toolchain"
    echo "Bumping the toolchain listed in $TOOLCHAIN_FILE from $CURRENT_TOOLCHAIN -> $LATEST_TOOLCHAIN"

    NEW_TOOLCHAIN_FILE=$(cat $TOOLCHAIN_FILE | sed "s/$CURRENT_TOOLCHAIN/$LATEST_TOOLCHAIN/g")
    echo "$NEW_TOOLCHAIN_FILE" > $TOOLCHAIN_FILE
}

# Clone the main branch of the rust-lang/rust repo
function clone_rustlang_rust() {
    local CLONE_DIR=$1
    local RUSTLANG_RUST_GIT_URL=$2
    echo "Cloning $RUSTLANG_RUST_GIT_URL into $CLONE_DIR"
    # Do we need the entire git history? Would it suffice to just get the history from the last full subtree sync?
    git clone --branch main --single-branch $RUSTLANG_RUST_GIT_URL $CLONE_DIR
    cd $CLONE_DIR
}

# Dynamically get the name of the HEAD branch.
function get_main_branch_name() {
    REMOTE_NAME=$1
    echo $(git remote show $REMOTE_NAME | grep 'HEAD branch' | cut -d' ' -f5)
}

# Get rustfmt's repository URL
function rustfmt_repository_url() {
    # This is one of the default environment variables set by GitHub Actions
    # https://docs.github.com/en/actions/learn-github-actions/variables#default-environment-variables
    local GITHUB_REPOSITORY=$GITHUB_REPOSITORY
    echo "https://github.com/$GITHUB_REPOSITORY"
}

# Get rustfmt's git URL
function rustfmt_git_url() {
    echo "$(rustfmt_repository_url).git"
}


# Follows instructions outlined by the Clippy docs to perform a subtree push
# https://doc.rust-lang.org/nightly/clippy/development/infrastructure/sync.html#syncing-changes-between-clippy-and-rust-langrust
function rustc_to_rustfmt_subtree_push() {
    local CLONE_DIR=$1
    local LATEST_NIGHTLY_COMMIT=$2
    local LAST_SUBTREE_PUSH_COMMIT=$3
    local RUSTFMT_LOCAL_PATH=$4
    local NEW_BRANCH_NAME=$5
    local RUSTLANG_RUST_GIT_URL=$6
    local LOCAL_RUSTFMT_REPO_ALIAS="rustfmt-local"
    # Path to the rustfmt subtree within the rust-lang/rust repo
    local RUSTFMT_TOOLS_PATH="src/tools/rustfmt"

    get_patched_subtree "$CLONE_DIR/git"

    # cloning will also CD into the rust-lang/rust repo
    clone_rustlang_rust "$CLONE_DIR/rust" $RUSTLANG_RUST_GIT_URL
    git remote add $LOCAL_RUSTFMT_REPO_ALIAS $RUSTFMT_LOCAL_PATH

    # The subtree-push doesn't necessarily happen with the HEAD of the rust-lang/rust repo.
    # We want to `push` changes up to whatever commit was last released.
    git switch --detach $LATEST_NIGHTLY_COMMIT
    # Need to bump up the stack size as we're about to go through the entire rustfmt history
    ulimit -s 60000
    echo "Running the subtree push"
    # The logs get really noisy so redirect everything to /dev/null
    git subtree push -P $RUSTFMT_TOOLS_PATH $LOCAL_RUSTFMT_REPO_ALIAS $NEW_BRANCH_NAME > /dev/null 2>&1
}

# Try to create a merge commit for the latest subtree-push
# A merge commit is only created if the changes from rust-lang/rust apply cleanly to rustfmt.
function try_create_subtree_push_merge_commit() {
    local RUSTFMT_REPO_PATH=$1
    local SUBTREE_PUSH_BRANCH_NAME=$2
    local REMOTE_HEAD_BRANCH=$3
    local REMOTE_REF="origin/$REMOTE_HEAD_BRANCH"

    cd $RUSTFMT_REPO_PATH
    git fetch origin $REMOTE_HEAD_BRANCH
    git switch $SUBTREE_PUSH_BRANCH_NAME

    echo "Trying create merge commit between $SUBTREE_PUSH_BRANCH_NAME and $REMOTE_REF"

    git merge $REMOTE_REF --no-ff --no-commit
    if [ $? -eq 0 ]; then
        echo "The subtree push was clean 😁. Creating a merge commit."
        git commit --no-edit
        return 0
    else
        echo "Unfortunately there are merge conflicts that need to be addressed 😢"
        git merge --abort
        return 1
    fi
}

# Create a Pull Request for the subtree-push
function create_subtree_push_pull_request() {
    local CLEAN_MERGE_COMMIT=$1
    local CURRENT_TOOLCHAIN=$2
    local LATEST_TOOLCHAIN=$3
    local COMMIT_MESSAGE=$4
    local NEW_BRANCH_NAME=$5
    local REMOTE_HEAD_BRANCH=$6

    # This is one of the default environment variables set by GitHub Actions
    # https://docs.github.com/en/actions/learn-github-actions/variables#default-environment-variables
    local GITHUB_REPOSITORY=$GITHUB_REPOSITORY
    local REPOSITORY_URL=$(rustfmt_repository_url)
    local GIT_URL=$(rustfmt_git_url)

    if [ $CLEAN_MERGE_COMMIT -eq 0 ]; then
        # No merge conflicts!!
        bump_rust_toolchain_version "$CURRENT_TOOLCHAIN" "$LATEST_TOOLCHAIN"
        git add rust-toolchain
        echo "Creating commit for the new $LATEST_TOOLCHAIN toolchain"
        git commit -m "$COMMIT_MESSAGE"
    fi

    # whether the merge commit applied cleanly or not create a PR
    # I believe the remote repo will always be `origin` in GitHub Actions
    echo "Pushing $NEW_BRANCH_NAME to $GIT_URL"
    git push origin $NEW_BRANCH_NAME
    # Skip the title of the commit
    local PR_MESSAGE=$(echo "$COMMIT_MESSAGE" | tail -n +2)
    local PR_URL=$(gh pr create --title "subtree-push $LATEST_TOOLCHAIN" --body "$PR_MESSAGE")

    echo "Created Pull Request $PR_URL"

    # TODO(ytmimi)
    # For convenience, if the commit applied cleanly, we could kick off the
    # Diff-Check Job using the GitHub CLI: https://cli.github.com/manual/gh_workflow_run
    DIFF_CHECK_URL="$REPOSITORY_URL/actions/workflows/check_diff.yml"

    if [ $CLEAN_MERGE_COMMIT -eq 0 ]; then
        gh pr comment $PR_URL --body "The subtree-push applied cleanly ✅.

Take a moment to review the changes. You'll also want to Run the [Diff-Check] job.

**Diff-Check Job Parameters**:
- Git URL: $GIT_URL
- Feature Branch: $NEW_BRANCH_NAME

After CI and the [Diff-Check] job pass this PR should be good to merge!

[Diff-Check]: $DIFF_CHECK_URL
"
    else
        gh pr comment $PR_URL --body "There was an issue and this subtree-push can't automatically be merged ⚠️

1. Please checkout branch \`$NEW_BRANCH_NAME\`, fix any merge conflicts, and then run \`git merge upstream/$REMOTE_HEAD_BRANCH --no-ff\`
2. Bump the toolchain listed in the \`rust-toolchain\` file to \`$LATEST_TOOLCHAIN\`, and commit those changes.
3. Run the [Diff-Check] Job.
   - **Diff-Check Job Parameters**:
     - Git URL: $GIT_URL
     - Feature Branch: $NEW_BRANCH_NAME
4. Wait for CI checks to pass.

[Diff-Check]: $DIFF_CHECK_URL
"
    fi

    # TODO(ytmimi): notify the team that a new subtree-push PR was created.
    # Additionally, include whether the subtree-push applied cleanly or not.
    # miri publishes messages to Zulip. I feel like we could do the same.
    # https://github.com/rust-lang/miri/blob/f006d42618a038f7e38d2b59d1b0664727e51382/.github/workflows/ci.yml#L205-L210
}

# Create a new Pull Request in the rustfmt repository for the git subtree-push
#
# **Note:** The Pull Request is created regardless if the changes from rust-lang/rust
# apply cleanly to rustfmt or not. If they apply cleanly, then great! All that's left to
# do is review and merge the changes. If there are conflicts one of the rustfmt maintainers
# will need to address those, create a merge commit, bump the nightly toolchain, and wait for CI to pass.
function run_rustfmt_subtree_push() {
    local RUSTLANG_RUST_URL=$1
    # Assumes that the current working directory is the root of the rustfmt repo
    local CWD=$(pwd)
    # TMP DIR used to clone the rust-lang/rust repo and a patched `git subtree` command
    local TMP_DIR=$(mktemp -d)
    local NIGHTLY="nightly"

    # Running `rustc -Vv` in the rustfmt repo should give us details for the nightly toolchain
    # specified in the `rust-toolchain` file
    CURRENT_RUSTFMT_RUSTC_VERSION=$(rustc -Vv)
    CURRENT_RUSTFMT_RUSTC_COMMIT_HASH=$(get_commit_hash "$CURRENT_RUSTFMT_RUSTC_VERSION")
    CURRENT_RUSTFMT_RUSTC_COMMIT_DATE=$(get_commit_date "$CURRENT_RUSTFMT_RUSTC_VERSION")
    CURRENT_RUSTFMT_RUSTC_RELEASE=$(get_release_number "$CURRENT_RUSTFMT_RUSTC_VERSION")
    CURRENT_TOOLCHAIN_DATE=$(toolchain_date "$CURRENT_RUSTFMT_RUSTC_COMMIT_DATE")
    CURRENT_TOOLCHAIN="$NIGHTLY-$CURRENT_TOOLCHAIN_DATE"

    echo "CURRENT_TOOLCHAIN: $CURRENT_TOOLCHAIN"

    # Running `rustc +nightly -Vv` should give us details about the latest nightly toolchain
    LATEST_NIGHTLY_RUSTC_VERSION=$(rustc +$NIGHTLY -Vv)
    LATEST_NIGHTLY_RUSTC_COMMIT_HASH=$(get_commit_hash "$LATEST_NIGHTLY_RUSTC_VERSION")
    LATEST_NIGHTLY_COMMIT_DATE=$(get_commit_date "$LATEST_NIGHTLY_RUSTC_VERSION")
    LATEST_NIGHTLY_RELEASE=$(get_release_number "$LATEST_NIGHTLY_RUSTC_VERSION")
    LATEST_TOOLCHAIN_DATE=$(toolchain_date "$LATEST_NIGHTLY_COMMIT_DATE")
    LATEST_TOOLCHAIN="$NIGHTLY-$LATEST_TOOLCHAIN_DATE"

    echo "LATEST_TOOLCHAIN $LATEST_TOOLCHAIN"

    COMMIT_MESSAGE="chore: bump rustfmt toolchain to $LATEST_TOOLCHAIN

Bumping the toolchain version as part of a git subtree push

current toolchain ($CURRENT_TOOLCHAIN):
  - $CURRENT_RUSTFMT_RUSTC_RELEASE (${CURRENT_RUSTFMT_RUSTC_COMMIT_HASH:0:9} $CURRENT_RUSTFMT_RUSTC_COMMIT_DATE)

latest toolchain ($LATEST_TOOLCHAIN):
  - $LATEST_NIGHTLY_RELEASE (${LATEST_NIGHTLY_RUSTC_COMMIT_HASH:0:9} $LATEST_NIGHTLY_COMMIT_DATE)
"

    NEW_BRANCH_NAME="subtree-push-$LATEST_TOOLCHAIN"

    rustc_to_rustfmt_subtree_push \
        $TMP_DIR \
        $LATEST_NIGHTLY_RUSTC_COMMIT_HASH \
        $CURRENT_RUSTFMT_RUSTC_COMMIT_HASH \
        $CWD \
        $NEW_BRANCH_NAME \
        "${RUSTLANG_RUST_URL}.git"

    if [ $? -eq 0 ]; then
        echo "subtree push was successfull. The $NEW_BRANCH_NAME branch should be available in the local rustfmt repo"
    else
        echo "Failed to create a subtree push from the local rust-lang/rust clone"
        return 1
    fi

    echo "Switching back to rustfmt to finish the subtree push"
    cd $CWD
    git switch $NEW_RUSTFMT_BRANCH

    local RUSTFMT_REMOTE_HEAD=$(get_main_branch_name "origin")
    try_create_subtree_push_merge_commit $CWD $NEW_BRANCH_NAME $RUSTFMT_REMOTE_HEAD
    CLEAN_MERGE_COMMIT=$?

    create_subtree_push_pull_request \
        $CLEAN_MERGE_COMMIT \
        $CURRENT_TOOLCHAIN \
        $LATEST_TOOLCHAIN \
        "$COMMIT_MESSAGE" \
        $NEW_BRANCH_NAME \
        $RUSTFMT_REMOTE_HEAD

    rm -rf $TMP_DIR
}

function print_help() {
    echo "Tools to help automate subtree syncs

usage: subtree_sync.sh <command> [<args>]

commands:
    subtree-push           Push changes from rust-lang/rust back to rustfmt.
"
}

function main() {
    local COMMAND=$1
    local RUSTLANG_RUST_URL=$2

    echo "Running Command $COMMAND"

    case $COMMAND in
        subtree-push)
            install_latest_nightly
            run_rustfmt_subtree_push $RUSTLANG_RUST_URL
            ;;
        *)
            print_help
            ;;
    esac
}

main $@
