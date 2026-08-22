fn dummy() {
    let error_code = match err {
        msgs::DecodeError::UnknownVersion => 0x4000 | 1, // unknown realm byte
        msgs::DecodeError::UnknownRequiredFeature
        | msgs::DecodeError::InvalidValue
        | msgs::DecodeError::ShortRead => 0x4000 | 22,   // invalid_onion
        _ => 0x2000 | 2,                                 // Should never happen
    };
}
