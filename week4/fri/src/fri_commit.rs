use crate::BatchedMerkleTree;
use lambdaworks_math::{
    field::{element::FieldElement, fields::fft_friendly::babybear::Babybear31PrimeField},
    polynomial::Polynomial,
};

pub fn _commit(
    _betas: Vec<FieldElement<Babybear31PrimeField>>,
    _polynomial: Polynomial<FieldElement<Babybear31PrimeField>>,
) -> Vec<BatchedMerkleTree<Babybear31PrimeField>> {
    todo!()
}
