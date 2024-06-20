// rustfmt-nfc_normalize_idents: true
// rustfmt-format_macro_matchers: true
// Normalize identifiers to NFC.

// Accents in source file are U+0301 COMBINING ACUTE ACCENT,
// in target they are precomposed characters.

struct Foó {
    pub foó: Option<Box<Foó>>,
}

const FOÓ: Foó = Foó { foó: None };

macro_rules! foó {
    (foó $foó:ident) => {
        $foó
    };
}

fn foó<FOÓ: foó::Foó<Foó>>(foó: Foó) -> Foó {
    // FIXME: some macro invocations, like this one, don't get normalized
    let foó: Foó = foó!(foó foó);
    match foó {
        Foó { foó: foó } if foó == foó => *foó.unwrap(),
    }
}

mod foó {
    use super::Foó;

    trait Foó<Foó>: Foó<Foó>
    where
        Self: Foó<Foó>,
    {
        type Foó<Foó>: Foó<Foó>;

        const FOÓ: Foó;
    }
}
