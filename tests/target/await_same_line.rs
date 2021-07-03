// rustfmt-await_same_line: true

fn main() {
    async_function().await??
        .field
        .async_function().await?
        .async_function_longgggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg()
        .await
        .field.await.await.await
        .function_with_args(
            arg_1, arg_2, arg_3, arg_4, arg_5, arg_6, arg_7, arg_8, arg_9,
        ).await
        .function_with_args(
            arg_1, arg_2, arg_3, arg_4, arg_5, arg_6, arg_7, arg_8, arg_9, arg_10,
        ).await;
}
