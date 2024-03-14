fn main() {
    mymacro!(  (  for<'a> Fn() ) + Send + Sync);
}
