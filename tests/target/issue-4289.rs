trait MyTrait
where
    Self: Clone, // comment on first constraint
    Self: Eq, // comment on last constraint
{
}

trait MyTrait2
where
    Self: Clone, /* another comment */
{
}
