use anyhow::Result;
use concurrency::Matrix;

fn main() -> Result<()> {
    let a = Matrix::new([1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3);
    let b = Matrix::new([1, 2, 3], 3, 1);

    let c = a * b;

    println!("{:?}", c);

    Ok(())
}
