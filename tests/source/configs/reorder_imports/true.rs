// rustfmt-reorder_imports: Alphabetically
// Reorder imports

use lorem;
use ipsum;
use dolor;
use sit;

fn foo() {
    use C;
    use B;
    use A;

    bar();

    use F;
    use E;
    use D;
}
