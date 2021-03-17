// With one comment per item - after the the `;`
use crate::foo1;
use crate::foo1; /* 2nd foo1 - comment after ; */
use crate::foo2::bar; /* 1st foo1::bar - comment after ; */
use crate::foo2::bar;

// With one comment per item - before the the `;`
use crate::foo3;
use crate::foo3; /* 2nd foo3 - comment before ; */
use crate::foo4::bar; /* 1st foo4::bar - comment before ; */
use crate::foo4::bar;

// With multiline comments or multi comments - after the `;`
use crate::foo5; /* foo5 - Multiline comment before ; line 1
                  * foo5 - Multiline comment before ; line 2 */
use crate::foo5;
use crate::foo6; // foo6- mixed comments before ; - 1st line comment ;
/* foo6- mixed comments before ; - 2nd block comment */
use crate::foo6;

// With multiline comments or multi comments - before the `;`
use crate::foo8; // foo8- mixed comments before ; - 1st line comment ;
/* foo8- mixed comments before ; - 2nd block comment */
use crate::foo8;

// With two comments per item
use crate::foo11; /* 1st foo11 - comment */
use crate::foo11; /* 2nd foo11 - comment */

// With one comment for a module
use crate::foo21::bar;
use crate::foo21::foo; /* external comment for foo21 {foo} */
use crate::foo21::{self}; /* external comment for foo21 {self} */

// With internal and external comment for a module
use crate::foo22::{self}; /* external comment for foo22 {self} */
use crate::foo22::{bar /* internal comment for foo22 {bar} */};
use crate::foo22::{foo /* internal comment for foo22 {foo} */};
