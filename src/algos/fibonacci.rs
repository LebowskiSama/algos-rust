pub fn fibonacci(x:i64) -> i64 {
    // base case
    if x == 0 {
        return 0
    }else if x == 1 {
        return 1
    }

    return fibonacci(x - 1) + fibonacci(x - 2)
}