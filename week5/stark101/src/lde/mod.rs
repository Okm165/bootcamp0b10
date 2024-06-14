use lambdaworks_crypto::merkle_tree::{
    backends::field_element::FieldElementBackend, merkle::MerkleTree,
};
use lambdaworks_math::{
    fft::errors::FFTError,
    field::{
        element::FieldElement,
        traits::{IsFFTField, IsField},
    },
    polynomial::Polynomial,
    traits::AsBytes,
};
use sha3::Keccak256;

pub fn commit<F: IsField>(
    values: Vec<FieldElement<F>>,
) -> MerkleTree<FieldElementBackend<F, Keccak256, 32>>
where
    FieldElement<F>: AsBytes + Sync + Send,
{
    MerkleTree::build(&values)
}

pub fn lde<F: IsField + IsFFTField>(
    poly: Polynomial<FieldElement<F>>,
    domain_size: usize,
) -> Result<Vec<FieldElement<F>>, FFTError> {
    let scaled = poly.scale(&FieldElement::<F>::from(3));
    Polynomial::evaluate_fft::<F>(&scaled, 1, Some(domain_size))
}
