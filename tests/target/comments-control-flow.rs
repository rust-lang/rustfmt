// Issue #4549

fn main() {
    if /*w*/ 5 /*x*/ == /*y*/ 6 /*z*/ {}
}

fn main() {
    /*pre-if*/
    if /*post-if*/ 5 /*x*/ == /*y*/ 6 /*pre-if-block*/ {
        x = y + 7;
    }
    /*pre-elseif*/
    else if /*post-elseif*/ z == u /*pre-elseif-block*/ {
        y = 5;
    }
    /*pre-else*/
    else /*post-else*/ {
        z = x;
    }
}

fn main() {
    /*pre-if*/
    if /*post-if*/ 5 /*x*/ == /*y*/ 6 /*pre-if-block*/ {
        x = y + 7;
    }
    /*pre-elseif*/
    else if /*post-elseif Longgggggggggggggggggggggggggggggggggggggggggggggggggggggggggg*/ z == u
    /*pre-elseif-block*/ {
        y = 5;
    }
    /*pre-else*/
    else /*post-else*/ {
        z = x;
    }
}

fn main() {
    /*pre-if*/
    if // post-if - make sure that x and y states are the same
    5 /*x*/ == /*y*/ 6 //pre-if-block - when they are same change y to next state
    {
        x = y + 7;
    }
}

fn main() {
    /*pre-if*/
    if // post-if - make sure that x and y states are the sameeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee
    5 /*x*/ == /*y*/ 6
    //pre-if-block - when they are same change y to next stateeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee
    {
        x = y + 7;
    }
}

fn main() {
    if 5 == 6 {
        x = y + 7;
    } else if z == u {
        y = 5;
    } else {
        z = x;
    }
}

fn main() {
    if // if comment
    5 == 6 {
        x = y + 7;
    }
}
fn main() {
    if
    // if comment
    5 == 6 {
        x = y + 7;
    }
}
