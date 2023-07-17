// rustfmt-version: One
// rustfmt-hard_tabs: true

// Ensure multiline comments are indented properly,
// including when second line is prefixed by tab or at the beginning of the line

/* First comment line
* second comment line - no prefix
* last comment line */

/* First comment line
 * second comment line - blank prefix
 * last comment line */

	/* First comment line
	* second comment line - tab prefix
	* last comment line */

/* First comment line
                    * second comment line - blank prefix
* last comment line - no prefix */

/* First comment line
                 * second comment line - blank prefix
          * last comment line */

type T1 = TT<
u32, /* First comment line
* second comment line - no prefix
* last comment line */
>;

type T2 = TT<
u32, /* First comment line
      * second comment line - blank prefix
      * last comment line */
>;

type T2 = TT<
u32, /* First comment line
	* second comment line - tab prefix
	* last comment line */
>;

type T3 = TT<
u32,	/* First comment line
		* second comment line - tab prefix
		* last comment line */
>;
