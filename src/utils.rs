use std::fmt::Display;
use std::process::exit;
use std::time::Duration;

// This function has generic type T to bypass compiler's "type does not match" errors.
// Returning value does not matter, because it always terminates the process.
pub fn abort<T, M: Display>(msg: M) -> T {
    eprintln!("{}", msg);
    exit(1);
}

pub fn humanize_duration(duration: Duration) -> String {
    let secs = duration.as_secs() as f64 + f64::from(duration.subsec_nanos()) / 1_000_000_000.0;
    format!("{:.3}s", secs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_humanize_duration() {
        let duration = Duration::from_millis(12_345);
        assert_eq!(humanize_duration(duration), "12.345s");
    }
}
