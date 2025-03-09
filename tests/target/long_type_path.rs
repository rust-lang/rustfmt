fn test() {
    let a: long_type_path::long_type_path::long_type_path::long_type_path::long_type_path::long_type_path::long_type_path::long_type_path::long_type_path::long_type_path::Long =
        Default::default();
}

fn test2() {
    let offenders = current_validators
        .into_iter()
        .enumerate()
        .filter_map(|(_, id)| {
            <<Runtime as pallet_im_online::Config>::ValidatorSet as ValidatorSetWithIdentification<sp_runtime::AccountId32>>::IdentificationOf::convert(id.clone())
                .map(|full_id| (id, full_id))
        })
        .collect::<Vec<IdentificationTuple<Runtime>>>();
}
