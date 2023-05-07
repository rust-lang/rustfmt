// rustfmt-version: Two

// Original - return-type and empty "where" - no comments
mod inner {
    fn foo1() -> String {
        String::new()
    }
}

mod inner {
    fn foo2() -> String {
        String::new()
    }
}

// Return-type with no "where" - one comment
fn main() {
    fn foo1() -> String /* same-line with ret-type and brace comment */ {}
}

fn main() {
    fn foo2() -> String /* same-line with ret-type comment - brace in new-line */ {}
}

fn main() {
    fn foo3() -> String
    /* new-line comment - brace in new-line */
    {
    }
}

fn main() {
    fn foo4() -> String // same-line with ret-type comment
    {
    }
}

// Return-type and empty "where" - one comment
fn main() {
    fn foo1() -> String /* same-line with ret-type/where/brace brace pre-where comment */ {}
}

fn main() {
    fn foo2() -> String /* same-line with ret-type/where pre-where comment - brace in new-line */ {}
}

fn main() {
    fn foo3() -> String /* same-line with ret-type pre-where comment - where in new-line */ {}
}

fn main() {
    fn foo4() -> String
    /* new-line pre-where comment - where in new-line */
    {
    }
}

fn main() {
    fn foo5() -> String // same-line with ret-type pre-where comment
    {
    }
}

fn main() {
    fn foo6() -> String
    // new-line pre-where comment
    {
    }
}

// Return-type and empty "where" - two inline comments
fn main() {
    fn foo1() -> String /* pre-where same-line */ /* post-where same line */ {}
}

fn main() {
    fn foo2() -> String
    /* pre-where new-line */ /* post-where same line */ {
    }
}

fn main() {
    fn foo3() -> String /* pre-where same with ret - where in new */
    /* post-where same line */ {
    }
}

fn main() {
    fn foo4() -> String /* pre-where same with ret - where in new */
    /* post-where new line - brace in same */ {
    }
}

fn main() {
    fn foo5() -> String
    /* pre-where new line - where in new */
    /* post-where new line - brace in same */ {
    }
}

fn main() {
    fn foo6() -> String
    /* pre-where new line - where in new */
    /* post-where new line - brace in new */
    {
    }
}

// Return-type and empty "where" - two one-line comments
fn main() {
    fn foo1() -> String // pre-where same with ret - where in new
    // post-where same with where - brace in new
    {
    }
}

fn main() {
    fn foo2() -> String
    // pre-where new line
    // post-where same with where - brace in new
    {
    }
}

// Return-type and empty "where" - more comments
fn main() {
    fn foo<F>(foo2: F) -> String
    /* pre-where same with ret - where in new */
    where
        /* post-where same with where - following in new" */
        F: Fn(), /* comment after where declaration */
    {
    }
}
