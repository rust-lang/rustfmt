// rustfmt-post_comment_alignment: SingleSpace

use std::collections::{
    BTreeMap, // I am a TREE!!!!!!
    HashMap,
    HashSet, // I am a hash set!
};

fn foo(
    a: usize, // Chirp
    b: usize, // Bark
    c: f32, // Meow
) {
}

enum Animal {
    Cat, // THIS IS A BUTTERFLY!
    Dog, // THIS IS ME.
    Bird, /* OKOKOKOKOKOKOK */
}

fn bar(
    a: usize, /* Chirp */
    b: usize, // Bark
    c: f32, /* Meow */
    animal: Animal,
) -> &str {
    match animal {
        Animal::Cat => "meow", // Is this a bird?
        Animal::Dog => "bark", // Is this a cat?
        Animal::Bird => "chirp", // Is this a dog?
    }
}
