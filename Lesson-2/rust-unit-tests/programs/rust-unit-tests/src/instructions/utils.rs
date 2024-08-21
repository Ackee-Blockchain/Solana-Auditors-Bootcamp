// Define a constant named MAGIC_NUMBER with a value of 254, and a type of u8.
const MAGIC_NUMBER: u8 = 254;

// Define a public function named `math_function` that takes an input of type u8 and returns a u8.
pub fn math_function(input: u8) -> u8 {
    // Calculate the divisor by subtracting the input from MAGIC_NUMBER.
    let divisor = MAGIC_NUMBER - input;
    // Return the result of dividing the input by the divisor.
    input / divisor
}

// Start a module for unit tests. The #[cfg(test)] attribute ensures this module is only compiled when testing.
#[cfg(test)]
mod tests {
    // Import all items from the parent module (where `math_function` is defined).
    use super::*;

    // Define a test case named `test_math`.
    #[test]
    fn test_math() {
        // Assert that the result of `math_function(10)` is equal to 0.
        assert_eq!(math_function(10), 0);
    }

    // Define a test case that is expected to panic. The #[should_panic] attribute indicates this.
    #[test]
    #[should_panic]
    fn test_math_general_panic() {
        // This should panic because the divisor would be zero (254 - 254 = 0).
        math_function(254);
    }

    // Define a test case that is expected to panic with a specific error message.
    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_math_expected_panic1() {
        // This should panic due to overflow (255 is greater than MAGIC_NUMBER).
        math_function(255);
    }

    // Define another test case expected to panic with a different specific error message.
    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_math_expected_panic2() {
        // This should panic due to division by zero (as divisor will be zero).
        math_function(254);
    }
}
