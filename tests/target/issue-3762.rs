// rustfmt-format_macro_bodies: true

extern crate toml;
#[macro_use]
extern crate serde_derive;

use toml::Value;

macro_rules! float_inf_tests {
    ($ty:ty) => {{
        #[derive(Serialize, Deserialize)]
        struct S {
            sf1: $ty,
            sf2: $ty,
            sf3: $ty,
            sf4: $ty,
            sf5: $ty,
            sf6: $ty,
            sf7: $ty,
            sf8: $ty,
        }
        let inf: S = toml::from_str(
            r"
        # infinity
        sf1 = inf  # positive infinity
        sf2 = +inf # positive infinity
        sf3 = -inf # negative infinity

        # not a number
        sf4 = nan  # actual sNaN/qNaN encoding is implementation specific
        sf5 = +nan # same as `nan`
        sf6 = -nan # valid, actual encoding is implementation specific

        # zero
        sf7 = +0.0
        sf8 = -0.0
        ",
        )
        .expect("Parse infinities.");

        assert!(inf.sf1.is_infinite());
        assert!(inf.sf1.is_sign_positive());
        assert!(inf.sf2.is_infinite());
        assert!(inf.sf2.is_sign_positive());
        assert!(inf.sf3.is_infinite());
        assert!(inf.sf3.is_sign_negative());

        assert!(inf.sf4.is_nan());
        assert!(inf.sf4.is_sign_positive());
        assert!(inf.sf5.is_nan());
        assert!(inf.sf5.is_sign_positive());
        assert!(inf.sf6.is_nan());
        assert!(inf.sf6.is_sign_negative());

        assert_eq!(inf.sf7, 0.0);
        assert!(inf.sf7.is_sign_positive());
        assert_eq!(inf.sf8, 0.0);
        assert!(inf.sf8.is_sign_negative());

        let s = toml::to_string(&inf).unwrap();
        assert_eq!(
            s,
            "\
sf1 = inf
sf2 = inf
sf3 = -inf
sf4 = nan
sf5 = nan
sf6 = -nan
sf7 = 0.0
sf8 = -0.0
"
        );

        toml::from_str::<Value>(&s).expect("roundtrip");
    }};
}

#[test]
fn float_inf() {
    float_inf_tests!(f32);
    float_inf_tests!(f64);
}
