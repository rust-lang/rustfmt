// rustfmt-self_shorthand: true
struct Peach;

impl Peach {
    fn apple(self: Self) {}
    fn orange(mut self: Self) {}
    fn banana(self: &Self) {}
    fn lemon<'a>(self: &'a Self) {}
    fn pear<'a>(self: &'a mut Self) {}
    fn chaenomeles(self: &mut Self) {}
}

struct PeachWithComments;

impl PeachWithComments {
    fn apple(/* pre self */ self: /* pre Self */ Self /* post Self */) {}
    fn orange(/* pre mut */ mut /* post mut - pre self */ self: Self /* post Self */) {}
    fn banana(self: /* pre ref */ &Self /* post Self */) {}
    fn lemon<'a>(self: &/* post ref */'a Self) {}
    fn pear<'a>(self: &'a/* post lt - pre mut */ mut Self ) {}
    fn chaenomeles(self: &mut /* pre mut */ Self) {}
}

struct PeachWithMultiParams;

impl PeachWithMultiParams {
    fn apple(self: Self, a: String, b: String) {}
    fn orange(mut self: Self, a: String, b: String) {}
    fn banana(self: &Self, a: String, b: String) {}
    fn lemon<'a>(self: &'a Self, a: String, b: String) {}
    fn pear<'a>(self: &'a mut Self, a: String, b: String) {}
    fn chaenomeles(self: &mut Self, a: String, b: String) {}
}

// Example from https://doc.rust-lang.org/stable/reference/items/associated-items.html#methods
struct Example;
type Alias = Example;
trait Trait { type Output; }
impl Trait for Example { type Output = Example; }
impl Example {
    fn by_value(self: Self) {}
    fn by_ref(self: &Self) {}
    fn by_ref_mut(self: &mut Self) {}
    fn by_box(self: Box<Self>) {}
    fn by_rc(self: Rc<Self>) {}
    fn by_arc(self: Arc<Self>) {}
    fn by_pin(self: Pin<&Self>) {}
    fn explicit_type(self: Arc<Example>) {}
    fn with_lifetime<'a>(self: &'a Self) {}
    fn nested<'a>(self: &mut &'a Arc<Rc<Box<Alias>>>) {}
    fn via_projection(self: <Example as Trait>::Output) {}
}
