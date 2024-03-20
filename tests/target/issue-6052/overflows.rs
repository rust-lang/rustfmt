// Passing the max-line boundary seems to be, combined with the other stuff, causing the panic
pub enum Dummy<
    SomeVeryLongStructDeclarationAsItemMakingTheLineOverflowTheRightHandSideAssignmentIsImportant
    = MyDefault,
> {}

pub enum Dummy2<
    SomeVeryLongStructDeclarationAsItemMakingTheLineOverflowTheRightHandSideAssignmentIsImport
    = MyDefault,
> {}

pub enum Dummy3<
    SomeVeryLongStructDeclarationAsItemMakingTheLineOverflowTheRightHandSideAssignmentIsImport
    =
    MyDefaultThatIsAlsoTooLongToBeFitIntoTheNextLineCausingATripleSplitThisMayBeOverdoingItOrNotIdk,
> {}

pub enum Dummy4<
    SomeVeryLongStructDeclarationAsItemMakingTheLineOverflowTheRightHandSideAssignmentIsImport
    = MyDefaultThatIsAlsoTooLongToBeFitIntoTheNextLineCausingATripleSplitThisMayBeOverdoingItOrNot,
> {}
