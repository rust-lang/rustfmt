fn fail() {
    {
        {
            if true {
                return Err(SomeError::OutOfMemory(OutOfMemory::new(
                    mem::size_of::<SomeType>()
                )));
            } else if true {
                return Err(SomeError::OutOfMemory(OutOfMemory::new(
                    core::mem::size_of::<SomeType>()
                )));
            } else {
                return Err(SomeError::OutOfMemory(OutOfMemory::new(
                    mem::size_of::<SomeType>(
                        aaaaaaaaaaaaaaaaaa,
                        bbbbbbbbbbbbbbbbbbbb,
                        cccccccccccccc,
                        dddddddddddddddddd,
                        eeeeeeeeeeeeeeeee,
                    ),
                )));
            }
        }
    }
}
