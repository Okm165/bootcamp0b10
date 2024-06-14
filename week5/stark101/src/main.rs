pub mod lde;
use lambdaworks_math::{
    field::{
        element::FieldElement, fields::fft_friendly::stark_252_prime_field::Stark252PrimeField,
    },
    polynomial::Polynomial,
};
use lde::{commit, lde};

fn main() {
    let poly: Polynomial<FieldElement<Stark252PrimeField>> = Polynomial::new(&[
        FieldElement::from(1),
        FieldElement::from(2),
        FieldElement::from(3),
    ]);

    let low_degree_extension = lde(poly).unwrap();
    // println!("{:?}", low_degree_extension.iter().map(|f| f.representative().to_hex()).collect::<Vec<String>>());

    let commitment = commit(low_degree_extension);
    // println!("{:?}", hex::encode(commitment.root.as_slice()));
}
