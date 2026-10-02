unsafe fn foo()  ->  i32  {  42  }

fn  main  ()  {
        'label:  loop  {
                break  'label  unsafe  {  foo()  }
        };
}

fn test_for_issue_5377() {
    loop {
        break false
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
fn minimal(){
    loop  {  break  5  };
}

//nested loop tests
fn nested_inner_break() {
    loop  {
        loop  {  break  5  };
        break
    };
}

fn nested_labeled_break() {
    'outer:  loop  {
        loop  {
            break  'outer  5
        }
    };
}

fn nested_deeply() {
    'outer:  loop  {
        loop  {
            loop  {
                break  'outer  unsafe  {  foo()  }
            }
        }
    };
}

fn nested_deeply_four_layers() {
    'outer:  loop  {
        loop  {
            loop  {
                loop  {
                    break  'outer  unsafe  {  foo()  }
                }
            }
        }
    };
}


fn break_inside_closure() {
    loop  {
        let f = ||  loop  {  break  5  };
        let _ = f();
        break
    };
}
