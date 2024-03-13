fn main() {
    match e { // this line still gets formatted
            MyStruct {
                field_a,
                .. // this comment here apparently causes trouble
            } => (),
    _ => (), // this line is no longer formatted
        };
}

fn main() {
    match e {                                           // this line still gets formatted
        MyStruct {
            field_a,
                field_b, // this should be aligned with field_a
                    field_c, // this should be aligned with field_a
                        } => (),
                        
        _ => (), // this line is no longer formatted
    };
}
