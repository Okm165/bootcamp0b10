mod fold_polynomial;
mod fri_commit;
mod fri_decommit;

use lambdaworks_crypto::merkle_tree::{backends::types::BatchKeccak256Backend, merkle::MerkleTree};
use lambdaworks_math::field::{
    element::FieldElement, fields::fft_friendly::babybear::Babybear31PrimeField,
};

pub type BatchedMerkleTreeBackend<F> = BatchKeccak256Backend<F>;
pub type BatchedMerkleTree<F> = MerkleTree<BatchedMerkleTreeBackend<F>>;

fn main() {
    let a = FieldElement::<Babybear31PrimeField>::from(10);

    println!("{}", a.representative().to_hex())
}
