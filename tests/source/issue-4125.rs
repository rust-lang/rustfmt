// rustfmt-max_width: 60

fn main() {
    match (pat, num) {
        (PatternOne, 1) | (RoomForBlockOnSameLine, 10) => {}

        (PatternOne, 1) | (PatternStretchedToBounds, 2) => {}

        UnsplitableVeryLongArmPatternWithRoomForBlock0 => {}

        UnsplitableVeryLongArmPatternWithBraceAtEndOfLn => {}

        UnsplitableVeryLongArmPatternWithArrowAtColnWidth => {}

        UnsplitableVeryLongArmPatternWithArrowPastColnWidth => {}

        UnsplitableVeryLongArmPatternGoingBeyondMaxColumnWidth => {}
    }
}
