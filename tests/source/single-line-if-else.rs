
// Format if-else expressions on a single line, when possible.

fn main() {
    let a = if 1 > 2 {
        unreachable!()
    } else {
        10
    };

    let a = if x { 1 } else if y { 2 } else { 3 };

    let b = if cond() {
        5
    } else {
        // Brief comment.
        10
    };

    let c = if cond() {
        statement();

        5
    } else {
        10
    };

    let d   = if  let  Some(val)  =  turbo 
    { "cool" } else {
     "beans" };

    if cond() { statement(); } else { other_statement(); }

    if true  {
        do_something()
    }

    let a = if x { 1 } else { 3 };

    // if may be formatted on a single line if it is "short"
    // and only contain a single expression
    if true { return }

    if true {
        return
    }

    if true { return; }

    if a { let y = 1; return y }

    for i in 0..2 { if g == true { continue } }

    let x = if veeeeeeeeery_loooooong_condition() { aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa } else { bbbbbbbbbb };
  
    let x = if veeeeeeeeery_loooooong_condition()     {    aaaaaaaaaaaaaaaaaaaaaaaaa }   else  {
        bbbbbbbbbb };

    funk(if test() {
             1
         } else {
             2
         },
         arg2);
}
