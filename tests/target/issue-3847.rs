// Tests for multi one-line post-comments of list item
// (related cases when `normalize_comments` is set are already included in other test files).

// Original cases from issue #3847
type T1 = Result<
    u32, // Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam
    // diam ac cursus. Aliquam condimentum in erat quis pretium.
    // accumsan urna. Cras volutpat sit amet quam.
    bool,
>;
type T2 = Result<
    u32, // Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam
    // diam ac cursus. Aliquam condimentum in erat quis pretium.
    // accumsan urna. Cras volutpat sit amet quam.
    bool,
>;

// Additional test cases with lists
fn main() {
    let a = [
        "GOOD", // Comment1
        // Comment2
    ];
    let b = [
        "WASBAD", // Comment1
        // Comment2
        "CCC",
    ];
}

// Additional tests with multi-line comments
type T3_good = Result<
    u32, /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam
          * diam ac cursus. Aliquam condimentum in erat quis pretium.
          * accumsan urna. Cras volutpat sit amet quam. */
    bool,
>;
type T4_good = Result<
    u32, /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam
          * diam ac cursus. Aliquam condimentum in erat quis pretium.
          * accumsan urna. Cras volutpat sit amet quam. */
    bool,
>;

fn main() {
    let a = [
        "WASGOOD1", /* Comment1
                     * Comment2 */
        "WASGOOD2", /* Comment1
                     * Comment2 */
        "CCC",
    ];
}
