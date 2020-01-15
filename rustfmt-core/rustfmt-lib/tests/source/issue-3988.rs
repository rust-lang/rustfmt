// rustfmt-normalize_doc_attributes: true
// rustfmt-max_width: 10

mod a {
  mod b {
      #[doc = "test"]fn foo() -> () { 
      }
  }
}
