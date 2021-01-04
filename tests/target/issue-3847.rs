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

// Tests with one multi-line block comment
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

// Tests with one-line block-comments
type T5_good = Result<
    u32, /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam */
    /* diam ac cursus. Aliquam condimentum in erat quis pretium. */
    /* accumsan urna. Cras volutpat sit amet quam. */
    bool,
>;
type T6_good = Result<
    u32, /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam */
    /* diam ac cursus. Aliquam condimentum in erat quis pretium. */
    /* accumsan urna. Cras volutpat sit amet quam. */
    bool,
>;

// Tests with mix one-line and multi-linecomments - one-line is first
type T8_good = Result<
    u32, // Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam
    /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam
     * diam ac cursus. Aliquam condimentum in erat quis pretium.
     * accumsan urna. Cras volutpat sit amet quam. */
    bool,
>;
type T9_good = Result<
    u32, /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam */
    /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam
     * diam ac cursus. Aliquam condimentum in erat quis pretium.
     * accumsan urna. Cras volutpat sit amet quam. */
    // Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam
    bool,
>;
type T9_good = Result<
    u32, /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam */
    /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam
     * diam ac cursus. Aliquam condimentum in erat quis pretium.
     * accumsan urna. Cras volutpat sit amet quam. */
    // Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam
    bool,
>;

// Tests with mix one-line and multi-linecomments - multi-line is first
type T10_good = Result<
    u32, /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam
          * diam ac cursus. Aliquam condimentum in erat quis pretium.
          * accumsan urna. Cras volutpat sit amet quam. */
    // Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam
    bool,
>;
type T11_good = Result<
    u32, /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam
          * diam ac cursus. Aliquam condimentum in erat quis pretium.
          * accumsan urna. Cras volutpat sit amet quam. */
    /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam */
    // Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam
    bool,
>;
type T12_good = Result<
    u32, /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam
          * diam ac cursus. Aliquam condimentum in erat quis pretium.
          * accumsan urna. Cras volutpat sit amet quam. */
    /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam */
    // Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam
    bool,
>;
type T12_good = Result<
    u32, /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam
          * diam ac cursus. Aliquam condimentum in erat quis pretium.
          * accumsan urna. Cras volutpat sit amet quam. */
    /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam */
    // Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam
    bool,
>;
type T13_good = Result<
    u32, /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam
          * diam ac cursus. Aliquam condimentum in erat quis pretium.
          * accumsan urna. */
    /*Cras volutpat sit amet quam. */
    /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam */
    bool,
>;

// Tests with mix one-line and multi-linecomments -
// multi-line is first with newline between comments
type T14_good = Result<
    u32, /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam
          * diam ac cursus. Aliquam condimentum in erat quis pretium.
          * accumsan urna. Cras volutpat sit amet quam. */
    /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam */
    // Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam
    bool,
>;
type T15_good = Result<
    u32, /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam
          * diam ac cursus. Aliquam condimentum in erat quis pretium.
          * accumsan urna. Cras volutpat sit amet quam. */
    /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam */
    // Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam
    bool,
>;

// Tests with first comment is not in same line of item
type T16_good = Result<
    u32, /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam */
    /* diam ac cursus. Aliquam condimentum in erat quis pretium. */
    /* accumsan urna. Cras volutpat sit amet quam. */
    bool,
>;
type T17_good = Result<
    u32, /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam */
    /* diam ac cursus. Aliquam condimentum in erat quis pretium. */
    /* accumsan urna. Cras volutpat sit amet quam. */
    bool,
>;
