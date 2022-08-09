pub fn factorial(x: i64) -> i64 {
    if x == 1 {
        return 1
    } else if x == 0 {
        return 1
    } else if x < 0 {
        return -1
    }

    return x * factorial(x - 1)
}