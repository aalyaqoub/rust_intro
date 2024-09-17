use clap::{Parser};

/// Find the nth fibonacci using recursion
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// The nth fibonacci
    #[arg(short)]
    n: u64,
}

fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n-1) + fibonacci(n-2);
    }
}

fn main() {
    let args:Args = Args::parse();

    let fibonacci_num:u64 = fibonacci(args.n);

    println!("The {}th fibonacci number is: {}", args.n, fibonacci_num);
}
