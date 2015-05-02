// Imports.

// Long import.
use syntax::ast::{ItemForeignMod, ItemImpl, ItemMac, ItemMod, ItemStatic,
                  ItemDefaultImpl};

use {Foo, Bar};
use Foo::{Bar, Baz};
pub use syntax::ast::{Expr_, Expr, ExprAssign, ExprCall, ExprMethodCall,
                      ExprPath};

// Nested imports.
mod foo {
    mod bar {
        mod baz {
            mod test {
                use Bar::{ItemForeignMod, ItemImpl, ItemMac, ItemMod,
                          ItemStatic, ItemDefaultImpl};

                fn test() {
                }
            }
        }
    }
}

