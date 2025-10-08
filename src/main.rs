use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: fibonacci-cli <n>");
        return;
    }

    let n: usize = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Input harus angka");
            return;
        }
    };

    let result = fibonacci(n);
    println!("Fibonacci({}) = {}", n, result);
}

fn fibonacci(n: usize) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
