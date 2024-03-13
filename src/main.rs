mod exercise1;

use exercise1::factorial;

fn main() {
    for n in 0..22 {
        println!("{}! = {}", n, factorial(n));
    }
}
