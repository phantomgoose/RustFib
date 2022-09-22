use std::collections::HashMap;

fn main() {
    let mut dp = HashMap::new();
    let n = 21;
    println!("{}th Fibonacci number is {}", 10, get_fib(n, &mut dp));
}

fn get_fib(n: i32, dp: &mut HashMap<i32, i32>) -> i32 {
    match dp.get(&n) {
        Some(val) => *val,
        _ => {
            let res = match n {
                0..=1 => n,
                _ => get_fib(n - 2, dp) + get_fib(n - 1, dp),
            };

            dp.insert(n, res);
            res
        }
    }
}
