// rustfmt-imports_granularity: Crate

// With one comment per item - after the the `;`
use crate::foo2::bar; /* 1st foo1::bar - comment after ; */
use foo1; /* 2nd foo1 - comment after ; */

// With one comment per item - before the the `;`
use crate::foo4::bar; /* 1st foo4::bar - comment before ; */
use foo3; /* 2nd foo3 - comment before ; */

// With multiline comments or multi comments - after the `;`
use crate::foo5; /* foo5 - Multiline comment before ; line 1
                  * foo5 - Multiline comment before ; line 2 */
use crate::{foo5, foo6}; // foo6- mixed comments before ; - 1st line comment ;
/* foo6- mixed comments before ; - 2nd block comment */
use crate::foo6;

// With multiline comments or multi comments - before the `;`
use crate::foo8; // foo8- mixed comments before ; - 1st line comment ;
/* foo8- mixed comments before ; - 2nd block comment */
use crate::{foo7, foo8}; /* foo7 - Multiline comment before ; line 1
                          * foo7 - Multiline comment before ; line 2 */

// With one comment for a module
use crate::foo21::{self, bar, foo}; /* external comment for foo21 {self} */
