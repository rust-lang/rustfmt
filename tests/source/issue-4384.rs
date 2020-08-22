fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4];

    v.iter()
        .filter(|    x| 
    // bla bla
             **x != 2)
        .for_each(|x| println!("{}", x));
}
