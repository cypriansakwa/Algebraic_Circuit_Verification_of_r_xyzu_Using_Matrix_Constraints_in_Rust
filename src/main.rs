use ndarray::{array}; // Removed unused imports
use rand::{rng, Rng};

fn generate_random_value() -> u128 {
    let mut rng = rng(); // Updated from thread_rng()
    rng.random_range(1..100) // Updated from gen_range()
}

fn main() {
    let l = array![
        [0, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 1, 0]
    ];

    let r = array![
        [0, 0, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 1]
    ];

    let o = array![
        [0, 0, 0, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 0, 0, 0, 1],
        [0, 1, 0, 0, 0, 0, 0, 0]
    ];

    let x = generate_random_value();
    let y = generate_random_value();
    let z = generate_random_value();
    let u = generate_random_value();

    println!("Generated values: x = {}, y = {}, z = {}, u = {}", x, y, z, u);

    let r_value = x.checked_mul(y).unwrap()
                    .checked_mul(z).unwrap()
                    .checked_mul(u).unwrap();
    let v1 = x * y;
    let v2 = z * u;

    let a = array![1, r_value, x, y, z, u, v1, v2];

    let left_result = l.dot(&a);
    let right_result = r.dot(&a);
    let output_result = o.dot(&a);

    let elementwise_product = &left_result * &right_result;

    assert!(
        output_result == elementwise_product,
        "System contains an inequality!"
    );

    println!("Verification successful: The equation holds.");
}
