// rustfmt-binop_separator: Back

// Test 1
fn main() {
    let x = 2      /* x */             *              4;
}

// Test 2
fn main() {
    let x = 2      /* xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx */             *              4;
}

// Test 3
fn main() {
    let x = 2      *              /* yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy */             4;
}

// Test 4
fn main() {
    let x = 2        /* x */      *       /* y */     4;
}

// Test 5
fn main() {
    let x = 2        /* xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx */      *       
					/* yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy */     4;
}


// Test 11.1 - One line
fn main() {
	if context()                     &&            bounds.len() == 1                        ||           context&& !res.ends('+') {a+b}
}
// Test 11.2 - One line with comments
fn main() {
	if context() /* cond 1 */        &&            bounds.len() == 1 /* cond 2 */           ||           context&& !res.ends('+') {a+b}
}

// Test 12.1 - Pre-comments break to few lines - only "&&"
fn main() {
	if context() /* condition 1 expression */ && bounds.len() == 1 /* condition 2 expression */ && context /* condition 3 expression */ && !res.ends('+') {a+b}
}
// Test 12.2 - Pre-comments break to few lines - "&&" and "||"
fn main() {
	if context() /* condition 1 expression */ && bounds.len() == 1 /* condition 2 expression */ || context /* condition 3 expression */ && !res.ends('+') {a+b}
}

// Test 13.1 - Post-comments break to few lines - only "&&"
fn main() {
	if context() && /* condition 1 expression */ bounds.len() == 1 && /* condition 2 expression */ context && /* condition 3 expression */  !res.ends('+') /* closing comment */ {a+b}
}
// Test 13.2 - Pre and Post-comments break to few lines - only "&&"
fn main() {
	if context() /* pre comment 1 */ && /* post comment 1 */ bounds.len() == 1 /* pre comment 2 */ && /* post comment 2 */ context /* pre comment 3 */ && /* post comment 3 */  !res.ends('+') /* closing comment */ {a+b}
}

// Test 14.1 - Each "&&" starts new line
fn main() {
	if context.inside_macro() && bounds.len() == 1&& context.snippet(self.span).ends_with('+') & !res.ends_with('+') { a+b }
}
// Test 14.2 - "||" starts new line
fn main() {
	if context.inside_macro() && bounds.len() == 1 || context.snippet(self.span).ends_with('+') & !res.ends_with('+') { a+b }
}

// Test 15.1 - Each "&&" starts new line - with comments
fn main() {
	if context.inside_macro()  /* cond 1 */ && bounds.len() == 1 /* cond 2 */ && context.snippet(self.span).ends_with('+') /* cond 3 */ && !res.ends_with('+') { a+b }
}
// Test 15.2 - "||" starts new line - with comments
fn main() {
	if context.inside_macro()  /* cond 1 */ && bounds.len() == 1 /* cond 2 */ || context.snippet(self.span).ends_with('+') /* cond 3 */ && !res.ends_with('+') { a+b }
}

// Test 16 - Each "." and some "&&"/"||" starts new line
fn main() {
	if a {
		if b {
			if c {
				if d {
					if e {
				if item_kind.is_same_item_kind(&***ppi, self.file_mod_map).context.inside_macro()&& bounds.len() == 1 || context.snippet(self.span).ends_with('+')&& !res.ends_with('+') { a+b }
					}
				}
			}
		}
	}
}


// Test 17.1
fn main() {
    let x = 2        /* v */
	/* w */        *       /* x */
	        4   ;
}

// Test 17.2 
fn main() {
    let x = 2        /* v */
	/* w */        *       /* x */
	/* y */        4   ;
}


// Test 18.1 - Multi separators - One line Pre-comments
fn main() {
/* comments before the if statement */
    if context1()   /* condition 1 explanations */
        && bounds.len() == 1    /* condition 2 explanations */
        && context3 /* condition 3 explanations */
        && !res.ends('+') /* condition 4 explanations */
/* comments to the executed block */
    {a + b}
}

// Test 18.2 - Multi separators - One line Post-comments
fn main() {
	     /* comments before the if statement */
    if context1() &&   /* condition 1 explanations */
        bounds.len() == 1 &&    /* condition 2 explanations */
        context3 && /* condition 3 explanations */
        !res.ends('+') /* condition 4 explanations */
	    /* comments to the executed block */
    {a + b}
}

// Test 19.1 - Multi separators - Multi line Pre-comments
fn main() {
	/* comments before the if statement
	 * with some explanations about the if */
    if context1()   /* condition 1 explanations
		            * with some added details about condition 1 */
        && bounds.len() == 1    /* condition 2 explanations
		                   * with some added details about condition 2 */
        && context3 /* condition 3 explanations
		                * with some added details about condition 3 */
        && !res.ends('+') /* condition 4 explanations
		                   * with some added details about condition 4 */
	/* comments to the executed block
	 * with some explanations about the executed block */
    {a + b}
}

// Test 19.2 - Multi separators - Multi line Post-comments
fn main() {
	/* comments before the if statement
	 * with some explanations about the if */
    if context1() /* pre condition1 */ &&  /* condition 1 explanations
		              * with some added details about condition 1 */
        bounds.len() == 1 &&   /* condition 2 explanations
		                   * with some added details about condition 2 */
        context3 && /* condition 3 explanations
		                * with some added details about condition 3 */
        !res.ends('+') /* condition 4 explanations
		                   * with some added details about condition 4 */
	/* comments to the executed block
	 * with some explanations about the executed block */
    {a + b}
}