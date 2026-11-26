use crate::config::StyleEdition;

/// Defines the default value for the given style edition
#[allow(dead_code)]
pub trait StyleEditionDefault {
    type ConfigType;
    fn style_edition_default(style_edition: StyleEdition) -> Self::ConfigType;
}

/// macro to help implement `StyleEditionDefault` for config options
#[macro_export]
macro_rules! style_edition_default {
    // Handle the macro expansion when only the default `_` value is set.
    // The default value applies to all style editions.
    ($ty:ident, $config_ty:ty, _ => $default:expr) => {
        impl $crate::config::style_edition::StyleEditionDefault for $ty {
            type ConfigType = $config_ty;

            fn style_edition_default(_: $crate::config::StyleEdition) -> Self::ConfigType {
                $default
            }
        }
    };
    // Handle the macro expansion for `style_edition` 2024 and the default `_`.
    // The value set for `style_edition` 2024 applies to 2024 and later.
    // The default applies to `style_edition` 2021, 2018, 2015.
    ($ty:ident, $config_ty:ty, Edition2024 => $default_2024:expr, _ => $default_2015:expr) => {
        impl $crate::config::style_edition::StyleEditionDefault for $ty {
            type ConfigType = $config_ty;

            fn style_edition_default(
                style_edition: $crate::config::StyleEdition,
            ) -> Self::ConfigType {
                match style_edition {
                    $crate::config::StyleEdition::Edition2015
                    | $crate::config::StyleEdition::Edition2018
                    | $crate::config::StyleEdition::Edition2021 => $default_2015,
                    $crate::config::StyleEdition::Edition2024
                    | $crate::config::StyleEdition::Edition2027 => $default_2024,
                }
            }
        }
    };
    // Handle the macro expansion for `style_edition` 2027 and the default `_`.
    // The value set for `style_edition` 2027 applies to 2027 and later.
    // The default applies to `style_edition` 2024, 2021, 2018, 2015.
    ($ty:ident, $config_ty:ty, Edition2027 => $default_2027:expr, _ => $default_2015:expr) => {
        impl $crate::config::style_edition::StyleEditionDefault for $ty {
            type ConfigType = $config_ty;

            fn style_edition_default(
                style_edition: $crate::config::StyleEdition,
            ) -> Self::ConfigType {
                match style_edition {
                    $crate::config::StyleEdition::Edition2015
                    | $crate::config::StyleEdition::Edition2018
                    | $crate::config::StyleEdition::Edition2021
                    | $crate::config::StyleEdition::Edition2024 => $default_2015,
                    $crate::config::StyleEdition::Edition2027 => $default_2027,
                }
            }
        }
    };
    // Handle the macro expansion to explicity set `style_edition` 2027, 2024, and the fallback `_`.
    // The fallback applies to `style_edition` 2021, 2018, 2015.
    (
        $ty:ident,
        $config_ty:ty,
        Edition2027 => $default_2027:expr,
        Edition2024 => $default_2024:expr,
        _ => $default_2015:expr
    ) => {
        impl $crate::config::style_edition::StyleEditionDefault for $ty {
            type ConfigType = $config_ty;

            fn style_edition_default(
                style_edition: $crate::config::StyleEdition,
            ) -> Self::ConfigType {
                match style_edition {
                    $crate::config::StyleEdition::Edition2015
                    | $crate::config::StyleEdition::Edition2018
                    | $crate::config::StyleEdition::Edition2021 => $default_2015,
                    $crate::config::StyleEdition::Edition2024 => $default_2024,
                    $crate::config::StyleEdition::Edition2027 => $default_2027,
                }
            }
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::StyleEdition;

    #[test]
    fn test_impl_default_value_for_all_style_editions() {
        struct Unit;
        style_edition_default!(Unit, usize, _ => 100);

        // regardless of the style edition used the value will always return 100
        assert_eq!(Unit::style_edition_default(StyleEdition::Edition2015), 100);
        assert_eq!(Unit::style_edition_default(StyleEdition::Edition2018), 100);
        assert_eq!(Unit::style_edition_default(StyleEdition::Edition2021), 100);
        assert_eq!(Unit::style_edition_default(StyleEdition::Edition2024), 100);
        assert_eq!(Unit::style_edition_default(StyleEdition::Edition2027), 100);
    }

    #[test]
    fn test_impl_default_values_when_setting_style_edition_2024() {
        struct Unit;
        style_edition_default!(Unit, usize, Edition2024 => 50, _ => 100);

        // style edition 2015-2021 are all the same
        assert_eq!(Unit::style_edition_default(StyleEdition::Edition2015), 100);
        assert_eq!(Unit::style_edition_default(StyleEdition::Edition2018), 100);
        assert_eq!(Unit::style_edition_default(StyleEdition::Edition2021), 100);

        // Set style edition 2024, which is inherited by 2027
        assert_eq!(Unit::style_edition_default(StyleEdition::Edition2024), 50);
        assert_eq!(Unit::style_edition_default(StyleEdition::Edition2027), 50);
    }

    #[test]
    fn test_impl_default_values_when_setting_style_edition_2027() {
        struct Unit;
        style_edition_default!(Unit, usize, Edition2027 => 50, _ => 100);

        // style edition 2015-2024 are all the same
        assert_eq!(Unit::style_edition_default(StyleEdition::Edition2015), 100);
        assert_eq!(Unit::style_edition_default(StyleEdition::Edition2018), 100);
        assert_eq!(Unit::style_edition_default(StyleEdition::Edition2021), 100);
        assert_eq!(Unit::style_edition_default(StyleEdition::Edition2024), 100);

        // style edition 2027
        assert_eq!(Unit::style_edition_default(StyleEdition::Edition2027), 50);
    }

    #[test]
    fn test_impl_default_values_when_setting_style_edition_2024_2027() {
        struct Unit;
        style_edition_default!(Unit, usize, Edition2027 => 75, Edition2024 => 50, _ => 100);

        // style edition 2015-2021 are all the same
        assert_eq!(Unit::style_edition_default(StyleEdition::Edition2015), 100);
        assert_eq!(Unit::style_edition_default(StyleEdition::Edition2018), 100);
        assert_eq!(Unit::style_edition_default(StyleEdition::Edition2021), 100);

        // style edition 2024
        assert_eq!(Unit::style_edition_default(StyleEdition::Edition2024), 50);
        // style edition 2027
        assert_eq!(Unit::style_edition_default(StyleEdition::Edition2027), 75);
    }
}
