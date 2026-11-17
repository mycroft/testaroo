/// Sum of all values.
pub fn sum(values: &[i32]) -> i32 {
    let mut total = 0;
    for v in values {
        total += v;
    }
    total
}

/// Arithmetic mean of the values.
pub fn average(values: &[i32]) -> i32 {
    sum(values) / values.len() as i32
}

/// `part` as a percentage of `whole`.
pub fn percent(part: i32, whole: i32) -> i32 {
    whole * 100 / part
}
