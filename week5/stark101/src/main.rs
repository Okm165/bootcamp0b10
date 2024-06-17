pub mod constraints;
pub mod fri;
pub mod trace;

use constraints::eval_composition_polynomial;
use fri::{commit::commit, decommit::layers_decommit};
use lambdaworks_crypto::merkle_tree::backends::types::Keccak256Backend;
use lambdaworks_math::{
    field::{
        element::FieldElement, fields::fft_friendly::stark_252_prime_field::Stark252PrimeField,
        traits::IsFFTField,
    },
    polynomial::Polynomial,
};
use trace::fibonacci_trace;

const TRACE_LENGTH: usize = 32;
const DOMAIN_SIZE: usize = 8192;

fn main() {
    let offset = FieldElement::<Stark252PrimeField>::from(3);
    let trace_poly_generator = Stark252PrimeField::get_primitive_root_of_unity(5).unwrap();
    let lde_poly_generator = Stark252PrimeField::get_primitive_root_of_unity(13).unwrap();

    let trace = fibonacci_trace::<Stark252PrimeField>(TRACE_LENGTH);

    let trace_poly = Polynomial::interpolate_fft::<Stark252PrimeField>(&trace).unwrap();
    println!("trace_poly degree: {}", trace_poly.degree());

    let alphas = vec![FieldElement::from(238), FieldElement::from(912)];

    let (_, composition_poly_evals) = (0..DOMAIN_SIZE).fold(
        (offset, Vec::<FieldElement<Stark252PrimeField>>::new()),
        |(eval_point, mut evals), _| {
            evals.push(eval_composition_polynomial(
                &trace_poly,
                &eval_point,
                &alphas,
                &trace_poly_generator,
            ));
            (eval_point * lde_poly_generator, evals)
        },
    );

    let interpolated_cp =
        Polynomial::interpolate_fft::<Stark252PrimeField>(&composition_poly_evals).unwrap();
    println!(
        "composition polynomial degree: {}",
        interpolated_cp.degree()
    );

    // randomly selected by verifier
    let betas: Vec<FieldElement<Stark252PrimeField>> = vec![
        FieldElement::from(100),
        FieldElement::from(881),
        FieldElement::from(331),
        FieldElement::from(912),
    ];
    println!(
        "folding betas: {:?}",
        betas
            .iter()
            .map(|b| b.representative().to_hex())
            .collect::<Vec<String>>()
    );

    // randomly selected by verifier
    let queries: Vec<usize> = vec![3892, 1828, 122];
    println!("fri queries: {:?}", queries);

    let (layers, last_layer_poly) = commit(
        &betas,
        &interpolated_cp,
        &lde_poly_generator,
        &DOMAIN_SIZE,
        offset,
        &queries,
    );

    //TODO verifier receives f(x) f(gx) f(g*g*x) calculates cp(x) and check it is present in first layer of FRI
    let gamma = lde_poly_generator.pow(queries[0]) * offset;
    let cp_gamma = eval_composition_polynomial(&trace_poly, &gamma, &alphas, &trace_poly_generator);
    assert!(
        layers[0].x_inclusion_proof[0].verify::<Keccak256Backend<Stark252PrimeField>>(
            &layers[0].merkle_root,
            queries[0],
            &cp_gamma
        )
    );

    // Verify FRI
    layers_decommit(&layers, &betas, &queries, lde_poly_generator);
    assert!(last_layer_poly.degree() <= trace_poly.degree() / 2_usize.pow(betas.len() as u32));

    println!("proof correct");
}
