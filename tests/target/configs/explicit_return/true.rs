// rustfmt-explicit_return: true

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }

    return lhs % rhs == 0;
}
