// rustfmt-imports_granularity: Item

// With one comment per item - after the the `;`
use crate::foo2::bar; /* 1st foo1::bar - comment after ; */

// With one comment per item - before the the `;`
use crate::foo4::bar; /* 1st foo4::bar - comment before ; */

// With multiline comments or multi comments - after the `;`
use crate::foo5; /* foo5 - Multiline comment before ; line 1
                  * foo5 - Multiline comment before ; line 2 */
use crate::foo6; // foo6- mixed comments before ; - 1st line comment ;
/* foo6- mixed comments before ; - 2nd block comment */

// With multiline comments or multi comments - before the `;`
use crate::foo8;

// With one comment for a module
use crate::foo21::bar;
use crate::foo21::foo; /* external comment for foo21 {foo} */
use crate::foo21::{self}; /* external comment for foo21 {self} */
