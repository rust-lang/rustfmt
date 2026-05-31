impl Default for WhitespaceCharacters {
    fn default() -> Self {
        Self {
            space: '·', // U+00B7
            nbsp: '⍽', // U+237D
            tab: '→', // U+2192
            newline: '⏎', // U+23CE
        }
    }
}

const RAINBOWS: &[&str] = &[
    "rаinЬοѡ", // hue: 0
    "raіnЬοw", // hue: 2
    "rаіɴЬow", // hue: 2
    "raіɴЬoѡ", // hue: 8
    "ʀainЬow", // hue: 8
    "ʀaіɴboѡ", // hue: 8
    "ʀаіnbοw", // hue: 11
    "rainЬoѡ", // hue: 14
    "raіɴbow", // hue: 14
    "rаiɴЬow", // hue: 20
    "raіnЬow", // hue: 26
    "ʀaiɴbοw", // hue: 32
    "raіɴboѡ", // hue: 35
    "rаiɴbow", // hue: 35
    "rаіnbοw", // hue: 38
    "rаinЬow", // hue: 47
    "ʀaіnboѡ", // hue: 47
    "ʀaіnЬoѡ", // hue: 47
    "ʀаіɴbοw", // hue: 53
    "ʀaіnЬοѡ", // hue: 57
    "raiɴЬoѡ", // hue: 68
    "ʀainbοѡ", // hue: 68
    "ʀаinboѡ", // hue: 68
    "ʀаiɴbοw", // hue: 68
    "ʀаіnbow", // hue: 68
    "rаіnЬοѡ", // hue: 69
    "ʀainЬοw", // hue: 71
    "raiɴbow", // hue: 73
    "raіnЬoѡ", // hue: 74
    "rаіɴbοw", // hue: 77
    "raіnЬοѡ", // hue: 81
    "raiɴЬow", // hue: 83
    "ʀainbοw", // hue: 83
    "ʀаinbow", // hue: 83
    "ʀаiɴbοѡ", // hue: 83
    "ʀаіnboѡ", // hue: 83
    "ʀаіɴЬοѡ", // hue: 84
    "rainЬow", // hue: 85
    "ʀаiɴЬοw", // hue: 86
    "ʀаіnbοѡ", // hue: 89
    "ʀаіnЬοw", // hue: 92
    "rаiɴbοw", // hue: 95
    "ʀаіɴbοѡ", // hue: 98
    "ʀаiɴЬοѡ", // hue: 99
    "raіnbοw", // hue: 101
    "ʀаіɴЬοw", // hue: 101
    "ʀaiɴboѡ", // hue: 104
    "ʀаinbοѡ", // hue: 104
    "rаiɴbοѡ", // hue: 107
    "ʀаinЬοw", // hue: 107
    "rаiɴЬοw", // hue: 110
    "rаіnboѡ", // hue: 110
    "rаіnbοѡ", // hue: 113
    "ʀainЬοѡ", // hue: 114
    "rаіnЬοw", // hue: 116
    "ʀaіɴЬow", // hue: 116
    "rаinbοw", // hue: 122
    "ʀаіɴboѡ", // hue: 125
    "rаinbοѡ", // hue: 131
    "rainbow", // hue: 134
    "rаinЬοw", // hue: 134
    "ʀаiɴboѡ", // hue: 140
    "rainЬοѡ", // hue: 141
    "raіɴЬow", // hue: 143
    "ʀainЬoѡ", // hue: 143
    "ʀaіɴbow", // hue: 143
    "ʀainbow", // hue: 148
    "rаіɴboѡ", // hue: 149
    "ʀainboѡ", // hue: 155
    "ʀaіnbow", // hue: 155
    "ʀaіnЬow", // hue: 155
    "raiɴbοw", // hue: 158
    "ʀаiɴЬoѡ", // hue: 158
    "rainbοw", // hue: 160
    "rаinbow", // hue: 160
    "ʀaіɴbοѡ", // hue: 164
    "ʀаiɴbow", // hue: 164
    "ʀаіnЬoѡ", // hue: 164
    "ʀaiɴЬοѡ", // hue: 165
    "rаiɴboѡ", // hue: 167
    "ʀaіɴЬοw", // hue: 167
    "ʀaіɴЬοѡ", // hue: 171
    "raіnboѡ", // hue: 173
    "ʀаіɴЬoѡ", // hue: 173
    "rаіɴbοѡ", // hue: 176
    "ʀаinЬow", // hue: 176
    "rаiɴЬοѡ", // hue: 177
    "rаіɴЬοw", // hue: 179
    "ʀаinЬoѡ", // hue: 179
    "ʀаіɴbow", // hue: 179
    "rаiɴЬoѡ", // hue: 182
    "raіɴbοѡ", // hue: 188
    "rаіnЬoѡ", // hue: 188
    "raiɴЬοѡ", // hue: 189
    "raіɴЬοw", // hue: 191
    "ʀaіɴbοw", // hue: 191
    "ʀаіnЬow", // hue: 191
    "rainbοѡ", // hue: 194
    "rаinboѡ", // hue: 194
    "rаіnbow", // hue: 194
    "rainЬοw", // hue: 197
    "rаinЬoѡ", // hue: 206
    "rаіɴbow", // hue: 206
    "rаіɴЬοѡ", // hue: 210
    "ʀaiɴЬow", // hue: 212
    "raіɴbοw", // hue: 218
    "rаіnЬow", // hue: 218
    "ʀaiɴbοѡ", // hue: 221
    "ʀaiɴЬοw", // hue: 224
    "ʀaіnbοѡ", // hue: 227
    "raiɴboѡ", // hue: 230
    "ʀaіnbοw", // hue: 230
    "ʀaіnЬοw", // hue: 230
    "ʀаinЬοѡ", // hue: 231
    "rainboѡ", // hue: 232
    "raіnbow", // hue: 232
    "ʀаіɴЬow", // hue: 233
    "ʀaіɴЬoѡ", // hue: 239
    "ʀаіnЬοѡ", // hue: 246
    "raiɴbοѡ", // hue: 248
    "ʀаiɴЬow", // hue: 248
    "raіɴЬοѡ", // hue: 249
    "raiɴЬοw", // hue: 251
    "rаіɴЬoѡ", // hue: 251
    "ʀaiɴbow", // hue: 251
    "ʀаinbοw", // hue: 251
    "raіnbοѡ", // hue: 254
];
