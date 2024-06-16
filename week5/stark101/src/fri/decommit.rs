use lambdaworks_crypto::merkle_tree::{backends::types::Keccak256Backend, merkle::MerkleTree};
use lambdaworks_math::{
    field::{
        element::FieldElement,
        traits::{IsField, IsPrimeField},
    },
    traits::AsBytes,
};

pub fn fri_butterfly<F: IsField>(
    f_x: &FieldElement<F>,
    f_neg_x: &FieldElement<F>,
    x: &FieldElement<F>,
    beta: &FieldElement<F>,
) -> FieldElement<F> {
    let x_inv = x.inv().unwrap();
    let two_inv = &FieldElement::<F>::from(2).inv().unwrap();
    let g_x2 = (f_x + f_neg_x) * two_inv;
    let h_x2 = (f_x - f_neg_x) * two_inv * x_inv;
    g_x2 + beta * h_x2
}

pub fn layers_decommit<F>(
    layers: &[(MerkleTree<Keccak256Backend<F>>, Vec<FieldElement<F>>)],
    betas: &[FieldElement<F>],
    x_idx: usize,
    mut root_of_unity: FieldElement<F>,
    mut offset: FieldElement<F>,
) -> ()
where
    F: IsField + IsPrimeField,
    FieldElement<F>: AsBytes + Sync + Send,
{
    for (i, (merkle, evals)) in layers.into_iter().take(layers.len()-1).enumerate() {
        let x_idx = x_idx % evals.len();
        let neg_x_idx = (x_idx + evals.len() / 2) % evals.len();
        let x = root_of_unity.pow(x_idx) * offset.to_owned();
        let f_x = &evals[x_idx];
        let f_neg_x = &evals[neg_x_idx];
        let f_x2 = fri_butterfly(&f_x, &f_neg_x, &x, &betas[i]);

        assert!(f_x2 == layers[i+1].1[x_idx % layers[i+1].1.len()]);
        println!("{:?}", layers[i+1].1.iter().find(|a| **a == f_x2));

        root_of_unity = root_of_unity.square();
        offset = offset.square();
    }
}
