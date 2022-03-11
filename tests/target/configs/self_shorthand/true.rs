// rustfmt-self_shorthand: true
struct Peach;

impl Peach {
    fn apple(self) {}
    fn orange(mut self) {}
    fn banana(&self) {}
    fn lemon<'a>(&'a self) {}
    fn pear<'a>(&'a mut self) {}
    fn chaenomeles(&mut self) {}
}

struct PeachWithComments;

impl PeachWithComments {
    fn apple(/* pre self */ /* pre Self */ self /* post Self */) {}
    fn orange(/* pre mut */ mut /* post mut - pre self */ self /* post Self */) {}
    fn banana(/* pre ref */ &self /* post Self */) {}
    fn lemon<'a>(& /* post ref */ 'a self) {}
    fn pear<'a>(&'a /* post lt - pre mut */ mut self) {}
    fn chaenomeles(&mut /* pre mut */ self) {}
}

struct PeachWithMultiParams;

impl PeachWithMultiParams {
    fn apple(self, a: String, b: String) {}
    fn orange(mut self, a: String, b: String) {}
    fn banana(&self, a: String, b: String) {}
    fn lemon<'a>(&'a self, a: String, b: String) {}
    fn pear<'a>(&'a mut self, a: String, b: String) {}
    fn chaenomeles(&mut self, a: String, b: String) {}
}

// Example from https://doc.rust-lang.org/stable/reference/items/associated-items.html#methods
struct Example;
type Alias = Example;
trait Trait {
    type Output;
}
impl Trait for Example {
    type Output = Example;
}
impl Example {
    fn by_value(self) {}
    fn by_ref(&self) {}
    fn by_ref_mut(&mut self) {}
    fn by_box(self: Box<Self>) {}
    fn by_rc(self: Rc<Self>) {}
    fn by_arc(self: Arc<Self>) {}
    fn by_pin(self: Pin<&Self>) {}
    fn explicit_type(self: Arc<Example>) {}
    fn with_lifetime<'a>(&'a self) {}
    fn nested<'a>(self: &mut &'a Arc<Rc<Box<Alias>>>) {}
    fn via_projection(self: <Example as Trait>::Output) {}
}
