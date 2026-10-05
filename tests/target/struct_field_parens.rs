// rustfmt-style_edition: 2027
// rustfmt-max_width: 80

// 6642
struct S {
    f: (
        TypeIsTooLongToFitOnOneLineAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
    ),
    f: (/* */ u32 /* */),
    f: (/* */ u32),
    f: (u32 /* */),
    f: (
        /*comment makes line too long ---------------------------------*/
        MyType
    ),
    f: (/*this line just fits ééééééééééééééééééééééééééééééééééééééé*/ MyType),
    f: (
        MyType
        /*comment makes line too long ---------------------------------*/
    ),
    f: (
        // comment could fit, but would comment out the type
        u32
    ),
    f: (
        u32
        // comment could fit, but would comment out the `)`
    ),
    f: (
        /* this comment can be nestled */ MyType
        /* but this makes things too long */
    ),
    f: (
        /* this comment is far too long to be nestled-----------------------*/
        MyType /* this one isn't*/
    ),
    f: (/* multi-line
     * comment, the last line
     * can be nestled */ MyType),
    f: (MyType /* multi-line
     * comment, the first line
     * can be nestled */),
}
