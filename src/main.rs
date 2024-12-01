mod code;

use code::d1::{get_product_difference, get_total_difference};

fn main() {
    let result1 = get_total_difference();
    println!("Total Differenect -> Result: {}\n", result1);

    let result2 = get_product_difference();
    println!("Total Product Difference -> Result: {}", result2);
}
