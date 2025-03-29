use ndarray::array;
use rand::Rng;


fn main() {
    // Define matrices L, R, O
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

    // Generate random values using u128 to prevent overflow
    let mut rng = rand::thread_rng();
    let x: u128 = rng.gen_range(1..=1000);
    let y: u128 = rng.gen_range(1..=1000);
    let z: u128 = rng.gen_range(1..=1000);
    let u: u128 = rng.gen_range(1..=1000);

    // Compute the algebraic circuit
    let r_value: u128 = x * y * z * u;
    let v1: u128 = x * y;
    let v2: u128 = z * u;

    // Create witness vector
    let a = array![1, r_value, x, y, z, u, v1, v2];

    // Perform element-wise multiplication check
    let lhs = o.dot(&a);
    let rhs = l.dot(&a) * r.dot(&a);

    // Assert that the constraints hold
    assert!(lhs == rhs, "System contains an inequality");

    println!("Verification successful!");
}
