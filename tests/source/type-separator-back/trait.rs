// rustfmt-type_separator: Back

pub trait PrettyPrinter<'tcx>: Printer<'tcx, Error = fmt::Error, Path = Self, Region = Self, Type = Self, DynExistential = Self, Const = Self> + fmt::Write + Clone + Default
{
    //
}

pub trait Foo: Add + AddAssign + Clone + Copy + Debug + Default + Eq + Hash + Ord + PartialEq + PartialOrd + Sized {
    //
}

pub trait Bar: FromIterator<usize> + FromIterator<bool> + FromIterator<String> + FromIterator<Option<(u16, u16)>> {
    //
}

pub trait CommentsMultiline /*before colon*/: /*after colon*/ BA /*BA after*/ + /*BB before*/ BB /*BB after*/ + /*BC before*/ BC /*BC after*/ + /*BD before*/ BD /*BD after*/
{
    //
}

pub trait CommentsEOL: BA + /*BAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA*/ BB + /*BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB*/
   BC + /*BCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC*/ BD + /*BDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD*/ BE /*BEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE*/ {
   //
}

pub trait CommentsA: BA /*BAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA*/+/*BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB*/ BB
   + BC /*BAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA*/ + /*BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB*/ BB+ BC /* BAAAAAAA */ {
   //
}

pub trait CommentsB<T> where T: BA /*BAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA*/+/*BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB*/ BB
   + BC /*BAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA*/ + /*BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB*/ BB+ BC /* BAAAAAAA */ {
   //
}
