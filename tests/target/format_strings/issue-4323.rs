// rustfmt-format_strings: true

fn bench_contains_short_long(b: &mut Bencher) {
    let haystack =
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Suspendisse quis lorem sit amet \
         dolor ultricies condimentum. Praesent iaculis purus elit, ac malesuada quam malesuada \
         in. Duis sed orci eros. Suspendisse sit amet magna mollis, mollis nunc luctus, imperdiet \
         mi. Integer fringilla non sem ut lacinia. Fusce varius tortor a risus porttitor \
         hendrerit. Morbi mauris dui, ultricies nec tempus vel, gravida nec quam.

Nam lectus enim, dapibus non nisi tempor, consectetur convallis massa. Maecenas eleifend dictum \
         feugiat. Etiam quis mauris vel risus luctus mattis a a nunc. Nullam orci quam, imperdiet \
         id vehicula in, porttitor ut nibh. Duis sagittis adipiscing nisl vitae congue. Donec \
         mollis risus eu leo suscipit, varius porttitor nulla porta. Pellentesque ut sem nec nisi \
         euismod vehicula. Nulla malesuada sollicitudin quam eu fermentum.";
}

fn bench_contains_short_long(b: &mut Bencher) {
    let haystack = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Suspendisse quis \
                    lorem sit amet dolor ultricies condimentum. Praesent iaculis purus elit, ac \
                    malesuada quam malesuada in. Duis sed orci eros. Suspendisse sit amet magna \
                    mollis, mollis nunc luctus, imperdiet mi. Integer fringilla non sem ut \
                    lacinia. Fusce varius tortor a risus porttitor hendrerit. Morbi mauris dui, \
                    ultricies nec tempus vel, gravida nec quam.";
}

fn bench_contains_short_long(b: &mut Bencher) {
    let haystack = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Suspendisse quis \
                    lorem sit amet dolor ultricies condimentum. Praesent iaculis purus elit, ac \
                    malesuada quam malesuada in. Duis sed orci eros. Suspendisse sit amet magna \
                    mollis, mollis nunc luctus, imperdiet mi. Integer fringilla non sem ut \
                    lacinia. Fusce varius tortor a risus porttitor hendrerit. Morbi mauris dui, \
                    ultricies nec tempus vel, gravida nec quam.";
}
