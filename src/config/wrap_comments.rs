use std::fmt;

/// Which comments to wrap
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub enum WrapComments {
    /// Don't wrap comments
    Off,
    /// Wrap all kinds of comments
    All,
    /// Only wrap doc comments
    Doc,
    /// Only wrap normal comments
    Normal,
}

impl WrapComments {
    pub(crate) fn is_normal(self) -> bool {
        matches!(self, WrapComments::All | WrapComments::Normal)
    }

    pub(crate) fn is_doc(self) -> bool {
        matches!(self, WrapComments::All | WrapComments::Doc)
    }
}

impl fmt::Display for WrapComments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WrapComments::Off => f.write_str("Off"),
            WrapComments::All => f.write_str("All"),
            WrapComments::Doc => f.write_str("Doc"),
            WrapComments::Normal => f.write_str("Normal"),
        }
    }
}

impl std::str::FromStr for WrapComments {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "off" | "false" => Ok(WrapComments::Off),
            "all" | "true" => Ok(WrapComments::All),
            "doc" => Ok(WrapComments::Doc),
            "normal" => Ok(WrapComments::Normal),
            _ => {
                Err("Bad variant, expected one of: `Off`, `false`, `All`, `true`, `Doc`, `Normal`")
            }
        }
    }
}

impl<'de> serde::de::Deserialize<'de> for WrapComments {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        struct StringOrBoolVisitor;

        impl<'de> serde::de::Visitor<'de> for StringOrBoolVisitor {
            type Value = String;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("string")
            }

            fn visit_str<E>(self, value: &str) -> Result<String, E> {
                Ok(String::from(value))
            }

            fn visit_bool<E>(self, value: bool) -> Result<String, E> {
                Ok(value.to_string())
            }
        }

        let s = d.deserialize_string(StringOrBoolVisitor)?;
        s.parse().map_err(|_| {
            static ALLOWED: &'static [&str] = &["Off", "false", "All", "true", "Doc", "Normal"];
            D::Error::unknown_variant(&s, ALLOWED)
        })
    }
}
