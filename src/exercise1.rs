fn factorial(n: i64) -> i64 {
    if n == 0 {
        return 1;
    }
    n * factorial(n - 1)
}

pub fn main() {
    for n in 0..22 {
        println!("{}! = {}", n, factorial(n));
    }
}
