use lambdaworks_math::field::{element::FieldElement, traits::IsField};

pub fn fibonacci_trace<F: IsField>(length: usize) -> Vec<FieldElement<F>> {
    let mut trace = vec![FieldElement::one(), FieldElement::one()];

    while trace.len() < length {
        trace.push(&trace[trace.len() - 2] + &trace[trace.len() - 1]);
    }

    trace
}
