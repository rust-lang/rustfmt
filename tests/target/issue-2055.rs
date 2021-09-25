pub trait CommentAfterColon1: /* Comment */ Clone {
    //
}

pub trait CommentAfterColon2: // Comment
    Clone
{
    //
}

pub trait CommentBeforeColon /*comment*/ : Clone {
    //
}

pub trait CommentsShort /*after ident*/ : /*BA before*/
    BA /*BA after*/ + /*BB before*/ BB /*BB after*/
{
    //
}

pub trait CommentsMultiline /*after ident*/ : /*BA before*/
    BA /*BA after*/
    + /*BB before*/ BB /*BB after*/
    + /*BC before*/ BC /*BC after*/
    + /*BD before*/ BD /*BD after*/
{
    //
}

pub trait CommentsShortWhere<T1, T2>
where
    T1: BA /*comment 1*/ + /*comment 2*/ BB + BC, /*comment 3*/
    T2: BA /*comment 4*/ + /*comment 5*/ BB + BC, /*comment 6*/
{
    //
}

pub trait CommentsWhere<T1, T2>
/*before where*/
where
    T1: /*comment 1*/
        BA /*comment 2*/
            + /*comment 3*/ BB /*comment 4*/
            + /*comment 5*/ BC /*comment 6*/
            + /*comment 7*/ BB /*comment 8*/
            + /*comment 9*/ BC, /*comment 10*/
    T2: , /*comment 11*/
{
    //
}

pub trait KitchenSink<T1, T2> /*before colon*/ : //after colon
    FromIterator<char> /*comment 1*/
    + /*comment 2*/ Printer<
        'tcx,
        Error = fmt::Error,
        Path = Self,
        Region = Self,
        Type = Self,
        DynExistential = Self,
        Const = Self,
    > /*comment 3*/ + /*comment 4*/ fmt::Write /*comment 5*/
    + /*comment 6*/ Clone /*comment 7*/
    + /*comment 8*/ Default /*comment 9*/
where
    T1: /*comment 10*/
        BA /*comment 11*/
            + /*comment 12*/ BB
            + BC /*comment 13*/
            + /*comment 14*/ BB
            + /*comment 15*/ BC, /*comment 16*/
    T2: /*comment 17*/
        BA /*comment 18*/
            + /*comment 19*/ BB
            + BC /*comment 20*/
            + /*comment 21*/ BB
            + /*comment 22*/ BC, /*comment 23*/
{
    //
}
