# Algebraic Circuit Verification of r = xyzu Using Matrix Constraints in Rust

This project implements an algebraic circuit verification using matrix constraints in Rust. The circuit computes the product `r = x * y * z * u` and verifies it using linear algebra operations on matrices.

## Features

- Uses **ndarray** for matrix operations.
- Uses **rand** for generating random values.
- Implements matrix constraints to verify the correctness of the computation.
- Prevents integer overflow by using `u128` for large multiplications.

## Installation

Ensure you have Rust installed. If not, install it from [Rust's official website](https://www.rust-lang.org/).

Clone the repository:

```sh
git clone https://github.com/cypriansakwa/Algebraic_Circuit_Verification_of_r_xyzu_Using_Matrix_Constraints_in_Rust.git
cd Algebraic_Circuit_Verification_of_r_xyzu_Using_Matrix_Constraints_in_Rust
```
## Usage
Build and run the project:
```sh
cargo run
```
## Dependencies
The project uses the following Rust crates:
- `ndarray` for matrix operations.
- `rand` for random number generation.

To add dependencies manually, run:
```sh
cargo add ndarray rand
```
## Explanation
- Defines matrices `L`, `R`, and `O` to enforce constraints.
- Generates random values `x`, `y`, `z`, `u` and computes `r = x * y * z * u`.
- Forms a witness vector.
- Uses matrix-vector multiplication to verify that the constraints hold.
- Asserts correctness and prints ``Verification successful!'' if the check passes.
