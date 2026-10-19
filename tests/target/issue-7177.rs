//@ rustfmt-config: issue-7177.toml
//@ rustfmt-wrap_comments: true
//@ rustfmt-unstable_features: true

pub fn queue_user_requested_email_otp() {}
pub fn queue_user_requested_email_otp_abc() {}
pub struct QueueUserRequestedEmailOtp;

pub mod email {
    /// A. [`queue_user_requested_email_otp`](fn@super::queue_user_requested_email_otp), over the channel.
    pub fn a() {}

    /// B. [`queue_user_requested_email_otp`](struct@super::QueueUserRequestedEmailOtp), over the channel.
    pub fn b() {}

    /// C. [`queue_user_requested_email_otp`](super::queue_user_requested_email_otp_abc), over the channel.
    pub fn c() {}

    /// D. [`queue_user_requested_email_otp`](some/path/queue_user_requested_email_otp.html), over it.
    pub fn d() {}

    /// E. [`queue_user_requested_email_otp`](https://example.com/queue_user_requested_email), over it.
    pub fn e() {}
}
