// rustfmt-version: Two

#[derive(Clone, Debug, Default)]
pub struct ReactionGroup(
    pub(in crate::room::timeline)
        IndexMap<(Option<OwnedTransactionId>, Option<OwnedEventId>), OwnedUserId>,
);
