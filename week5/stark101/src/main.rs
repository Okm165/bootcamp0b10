pub mod lde;
pub mod trace;
use lambdaworks_math::{
    field::{
        element::FieldElement, fields::fft_friendly::stark_252_prime_field::Stark252PrimeField,
    },
    polynomial::Polynomial,
};
use lde::{commit, lde};
use trace::fibonacci_trace;

fn main() {
    let poly: Polynomial<FieldElement<Stark252PrimeField>> = Polynomial::new(&fibonacci_trace(10));

    let low_degree_extension = lde(poly, 64).unwrap();
    // println!("{:?}", low_degree_extension.iter().map(|f| f.representative().to_hex()).collect::<Vec<String>>());

    let commitment = commit(low_degree_extension);
    println!("{:?}", hex::encode(commitment.root.as_slice()));
}
