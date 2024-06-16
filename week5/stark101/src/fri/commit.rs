use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use lambdaworks_crypto::merkle_tree::{backends::types::Keccak256Backend, proof::Proof};
use lambdaworks_math::field::traits::IsFFTField;
use lambdaworks_math::{
    field::{element::FieldElement, traits::IsField},
    polynomial::Polynomial,
    traits::AsBytes,
};

use super::next_fri_layer;

pub struct LayerCommitment<F: IsField> {
    pub merkle_root: [u8; 32],
    pub domain_size: usize,
    pub x_inclusion_proof: Vec<Proof<[u8; 32]>>,
    pub x: Vec<FieldElement<F>>,
    pub x_neg_inclusion_proof: Vec<Proof<[u8; 32]>>,
    pub x_neg: Vec<FieldElement<F>>,
}

pub fn commit<F>(
    betas: &[FieldElement<F>],
    poly: &Polynomial<FieldElement<F>>,
    domain_generator: &FieldElement<F>,
    domain_size: &usize,
    mut offset: FieldElement<F>,
    queries: &[usize],
) -> (Vec<LayerCommitment<F>>, Polynomial<FieldElement<F>>)
where
    F: IsField + IsFFTField,
    FieldElement<F>: AsBytes + Sync + Send,
{
    let mut layers = vec![];
    let mut curr_poly = poly.clone();
    let mut curr_domain_generator = domain_generator.clone();
    let mut curr_domain_size = *domain_size;

    let evals = Polynomial::evaluate_fft::<F>(&curr_poly.scale(&offset), 1, Some(curr_domain_size))
        .unwrap();

    let tree = MerkleTree::<Keccak256Backend<F>>::build(&evals);
    layers.push(LayerCommitment {
        merkle_root: tree.root,
        domain_size: evals.len(),
        x_inclusion_proof: queries
            .iter()
            .map(|q| tree.get_proof_by_pos(q % evals.len()).unwrap())
            .collect(),
        x: queries
            .iter()
            .map(|q| evals[q % evals.len()].to_owned())
            .collect(),
        x_neg_inclusion_proof: queries
            .iter()
            .map(|q| {
                tree.get_proof_by_pos((q + evals.len() / 2) % evals.len())
                    .unwrap()
            })
            .collect(),
        x_neg: queries
            .iter()
            .map(|q| evals[(q + evals.len() / 2) % evals.len()].to_owned())
            .collect(),
    });

    for beta in betas {
        let (p, d, ds) =
            next_fri_layer(&curr_poly, beta, &curr_domain_generator, &curr_domain_size);
        curr_poly = p;
        curr_domain_generator = d;
        curr_domain_size = ds;

        offset = offset.square();

        let evals =
            Polynomial::evaluate_fft::<F>(&curr_poly.scale(&offset), 1, Some(curr_domain_size))
                .unwrap();

        let tree = MerkleTree::<Keccak256Backend<F>>::build(&evals);
        layers.push(LayerCommitment {
            merkle_root: tree.root,
            domain_size: evals.len(),
            x_inclusion_proof: queries
                .iter()
                .map(|q| tree.get_proof_by_pos(q % evals.len()).unwrap())
                .collect(),
            x: queries
                .iter()
                .map(|q| evals[q % evals.len()].to_owned())
                .collect(),
            x_neg_inclusion_proof: queries
                .iter()
                .map(|q| {
                    tree.get_proof_by_pos((q + evals.len() / 2) % evals.len())
                        .unwrap()
                })
                .collect(),
            x_neg: queries
                .iter()
                .map(|q| evals[(q + evals.len() / 2) % evals.len()].to_owned())
                .collect(),
        });
    }

    (layers, curr_poly)
}
