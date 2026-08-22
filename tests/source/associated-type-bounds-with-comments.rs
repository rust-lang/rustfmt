#![feature(associated_type_defaults)]

// Exhaustive coverage of comments around associated-type bounds, keyed by
// position relative to the `:` / bounds / `where`, comment kind (line,
// block, multi-line line, multi-line block), and context (trait item with
// bounds, trait item without bounds, GAT, impl/free-alias shared path,
// bounds + RHS default). See #6761.
//
// Out of scope: a comment after bounds with no `where` clause at all (#6815)
// — that region has no span computed for it, so it needs separate handling.

trait Bound {}
impl Bound for () {}

// P1: between the ident/generics and the `:`.
trait P1 {
    type A // line
    : Bound
    where
        Self: Copy;

    type B /* inline */: Bound
    where
        Self: Copy;

    type C
    // multi-
    // line
    : Bound
    where
        Self: Copy;

    type D
    /* multi-
     * line
     */
    : Bound
    where
        Self: Copy;
}

// P2: between the `:` and the first bound.
trait P2 {
    type A: // line
        Bound
    where
        Self: Copy;

    type B: /* inline */ Bound
    where
        Self: Copy;

    type C:
        // multi-
        // line
        Bound
    where
        Self: Copy;

    type D:
        /* multi-
         * line
         */
        Bound
    where
        Self: Copy;
}

// P3: inside the bounds (generic args). Original #6761 repro.
trait P3 {
    type A: Iterator<
        // line
        Item = Self,
    >
    where
        Self: Copy;

    type B: Iterator<
        /* inline */
        Item = Self,
    >
    where
        Self: Copy;

    type C: Iterator<
        // multi-
        // line
        Item = Self,
    >
    where
        Self: Copy;

    type D: Iterator<
        /* multi-
         * line
         */
        Item = Self,
    >
    where
        Self: Copy;
}

// P4: between two bounds.
trait P4 {
    type A: Bound + // line
        Bound
    where
        Self: Copy;

    type B: Bound + /* inline */ Bound
    where
        Self: Copy;

    type C: Bound +
        // multi-
        // line
        Bound
    where
        Self: Copy;

    type D: Bound +
        /* multi-
         * line
         */
        Bound
    where
        Self: Copy;
}

// P5: after the last bound, before `where`. This is the duplication bug.
trait P5 {
    type A: Bound
    // line
    where
        Self: Copy;

    type B: Bound /* inline */
    where
        Self: Copy;

    type C: Bound
    // multi-
    // line
    where
        Self: Copy;

    type D: Bound
    /* multi-
     * line
     */
    where
        Self: Copy;
}

// P5, same-line trailing comment: the reviewer's exact input on #7014.
// rustfmt moves this onto its own line. That is `rewrite_where_keyword`'s
// shared behavior for every item kind (see tests/target/issue-3194.rs for
// struct/enum and type-alias-where-clauses-with-comments.rs for aliases),
// not something this fix introduces. Pinned here so the behavior is explicit.
trait P5SameLine {
    type A: Iterator<Item = u8> // trailing
    where
        Self: Copy;

    type B: Bound // trailing
    where
        Self: Copy;

    type C: Bound /* trailing */
    where
        Self: Copy;
}

// P5, no bounds: exercises the preserved `generics.span.hi()` default.
trait P5NoBounds {
    type A
    // line
    where
        Self: Copy;

    type B /* inline */
    where
        Self: Copy;

    type C
    // multi-
    // line
    where
        Self: Copy;

    type D
    /* multi-
     * line
     */
    where
        Self: Copy;
}

// Width: a colon comment must not wrap while the line still fits in
// max_width. `combine_strs_with_missing_comments` already measures the whole
// prefix, so it must be given the item shape, not the bounds-offset shape,
// or the prefix is charged twice and the line breaks early.
trait SomeModeratelyLongTraitName {}
trait ColonCommentWidth {
    type AssociatedTypeWithLongName: /* comment */ SomeModeratelyLongTraitName
    where
        Self: Copy;
}

// P6: after the `where` keyword, before the first predicate.
trait P6 {
    type A: Bound
    where
        // line
        Self: Copy;

    type B: Bound
    where
        /* inline */
        Self: Copy;

    type C: Bound
    where
        // multi-
        // line
        Self: Copy;

    type D: Bound
    where
        /* multi-
         * line
         */
        Self: Copy;
}

// GAT with bounds: non-empty `generics.span`.
trait GatP5 {
    type A<U>: Bound
    // line
    where
        U: Copy;

    type B<U>: Bound /* inline */
    where
        U: Copy;
}

// Impl associated type: shared rewrite_ty/where-clause path.
trait ImplTarget {
    type A<U>
    where
        U: Copy;
}
struct S;
impl ImplTarget for S {
    type A<U>
    // line, impl assoc type
    where
        U: Copy,
    = Vec<U>;
}

// Free type alias: shared rewrite_ty/where-clause path.
type Free<U>
// line, free alias
where
    U: Copy,
= Vec<U>;

// Bounds + RHS default, with comments (was Case 4, previously untested).
trait Rhs {
    type A: Bound = () // line
    where
        Self: Copy;

    type B: Bound /* inline */ = ()
    where
        Self: Copy;
}
