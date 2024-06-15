use lambdaworks_crypto::merkle_tree::{
    backends::field_element::FieldElementBackend, merkle::MerkleTree,
};
use lambdaworks_math::{
    field::{element::FieldElement, traits::IsField},
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
