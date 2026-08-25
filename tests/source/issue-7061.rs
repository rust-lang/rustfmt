unsafe fn foo()  ->  i32  {  42  }

fn  main  ()  {
        'label:  loop  {
                break  'label  unsafe  {  foo()  }
        };
}

// A `loop` that cannot produce a value keeps the old behaviour
// redundant semicolon is removed.
fn no_value() {
    loop  {  break  };
    while  false  {  };
    for _ in 0..0  {  };
}

//minimal loop that produces a value keeps the semicolon
fn minimal() {
    loop  {  break  5  };
}