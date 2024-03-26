// rustfmt-wrap_comments: true

// Ensure comments preserved no matter what
fn main() {
    let (/*
        */ () /*
        */ | /**/ () /**/) = (); //
    let (/*
            */ () /*
            */ | /**/ () /**/) = (); //
    let (//
        () //
        | () //
    ) = (); //

        let (/*comment*/
        (

        ) 


        |      (

        )
    /*comment*/) = ();
            
    enum Foo {
        A,
        B,
        C,
        Bar,
    } //FF
    let x = Foo::A;
    match x {
        Foo::
        Bar => { () },
        Foo::
        A
        // FIXME lol
        | Foo::
        B//
        | Foo::
        C => { () }//
      }
    let (/*aaa
        aaaa*/ () /*
        */ | /**/ () /**/) = (/* */);

    let (/*aaa
        aaaa*/ () /*
        */ | /**/ () /**/) = (/* */); //

    let (/**/ () /**/ | /**/ () /**/) = (); //
}
