pub mod commit;
pub mod decommit;

use lambdaworks_math::{
    field::{element::FieldElement, traits::IsField},
    polynomial::{self, Polynomial},
};

pub fn half_domain<F: IsField>(
    domain_generator: &FieldElement<F>,
    domain_size: &usize,
) -> (FieldElement<F>, usize) {
    (domain_generator.square(), domain_size / 2)
}

pub fn fold_polynomial<F>(
    poly: &Polynomial<FieldElement<F>>,
    beta: &FieldElement<F>,
) -> Polynomial<FieldElement<F>>
where
    F: IsField,
{
    let coef = poly.coefficients();
    let even_coef: Vec<FieldElement<F>> = coef.iter().step_by(2).cloned().collect();

    let odd_coef_mul_beta: Vec<FieldElement<F>> = coef
        .iter()
        .skip(1)
        .step_by(2)
        .map(|v| (v.clone()) * beta)
        .collect();

    let (even_poly, odd_poly) = polynomial::pad_with_zero_coefficients(
        &Polynomial::new(&even_coef),
        &Polynomial::new(&odd_coef_mul_beta),
    );
    even_poly + odd_poly
}

pub fn next_fri_layer<F: IsField>(
    poly: &Polynomial<FieldElement<F>>,
    beta: &FieldElement<F>,
    domain_generator: &FieldElement<F>,
    domain_size: &usize,
) -> (Polynomial<FieldElement<F>>, FieldElement<F>, usize) {
    let next_polynomial = fold_polynomial(poly, beta);
    let (next_domain_generator, next_domain_size) = half_domain(domain_generator, domain_size);
    (next_polynomial, next_domain_generator, next_domain_size)
}
