use lambdaworks_crypto::merkle_tree::backends::types::Keccak256Backend;
use lambdaworks_math::{
    field::{
        element::FieldElement,
        traits::{IsFFTField, IsField, IsPrimeField},
    },
    traits::AsBytes,
};

use super::commit::LayerCommitment;

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
    layers: &[LayerCommitment<F>],
    betas: &[FieldElement<F>],
    queries: &[usize],
    mut root_of_unity: FieldElement<F>,
) where
    F: IsField + IsFFTField + IsPrimeField,
    FieldElement<F>: AsBytes + Sync + Send,
{
    let mut next_layer_evals: Vec<FieldElement<F>> = vec![FieldElement::zero(); queries.len()];

    let curr_layer = &layers[0];

    println!("decommiting layer size: {}", curr_layer.domain_size);
    for (n, query) in queries.iter().enumerate() {
        let index = query % curr_layer.domain_size;
        let neg_index = (query + curr_layer.domain_size / 2) % curr_layer.domain_size;
        let eval_point = root_of_unity.pow(index);
        assert!(
            curr_layer.x_inclusion_proof[n].verify::<Keccak256Backend<F>>(
                &curr_layer.merkle_root,
                index,
                &curr_layer.x[n]
            )
        );
        assert!(
            curr_layer.x_neg_inclusion_proof[n].verify::<Keccak256Backend<F>>(
                &curr_layer.merkle_root,
                neg_index,
                &curr_layer.x_neg[n]
            )
        );
        let f_x2 = fri_butterfly(
            &curr_layer.x[n],
            &curr_layer.x_neg[n],
            &eval_point,
            &betas[0],
        );
        next_layer_evals[n] = f_x2;
    }

    root_of_unity = root_of_unity.square();

    for (i, beta) in betas.iter().enumerate().skip(1) {
        let curr_layer = &layers[i];
        println!("decommiting layer size: {}", curr_layer.domain_size);
        for (n, query) in queries.iter().enumerate() {
            let index = query % curr_layer.domain_size;
            let neg_index = (query + curr_layer.domain_size / 2) % curr_layer.domain_size;
            let eval_point = root_of_unity.pow(index);
            assert_eq!(next_layer_evals[n], curr_layer.x[n]);
            assert!(
                curr_layer.x_inclusion_proof[n].verify::<Keccak256Backend<F>>(
                    &curr_layer.merkle_root,
                    index,
                    &curr_layer.x[n]
                )
            );
            assert!(
                curr_layer.x_neg_inclusion_proof[n].verify::<Keccak256Backend<F>>(
                    &curr_layer.merkle_root,
                    neg_index,
                    &curr_layer.x_neg[n]
                )
            );
            let f_x2 = fri_butterfly(&curr_layer.x[n], &curr_layer.x_neg[n], &eval_point, beta);
            next_layer_evals[n] = f_x2;
        }

        root_of_unity = root_of_unity.square();
    }
}
