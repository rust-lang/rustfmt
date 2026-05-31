// rustfmt-post_comment_alignment: SameIndent

fn foo(
    a: usize,          // Chirp
    b: usize, // Bark
    c: f32,  // Meow
) {
}

fn bar(
    a: usize,/* Chirp */
    b: usize,   // Bark
    c: f32,        /* Meow */
) {
}

trait EvalContextExtPrivate<'mir, 'tcx: 'mir>: crate::MiriEvalContextExt<'mir, 'tcx> {
    fn macos_stat_write_buf(
        &mut self,
        metadata: FileMetadata,
        buf_op: &OpTy<'tcx, Tag>,
    ) -> InterpResult<'tcx, i32> {
        let imms = [
            immty_from_uint_checked(access_nsec, long_layout)?, // st_atime_nsec
            immty_from_uint_checked(modified_sec, time_t_layout)?, // st_mtime
            immty_from_uint_checked(modified_nsec, long_layout)?, // st_mtime_nsec
            immty_from_uint_checked(0u128, time_t_layout)?,     // st_ctime
            immty_from_uint_checked(0u128, long_layout)?,       // st_ctime_nsec
            immty_from_uint_checked(created_sec, time_t_layout)?, // st_birthtime
            immty_from_uint_checked(created_nsec, long_layout)?, // st_birthtime_nsec
            immty_from_uint_checked(metadata.size, off_t_layout)?, // st_size
            immty_from_uint_checked(0u128, blkcnt_t_layout)?,   // st_blocks
        ];

        Ok(0)
    }
}
