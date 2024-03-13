pub fn factorial(n: i64) -> i64 {
    if n == 0 {
        return 1;
    }
    n * factorial(n - 1)
}
