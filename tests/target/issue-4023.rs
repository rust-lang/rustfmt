// rustfmt-max_width: 100
// rustfmt-wrap_comments: true
// rustfmt-comment_width: 80
// rustfmt-normalize_comments: true
// rustfmt-unstable_features: true

/// [`MANAGE_MESSAGES`](../permissions/struct.Permissions.html#associatedconstant.MANAGE_MESSAGES)
/// [`MANAGE_CHANNEL`](./permissions/struct.Permissions.html#associatedconstant.MANAGE_CHANNEL)
struct Test {
    /// [`MANAGE_MESSAGES`]: ../permissions/struct.Permissions.html#associatedconstant.MANAGE_MESSAGES
    /// [`MANAGE_CHANNEL`]: ./permissions/struct.Permissions.html#associatedconstant.MANAGE_CHANNEL
    #[serde(default, rename = "rate_limit_per_user")]
    pub rate_limit: u16,
}
