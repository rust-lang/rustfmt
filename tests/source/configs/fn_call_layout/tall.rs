// rustfmt-fn_call_layout:Tall

fn main() {
    empty_args();
    single_arg(ipsum);
    two_args(ipsum, dolor);

    lorem(ipsum, dolor, sit, amet);
    lorem(ipsum, // some inine comment
        dolor, sit, amet);
    lorem(ipsum, /* some inine comment */
        dolor, sit, amet);
    ipsum(dolor, sit, amet, consectetur, adipiscing, elit, vivamus, ipsum, orci, rhoncus, vel, imperdiet);

    // issue 2010
    let a = i8x32::new(
                0, 1, -1, 2,
                -2, 3, -3, 4,
                -4, 5, -5, std::i8::MAX,
                std::i8::MIN + 1, 100, -100, -32,
                0, 1, -1, 2,
                -2, 3, -3, 4,
                -4, 5, -5, std::i8::MAX,
                std::i8::MIN + 1, 100, -100, -32);

    // issue 4146
    return_monitor_err(
        e,
        channel_state,
        chan,
        order,
        commitment_update.is_some(),
        revoke_and_ack.is_some(),
    );

    // line comment in the middle of the args
    CastCheck::new(
        &fn_ctxt, e, from_ty, to_ty,
        // We won't show any error to the user, so we don't care what the span is here.
        DUMMY_SP, DUMMY_SP,
    );

    // other examples with more complex args
    more_complex_args(
        |a, b, c| {if a == 998765390 {- b * 3 } else  {c} },
        std::ops::Range { start: 3, end: 5 },
        std::i8::MAX, String::from(" hello world!!").as_bytes(),
        thread::Builder::new()
            .name("thread1".to_string())
            .spawn(move || {
                use std::sync::Arc;

                let mut values = Arc::<[u32]>::new_uninit_slice(3);

                // Deferred initialization:
                let data = Arc::get_mut(&mut values).unwrap();
                data[0].write(1);
                data[1].write(2);
                data[2].write(3);

                let values = unsafe { values.assume_init() };
            }), "another argument"
    );

    // nested calls
    ipsum(dolor(sit::amet(consectetur(aaaaaaaaaaaaaa, bbbbbbbbbbbb, ccccccccccccc, ddddddddddddd, eeeeeeeee))));

    ipsum(dolor(sit::amet(consectetur(aaaaaaaaaaaaaa, bbbbbbbbbbbb, ccccccccccccc, ddddddddddddd, adipiscing(elit(|| ipsum(dolor(sit::amet(consectetur())))))))));

    aaaaaaaaaaaaaaaaaa::bbbbbbbbbbbbbb::cccccccccc(ipsum(), dolor(sit::amet(consectetur, adipiscing), elit(vivamus::ipsum::orci(rhoncus()))));

    ipsum(dolor(sit::amet(consectetur(adipiscing(elit(vivamus::ipsum::orci(rhoncus())))))));
}
