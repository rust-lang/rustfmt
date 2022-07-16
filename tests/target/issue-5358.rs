// Test /* comment */ inside trait generics gets duplicated.
trait Test</* comment */ T> {}

trait TestTwo</* comment */ T, /* comment */ V> {}
