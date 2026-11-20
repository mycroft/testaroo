use crate::stats;

/// How far along a task is, as a percentage of its steps.
pub fn progress(done: i32, total: i32) -> i32 {
    stats::percent(total, done)
}
