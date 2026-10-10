pub enum ManagementFrameBody<
    'a,
    RateIterator = SupportedRatesTLVReadRateIterator<'a>,
    ExtendedRateIterator = ExtendedSupportedRatesTLVReadRateIterator<'a>,
    TLVIterator: IntoIterator<Item = IEEE80211TLV<'a, RateIterator, ExtendedRateIterator>> = TLVReadIterator<'a>,
    ActionFramePayload = &'a [u8],
>
{
    Action(ActionFrameBody<ActionFramePayload>),
    ActionNoAck(ActionFrameBody<ActionFramePayload>),
    Beacon(BeaconFrameBody<'a, RateIterator, ExtendedRateIterator, TLVIterator>),
    ATIM,
    Unknown {
        sub_type: ManagementFrameSubtype,
        body: &'a [u8],
    },
}
