// rustfmt-normalize_comments:true

struct A {
    x: usize,
    // protected int[] observed;
    // (int, int)[] stack;
}

struct B {
    x: usize,
    // protected int[] observed;
    // some more details here
    // (int, int)[] stack;
}

struct C {
    x: usize,
    // protected int[] observed;
    // some more details here;
    // nested comment;
    //
    // (int, int)[] stack;
}
