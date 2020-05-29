macro_rules! abait {
    ($x:expr) => {
        Ok(())
    };
}

mod a {
    fn foo() -> Result<(), ()> {
        unsafe {
            (
                abait!(
                    proxy.load_account(ahc_client_end, TEST_ACCOUNT_ID.clone().as_mut().into())
                )?,
                (),
            )
        };
        Ok(())
    }
}
