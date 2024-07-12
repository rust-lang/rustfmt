// rustfmt-single_line_if: true
// rustfmt-single_line_if_else_max_width: 0

fn main() {
    let x = if true { 1 } else { 2 };

    funk(if test() { 1 } else { 2 }, arg2);

    if true { 1 } else { 2 }

    if test() { 1 } else { 2 }

    if let Some(a) = b { return }

    if let Some(a) = b { do_something(); return }

    if let Some(a) = b { return } else { continue }

    let a = if 1 > 2 {
        unreachable!()
    } else {
        10
    };

    let a = if x { 1 } else if y { 2 } else { 3 };

    if true { continue }

    if true {
        continue
    }


    if width == is_49_characters____long { continue }
    if width == is_50_characters_____long { continue }
    if width == is_51_characters______long { continue }

    if name == super_duper_really_really_mega_ultra_giga_long_name_with_a_cherry_on_top { return }

    if true { return } else { break }

    let x = if true { return } else { break };

    let b = if cond() {
        5
    } else {
        // Brief comment.
        10
    };

    let c = if cond() {
        statement();

        5
    } else {
        10
    };

    if cond() { statement(); } else { other_statement(); }
}
