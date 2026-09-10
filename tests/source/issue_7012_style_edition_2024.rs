// rustfmt-style_edition: 2024

fn test() {
    if outer {
        if inner {
            if even_more_inner {
                let items = page
                    .into_iter()
                    .map(
                        |list_media_response::ListEntry {
                             cdn,
                             media_id,
                             length,
                         }|
                         -> Result<_, RequestError<BackupAuthCredentialRejected>> {
                            Ok(ListMediaItem {
                                cdn,
                                media_id,
                                object_length: length,
                            })
                        },
                    )
                      .try_collect()?;
            }
        }
    }
}
