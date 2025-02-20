// rustfmt-version: Two

trait AllTheOthers1: Trait3 + Trait4 + Trait5 + Trait6 + Trait7 + Trait8 + Trait9 + Trait10 + Trait11 + Trait12 + Trait14
{
}

trait AllTheOthers2: Trait4 + Trait5 + Trait6 + Trait7 + Trait8 + Trait9 + Trait10 + Trait11 + Trait12 + Trait13 + Trait15
{
}

mod Module {
    trait AllTheOthers3: Trait5 + Trait6 + Trait7 + Trait8 + Trait9 + Trait10 + Trait11 + Trait12 + Trait13 + Trait14
    {
    }

    trait AllTheOthers4: Trait6 + Trait7 + Trait8 + Trait9 + Trait10 + Trait11 + Trait12 + Trait13 + Trait14 + Trait15
    {
    }
}
