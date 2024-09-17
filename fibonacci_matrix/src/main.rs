use clap::{Parser};
extern crate nalgebra as na;
use na::{SMatrix};

type Matrix2x2 = SMatrix<i32, 2, 2>;

/// Find the nth fibonacci using matrix math
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// The nth fibonacci
    #[arg(short)]
    n: u32,
}

fn main() {
    let args:Args = Args::parse();
    
    let matrix = Matrix2x2::new(1, 1,
                                1, 0);

    let fibonacci_matrix = matrix.pow(args.n);

    let fibonacci_num = fibonacci_matrix[(0,1)];

    println!("Fibonacci matrix:\n{}", fibonacci_matrix);
    println!("The {}th fibonacci number is: {}", args.n, fibonacci_num);
}