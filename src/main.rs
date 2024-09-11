#[allow(unused_variables)]
mod numbers;
#[allow(unused_variables)]
mod variables;

fn main() {
    println!("Rust Tutorial Materials:");

    variables::variables();
    numbers::numbers();

    println!("Variables = success");
    println!("Numbers = success")
}
