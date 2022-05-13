// main.rs
cfg_if::cfg_if! {
    if #[cfg(not(feature = "client"))] {
        mod a;
        if #[cfg(feature = "server")] {
            if #[cfg(not(feature = "client"))] {
                if #[cfg(feature = "server")] {
                    if #[cfg(not(feature = "client"))] {
                        mod b;
                    } else {
                        mod c;
                    }
                    if #[cfg(feature = "server")] {
                        if #[cfg(not(feature = "client"))] {
                            if #[cfg(feature = "server")] {
                                mod d;
                            } else {
                                mod e;
                            }
                        }
                    }
                }
            }
        }
    }
}
