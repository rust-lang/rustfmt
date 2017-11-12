impl<'tcx> Const<'tcx> {
    pub fn from_constval<'a>() -> Const<'tcx> {
        let val =
            match *cv {
                ConstVal::Variant(_) | ConstVal::Aggregate(..) | ConstVal::Unevaluated(..) => bug!("MIR must not use `{:?}` (aggregates are expanded to MIR rvalues)", cv),
            };
    }
}
