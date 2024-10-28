/// Converts milliseconds to seconds as a `f64`.
///
/// # Arguments
/// * `ms` - The time in milliseconds to convert.
///
/// # Example
///
/// ```
/// use ms_to_secs::ms_to_secs;
///
/// let seconds = ms_to_secs(1500);
/// assert_eq!(seconds, 1.5);
/// ```
pub fn ms_to_secs(ms: u64) -> f64 {
    ms as f64 / 1000.0
}

#[cfg(test)]
mod tests {
    use super::ms_to_secs;

    #[test]
    fn test_ms_to_secs() {
        assert_eq!(ms_to_secs(1000), 1.0);
        assert_eq!(ms_to_secs(1500), 1.5);
        assert_eq!(ms_to_secs(0), 0.0);
    }
}
