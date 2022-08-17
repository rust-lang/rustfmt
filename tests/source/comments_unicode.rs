impl Default for WhitespaceCharacters {
    fn default() -> Self {
        Self {
            space: '·',    // U+00B7
            nbsp: '⍽',    // U+237D
            tab: '→',     // U+2192
            newline: '⏎', // U+23CE
        }
    }
}
