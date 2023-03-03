// rustfmt-max_width: 120

fn check_copy_clone<'tcx>(cx: &LateContext<'tcx>, item: &Item<'_>, trait_ref: &hir::TraitRef<'_>, ty: Ty<'tcx>) {
    let clone_id = match cx.tcx.lang_items().clone_trait() {
        Some(id) if trait_ref.trait_def_id() == Some(id) => id,
        _ => return,
    };
    let Some(copy_id) = cx.tcx.lang_items().copy_trait() else { return };
    let (ty_adt, ty_subs) = match *ty.kind() {
        // Unions can't derive clone.
        ty::Adt(adt, subs) if !adt.is_union() => (adt, subs),
        _ => return,
    };
    // If the current self type doesn't implement Copy (due to generic constraints), search to see if
    // there's a Copy impl for any instance of the adt.
    if !is_copy(cx, ty) {
        if ty_subs.non_erasable_generics().next().is_some() {
            let has_copy_impl = cx.tcx.all_local_trait_impls(()).get(&copy_id).map_or(false, |impls| {
                impls
                    .iter()
                    .any(|&id| matches!(cx.tcx.type_of(id).subst_identity().kind(), ty::Adt(adt, _) if ty_adt.did() == adt.did()))
            });
            if !has_copy_impl {
                return;
            }
        } else {
            return;
        }
    }
    // Derive constrains all generic types to requiring Clone. Check if any type is not constrained for
    // this impl.
    if ty_subs.types().any(|ty| !implements_trait(cx, ty, clone_id, &[])) {
        return;
    }
    // `#[repr(packed)]` structs with type/const parameters can't derive `Clone`.
    // https://github.com/rust-lang/rust-clippy/issues/10188
    if ty_adt.repr().packed()
        && ty_subs
            .iter()
            .any(|arg| matches!(arg.unpack(), GenericArgKind::Type(_) | GenericArgKind::Const(_)))
    {
        return;
    }

    span_lint_and_note(
        cx,
        EXPL_IMPL_CLONE_ON_COPY,
        item.span,
        "you are implementing `Clone` explicitly on a `Copy` type",
        Some(item.span),
        "consider deriving `Clone` or removing `Copy`",
    );
}
