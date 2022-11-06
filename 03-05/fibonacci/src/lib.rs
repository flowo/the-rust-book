use std::fmt;

/* #region    --- Errors */

#[derive(Debug)]
pub enum FibonacciError {
    InvalidInput,
}

impl FibonacciError {}

impl fmt::Display for FibonacciError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FibonacciError::InvalidInput => {
                write!(f, "Invalid input, number must be greater than 0")
            }
        }
    }
}

/* #endregion --- Errors */

/* #region    --- Implementation */

/// Returns the nth fibonacci number starting with 0.
///
/// # Arguments
///
/// * `nth` - A number indicating which fibonacci number to compute
///
/// # Examples
///
/// ```
/// let nth_fib = fibonacci::get(1).expect("Error when number < 1");
///
/// assert!(nth_fib == 0);
/// ```
///
/// ```
/// let nth_fib = fibonacci::get(2).expect("Error when number < 1");
///
/// assert!(nth_fib == 1);
/// ```
///
/// ```
/// let nth_fib = fibonacci::get(3).expect("Error when number < 1");
///
/// assert!(nth_fib == 1);
/// ```
///
/// ```
/// let nth_fib = fibonacci::get(4).expect("Error when number < 1");
///
/// assert!(nth_fib == 2);
/// ```
///
/// ```
/// let nth_fib = fibonacci::get(10).expect("Error when number < 1");
///
/// assert!(nth_fib == 34);
/// ```
pub fn get(nth: u32) -> Result<u32, FibonacciError> {
    if nth < 1 {
        return Err(FibonacciError::InvalidInput);
    }

    let nth_fibonacci = match nth {
        1 => 0,
        2 => 1,
        _ => get(nth - 1)? + get(nth - 2)?,
    };

    Ok(nth_fibonacci)
}

/* #endregion --- Implementation */

/* #region    --- Tests */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_computes_the_nth_fibonacci_number() {
        assert_eq!(get(1).unwrap(), 0, "1st fibonacci");
        assert_eq!(get(2).unwrap(), 1, "2nd fibonacci");
        assert_eq!(get(3).unwrap(), 1, "3rd fibonacci");
        assert_eq!(get(4).unwrap(), 2, "4th fibonacci");
        assert_eq!(get(5).unwrap(), 3, "5th fibonacci");
        assert_eq!(get(6).unwrap(), 5, "6th fibonacci");
        assert_eq!(get(7).unwrap(), 8, "7th fibonacci");
        assert_eq!(get(8).unwrap(), 13, "8th fibonacci");
        assert_eq!(get(9).unwrap(), 21, "9th fibonacci");
    }

    #[test]
    #[should_panic]
    fn it_panics_on_invalid_input() {
        assert_eq!(get(0).unwrap(), 0, "0th fibonacci");
    }
}
/* #endregion --- Tests */
