// note: below comments are part of the test.
fn main() {
    let x = 42;
    //         ^ above line includes single space.
    let c = ||
    //        ^ above line includes single space.
    'c';
    let h = ||
    //        ^^^^ above line includes 3 spaces and 1 tab character.
    'h';
}
