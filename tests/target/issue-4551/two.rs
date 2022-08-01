// rustfmt-version: Two

// From #4551

fn main() {
    /* Comment1 line 1
     * Comment1 line 2 */
    let ys = 1; /* Comment2 line 1
                 * Comment2 line 2 with preceeding tabs */
}

fn main() {
    /* Comment1 line 1
     * Comment1 line 2 */
    // Following1 comment
    let y1 = 6; /* Comment2 line 1
                 * Comment2 line 2 - prefixed by spaces and 1 tab */
    // Following2 comment
    let y2 = 6; /* Comment3 line 1
                 * Comment3 line 2 - prefixed by spaces and 2 tabs */
    /* Following3 comment */
    let y3 = 8; /* Comment4 line 1
                 * Comment4 line 2  - prefixed by spaces */
    /* Following4 comment
     * cont of Following4 comment */
}

// Other tests

fn main() {
    let x = 111; /* First Comment with spaces prefix for each line - line 1
                  * First Comment line 2
                  * First Comment line 3
                  */
    // Following first comment
}

fn main() {
    let x = 222; /* Second Comment with tab prefix for each line - line 1
                  * Second Comment line 2
                  * Second Comment line 3
                  */
    /* Following second comment line 1
     * Following second comment line 2
     * Following second comment line 3
     */
}

fn main() {
    let x = 333; /* Third Comment with spaces prefix for each line - line 1
                  * Third Comment line 2
                  * Third Comment line 3
                  */
    /* Following third comment line 1
     * Following third comment line 2
     * Following third comment line 3
     */
}

fn main() {
    let x = 222; /* Second Comment line 1
                  * Second Comment line 2
                  * Second Comment line 3
                  */
    let y = 333; // Following second comment line 1
    // Following second comment line 2
    // Following second comment line 3
}

fn main() {
    let y3 = 8; /* Comment4 line 1
                 * Comment4 line 2 */
}

fn main() {
    let y3 = 8; /* Comment4 line 1
                 * Comment4 line 2 */
    y4 = 9;
}
