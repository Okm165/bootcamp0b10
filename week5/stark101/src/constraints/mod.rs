use lambdaworks_math::{
    field::{
        element::FieldElement,
        traits::{IsFFTField, IsField},
    },
    polynomial::Polynomial,
};

use crate::TRACE_LENGTH;

pub fn eval_composition_polynomial<F>(
    trace_poly: &Polynomial<FieldElement<F>>,
    evaluation_point: &FieldElement<F>,
    alphas: &[FieldElement<F>],
    root_of_unity: &FieldElement<F>,
) -> FieldElement<F>
where
    F: IsField + IsFFTField,
{
    eval_boundary_constraints(trace_poly, evaluation_point, root_of_unity) * &alphas[0]
        + eval_transition_constraints(trace_poly, evaluation_point, root_of_unity) * &alphas[1]
}

pub fn eval_selector_polynomial<F>(
    evaluation_point: &FieldElement<F>,
    root_of_unity: &FieldElement<F>,
) -> FieldElement<F>
where
    F: IsField + IsFFTField,
{
    let one = FieldElement::<F>::one();
    let vanishing_poly = evaluation_point.pow(TRACE_LENGTH) - one;
    let zeroifier_poly = (evaluation_point - root_of_unity.pow(TRACE_LENGTH - 2))
        * (evaluation_point - root_of_unity.pow(TRACE_LENGTH - 1));

    vanishing_poly
        * zeroifier_poly
            .inv()
            .expect("Zeroifier polynomial inversion failed")
}

pub fn eval_boundary_constraints<F>(
    trace_poly: &Polynomial<FieldElement<F>>,
    evaluation_point: &FieldElement<F>,
    root_of_unity: &FieldElement<F>,
) -> FieldElement<F>
where
    F: IsField + IsFFTField,
{
    let one = &FieldElement::<F>::one();
    let eval_at_point = &trace_poly.evaluate(evaluation_point);
    let inv_at_one = (evaluation_point - one)
        .inv()
        .expect("Inversion at one failed");
    let inv_at_root = (evaluation_point - root_of_unity)
        .inv()
        .expect("Inversion at root of unity failed");

    (eval_at_point - one) * inv_at_one + (eval_at_point - one) * inv_at_root
}

pub fn eval_transition_constraints<F>(
    trace_poly: &Polynomial<FieldElement<F>>,
    evaluation_point: &FieldElement<F>,
    root_of_unity: &FieldElement<F>,
) -> FieldElement<F>
where
    F: IsField + IsFFTField,
{
    let eval_at_point = trace_poly.evaluate(evaluation_point);
    let eval_at_root_point = trace_poly.evaluate(&(root_of_unity * evaluation_point));
    let eval_at_root_squared_point =
        trace_poly.evaluate(&(root_of_unity * root_of_unity * evaluation_point));

    let selector_poly = eval_selector_polynomial(evaluation_point, root_of_unity);

    (eval_at_root_squared_point - eval_at_root_point - eval_at_point)
        * selector_poly
            .inv()
            .expect("Selector polynomial inversion failed")
}
