/// Converts celsius to fahrenheit.
///
/// # Example
///
/// ```
/// let fahrenheit = c2f::convert_to_fahrenheit(0.);
///
/// assert_eq!(fahrenheit, 32.);
/// ```
pub fn convert_to_fahrenheit(value_in_celsius: f32) -> f32 {
    (9. / 5. * value_in_celsius) + 32.
}

/// Converts fahrenheit to celsius.
///
/// # Example
///
/// ```
/// let celsius = c2f::convert_to_celsius(32.);
///
/// assert_eq!(celsius, 0.);
/// ```
pub fn convert_to_celsius(value_in_fahrenheit: f32) -> f32 {
    (value_in_fahrenheit - 32.) * 5. / 9.
}

// Exercise

/* #region --- Test */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_to_fahrenheit() {
        assert_eq!(convert_to_fahrenheit(0.), 32., "freezing water");
        assert_eq!(convert_to_fahrenheit(36.5), 97.7, "healthy human");
        assert_eq!(convert_to_fahrenheit(39.1), 102.38, "sick human");
        assert_eq!(convert_to_fahrenheit(100.), 212., "boiling water");
    }

    #[test]
    fn it_converts_to_celsius() {
        assert_eq!(convert_to_celsius(32.), 0., "freezing water");
        assert_eq!(convert_to_celsius(97.7), 36.5, "healthy human");
        assert_eq!(convert_to_celsius(102.38), 39.1, "sick human");
        assert_eq!(convert_to_celsius(212.), 100., "boiling water");
    }
}
/* #endregion --- Test */
