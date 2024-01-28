// rustfmt-config: fn_width_100.toml

impl Trait {
    fn lorem(first: First, second: Second);
    fn lorem(first: FirstParameter, second: SecondParameter, third: ThirdParameter);
    fn lorem(
        first: FirstParameter,
        second: SecondParameter,
        third: ThirdParameter,
        fourth: FourthParameter,
    );

    fn lorem(first: First, second: Second) {
        // block
    }
    fn lorem(first: FirstParameter, second: SecondParameter, third: ThirdParameter) {
        // block
    }
    fn lorem(
        first: FirstParameter,
        second: SecondParameter,
        third: ThirdParameter,
        fourth: FourthParameter,
    ) {
        // block
    }
}
