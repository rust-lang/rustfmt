// rustfmt-single_line_simple_if: true
// rustfmt-unstable_features: true
// rustfmt-single_line_if_else_max_width: 0

fn main() {
    let a = if 1 > 2 {
        unreachable!()
    } else {
        10
    };

    let a = if x {
        1
    } else if y {
        2
    } else {
        3
    };

    if true { continue }

    if true { continue }

    if width == 50_characters_or_shorter { continue }
    if width == 51_characters_long_and_above {
        return;
    }

    if name == super_duper_really_really_mega_ultra_giga_long_name_with_a_cherry_on_top {
        return;
    }

    if true {
        return;
    } else {
        break;
    }

    let x = if true {
        return;
    } else {
        break;
    };

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

    if cond() {
        statement();
    } else {
        other_statement();
    }
}
