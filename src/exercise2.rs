use crate::read::get_i64;

fn fibonacci(n: i64) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

pub fn main() {
    println!("Enter -1 to exit\n");
    loop {
        // Prompt the user for n.
        let n = get_i64("N: ");

        // If n < 0, break out of the loop.
        if n < 0 {
            break;
        }

        // Calculate the Fibonacci number.
        println!("fibonacci({}) = {}\n", n, fibonacci(n));
    }
}
