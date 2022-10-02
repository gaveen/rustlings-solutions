// iterators4.rs
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a hint.

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    (1..=num).fold(1, |acc, elem| acc * elem)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }


    // Additional tests
    #[test] 
    fn factorial_of_3() {
        assert_eq!(6, factorial(3));
    }

    #[test] 
    fn factorial_of_5() {
        assert_eq!(120, factorial(5));
    }

    #[test] 
    fn factorial_of_6() {
        assert_eq!(720, factorial(6));
    }

    #[test] 
    fn factorial_of_10() {
        assert_eq!(3_628_800, factorial(10));
    }
}
