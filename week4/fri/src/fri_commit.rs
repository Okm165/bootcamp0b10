use crate::BatchedMerkleTree;
use lambdaworks_math::{
    field::{element::FieldElement, fields::fft_friendly::babybear::Babybear31PrimeField},
    polynomial::Polynomial,
};

pub fn commit(
    betas: Vec<FieldElement<Babybear31PrimeField>>,
    polynomial: Polynomial<FieldElement<Babybear31PrimeField>>,
) -> Vec<BatchedMerkleTree<Babybear31PrimeField>> {
    todo!()
}
