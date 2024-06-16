use lambdaworks_crypto::merkle_tree::backends::types::Keccak256Backend;
use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use lambdaworks_math::{
    field::{element::FieldElement, traits::IsField},
    polynomial::Polynomial,
    traits::AsBytes,
};

use super::next_fri_layer;

pub fn commit<F>(
    betas: &[FieldElement<F>],
    poly: &Polynomial<FieldElement<F>>,
    domain_generator: &FieldElement<F>,
    domain_size: &usize,
    mut offset: FieldElement<F>,
) -> Vec<(MerkleTree<Keccak256Backend<F>>, Vec<FieldElement<F>>)>
where
    F: IsField,
    FieldElement<F>: AsBytes + Sync + Send,
{
    let mut layers = vec![];
    let mut curr_poly = poly.clone();
    let mut curr_domain_generator = domain_generator.clone();
    let mut curr_domain_size = domain_size.clone();

    let (_, evals) = (0..curr_domain_size).fold(
        (offset.to_owned(), Vec::<FieldElement<F>>::new()),
        |(eval_point, mut evals), _| {
            evals.push(curr_poly.evaluate(&eval_point));
            (&eval_point * &curr_domain_generator, evals)
        },
    );
    layers.push((MerkleTree::build(&evals), evals));
    println!("{}", curr_domain_size);
    
    for beta in betas {
        offset = offset.square();
        let (p, d, ds) =
            next_fri_layer(&curr_poly, beta, &curr_domain_generator, &curr_domain_size);
        curr_poly = p;
        curr_domain_generator = d;
        curr_domain_size = ds;

        let (_, evals) = (0..curr_domain_size).fold(
            (offset.to_owned(), Vec::<FieldElement<F>>::new()),
            |(eval_point, mut evals), _| {
                evals.push(curr_poly.evaluate(&eval_point));
                (&eval_point * &curr_domain_generator, evals)
            },
        );
        layers.push((MerkleTree::build(&evals), evals));
        println!("{}", curr_domain_size);
    }

    layers
}
