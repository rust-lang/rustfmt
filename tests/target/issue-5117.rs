// rustfmt-version: Two

fn main() {
    let Struct {
        tuplestruct: TupleStruct(
            Foo {
                message: "hi friend",
                array: [
                    aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,
                    aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,
                ],
            },
            aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,
        ),
        ..
    } = x;
}
