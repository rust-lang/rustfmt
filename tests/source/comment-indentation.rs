/***************************************
 * Misc comments indentation tests
 ***************************************/

/*********************************
 * Several comments on one Compare line
************************************/
// Test 1 - several comments on one Compare line
fn main() {
    if    /*w*/    5    /*x*/    ==    /*y*/    6   /*z*/ {}
    }
// Test 2 - several comments on one Compare line    
fn main() {
        if              /* y */ a == b               /* x */ {}
        }
// Test 3 - several comments on one Cast line
fn main() {
        let x          /* yyy */  =                    /* x as */ 0f64            as i32;
        }

/*********************************
 * Comment remain in new line or code line - as in original code
************************************/
// Test 11 - Comment remain in new line as in original code
fn main() {
    if
    // comment1
    a        ==         b
    {}
    }
// Test 12 - Comment remain in new line as in original code    
fn main() {
    if           true
               // else-if-chain if comment
    {
        let             foo       =              ();
    }
}
// Test 13 - Comment remain in line of code as in original code
fn main() {
    if             // comment1
    a         ==           b
    {}
    }
// Test 14 - Comment remain in line of code as in original code
fn main() {
    if a == b               // x
    {}
    }
// Test 15 - Comment after assignment remain in new line and indented as the rhs code
fn main() {
    let xxxxx =
    // after_comment
    yyyyyy;
}
// Test 16 - Comment after assignment remain in code line
fn main() {
    let xxxxx = // after_comment
    yyyyyy;
}

/*********************************
 * Comment remain in new line or code line - as in original BinOp expression code
************************************/
// Test 21 - Comment remain in new line   
fn main () {
    let xxxxxxxx = yyyyyyyyyyy
        && zzzzzzzzzzzzzzzz
        // cccccccccccccccc
        && uuuuuuuuuuuu;
}
// Test 22 - Comment remain in new line
fn main () {
    let xxxxxxxx = yyyyyyyyyyy
        // cccccccccccccccc
        && uuuuuuuuuuuu;
}
// Test 23 - Comment remain in new line
fn main () {
    let xxxxxxxx =
        // cccccccccccccccc
        uuuuuuuuuuuu;
}
// Test 24 - Comment remain in new line
fn main () {
    let xxxxxxxx =
    // cccccccccccccccc
        uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu
        && tttttttttttttttttttttttttttttttttttttttttttt;
}
// Test 25 - Comment remain in code line
fn main () {
    let xxxxxxxx = // cccccccccccccccc
        uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu
        && tttttttttttttttttttttttttttttttttttttttttttt;
}
// Test 25- Comment remain in code line
fn main () {
    let xxxxxxxx =
        uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu // cccccccccccccccc
        && tttttttttttttttttttttttttttttttttttttttttttt;
}
// Test 26- Comment moved to new line because will exceed line width ??????????????????????????????
fn main () {
    let xxxxxxxxxxxxxxxxxxxxxxxxxxx =
        uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu // cccccccccccccccc
        && tttttttttttttttttttttttttttttttttttttttttttt;
}

/*********************************
 * Comment moved to new line after block starting '{' ?????????????????????????????????????
************************************/
// Test 31 - comment moved to new line after '{'
fn main() {
    if a == b {              /* x */
    c = 7;
    }
    }

// Test 32 - comment moved to new line after '{'
fn main () {
        if xxxxxxxx { // cccccccccccccccc
            yyyyyyyyy = uuuuuuuuuuuu;
        }
    }    

/*********************************
 * Comments in a list are alligned per the longest list item,
 * up to first multi comments per item or multi-line comment.
************************************/
// Test 41 - one-line comments - only one per list item
fn main() {
    let v = ["A", /* item comment */
        "B", /* item comment */
        "Ccccccccc", /* item comment */
        "Ddd", /* item comment */
        ];
}
// Test 42 - one-line and multi-line comments - only one per list item
fn main() {
    let v = ["A", /* item comment */
        "B", /* item comment */
        "Ccccccccc", /* item comment line 1
                  * line 2 */
        "Ddd", /* item comment */
        ];
}
// Test 43 - one-line and multi-line comments per same item
fn main() {
    let v = ["A",
        "B",
        "Ccccccccc", /* Multiline comment line 1 *
                  * Multiline comment line 2 */
                  /* Additional comment 2 */
        "Ddd",
        ];
}
// Test 44 - one-line and multi-line comments - several comments per item
fn main() {
        let v = ["A",
            "BB", /* Comment for prev multiline comment param */
            "Ccccccccc", /* Multiline comment line 1 *
                      * Multiline comment line 2 */
                      /* Additional comment 2 */
            "Ddd",	/* Yet another multiline comment line 1 *
                       * Yet another multiline comment line 2 */
            "EEEE",	/* And another multiline comment line 1 *
                       * And another multiline comment line 2 */
            ];
    }
// Test 45 - one-line and multi-line comments - several comments per item
fn main(
        p1: Binary, /* comment for prev multiline comment param */
        param2: Binary, /* Multiline comment line 1 *
                      * Multiline comment line 2 */
                /* Additional comment */
    ) {
       x = 7;
    }

/*********************************
 * Comments in Arithmetic expression
************************************/
// Test 51
fn main () {
	let y = 2 /* 1 This is the first bin op argument *
		  * 2 which will be multiplied by the second argument */
		* /* 3 This is the multiplying operation *
		  * 4 which multiplies both arguments */
		4 /* 5 This is the second argument *
		  * 6 which is multiplied by the first argument */;
	/* 7 And this is a comment for the next operation *
	 * 8 and some more info */
	let i = 7;
}
// Test 52
fn main () {
	let y = 2 /* 1 This is the first bin op argument */
		  /* 2 which will be multiplied by the second argument */
		* /* 3 This is the multiplying operation */
		  /* 4 which multiplies both arguments */
		4 /* 5 This is the second argument */
		  /* 6 which is multiplied by the first argument */;
	/* 7 And this is a comment for the next operation */
	/* 8 and some more info */
	let i = 7;
}
// Test 53
fn main () {
let y = 2
*
4 /* C5 */
/* C6 */;
}
// Test 54
fn main () {
let y = 2
*
4 /* C5 *
* C6 */;
}
// Test 55
fn main () {
let y = 2
*
4 /* 5 This is the second argument */
/* 6 which is multiplied by the first argument */;
}
// Test 56
fn main () {
let y = /* 0 Multiplication of two numebers */
2
*
4 /* 5 This is the second argument */
/* 6 which is multiplied by the first argument */;
/* 7 And this is a comment for the next operation */
let i = 7;
}
// Test 57
fn main () {
let y = /* 0 Multiplication of two numebers */
2
*
4; /* 5 This is the second argument */
/* 6 which is multiplied by the first argument */
/* 7 And this is a comment for the next operation */
let i = 7;
}

/*********************************
 * Comments before and after end of expression ';'
************************************/
// Test 61 - comments before and after ';' - first comment same line of code
fn main() {
    let y = 2
                    /* 5 This is the second argument */
        /* 6 which is multiplied by the first argument */;
                      /* 7 And this is a comment for the next operation */
}
// Test 62 - comments before and after ';' - first comment in new line
fn main() {
    let y = 2         /* 5 This is the second argument */
        /* 6 which is multiplied by the first argument */;
                      /* 7 And this is a comment for the next operation */
}
// Test 63 - comments after ';' with following expression
fn main() {
    let y = 2 * 4;
                   /* 5 This is the second argument */
        /* 6 which is multiplied by the first argument */
                                 /* 7 And this is a comment for the next operation */
    let i = 7;
}
