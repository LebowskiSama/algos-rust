pub mod algos;

# [cfg(test)]
mod tests {
    # [test]
    fn test_fibonacci() {
        use super::algos::fibonacci::fibonacci;
        assert_eq!(fibonacci(26), 121_393)
    }
    # [test]
    fn test_factorial() {
        use super::algos::factorial::factorial;
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(-5), -1);
    }
    # [test]
    fn test_karatsuba() {
        use super::algos::karatsuba::karatsuba;
        
        assert_eq!(karatsuba(20, 20), 20 * 20);
        assert_eq!(karatsuba(123, 45), 123 * 45);
        assert_eq!(karatsuba(925, 123456), 925 * 123456);
    }
}