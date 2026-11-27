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

fn nested_deeply_for_loop(){
     for x in 0..3 {
        for y in 0..3 {
            do_thing(x,y);
        };
    };
}

fn nested_deeply_while_loop() {
    while outer_cond() {
        while inner_cond() {
            do_thing()
        }
    };
}

// test cases for continue 
fn test_continue() {
    loop {
        continue
    };
}

fn labeled_continue() {
    'outer: loop {
        loop {
            continue 'outer
        }
    };
}

fn continue_and_break_with_value() {
    loop {
        if some_condn() {
            continue
        }
        break 5
    };
}

//test cases for return
fn test_return() {
    loop {
        return
    };
}

fn test_return_with_value() -> i32{
    loop {
        return 5
    };
}

fn return_and_break() -> i32{
    loop {
        if some_condn() {
            return 5
        }
        break 5
    };

    return 5
}

fn return_and_break_type2() -> i32{
    loop {
        if some_condn() {
            return 5
        }
        break 5
    }
}

