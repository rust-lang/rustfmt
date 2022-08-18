struct EmptyBody<T>
    where T: Eq {
}

struct LineComment<T>
    where T: Eq {
    // body
}

struct BlockComment<T>
    where T: Eq {
    /* block comment */
}

struct HasBody<T>
    where T: Eq {
    x: T
}
