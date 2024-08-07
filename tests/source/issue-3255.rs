fn foo(){  
    if true {      // Sample comment
                   // second-line comment
        1
    }
}


fn foo(){  
    if true {
        // Sample comment
        1
    }
}

fn foo(){  
    if true {    /* Sample comment */
        1
    }
}

fn foo(){  
    if true {     /* Sample
            comment */
        1
    }
}

// This isn't ideal.
fn foo(){
    if true {     /* Sample
                   * another line
                   * another line
                   * end
                   */
        1
    }
}
