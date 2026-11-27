// rustfmt-error_on_line_overflow: true

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    EnumString,
    EnumIter,
    strum_macros::Display,
    PartialEq,
    Eq,
)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Create,
    Update,
    Delete,
    Undefined,
}
