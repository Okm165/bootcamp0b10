pub mod constraints;
pub mod lde;
pub mod trace;

use constraints::eval_composition_polynomial;
use lambdaworks_math::{
    field::{
        element::FieldElement, fields::fft_friendly::stark_252_prime_field::Stark252PrimeField,
        traits::IsFFTField,
    },
    polynomial::Polynomial,
};
use trace::fibonacci_trace;

const TRACE_LENGTH: usize = 32;

fn main() {
    let offset = FieldElement::<Stark252PrimeField>::from(3);
    let trace = fibonacci_trace::<Stark252PrimeField>(TRACE_LENGTH);
    
    let trace_poly =
        Polynomial::interpolate_fft::<Stark252PrimeField>(&trace).unwrap();

    let trace_poly_generator = Stark252PrimeField::get_primitive_root_of_unity(5).unwrap();
    let lde_poly_generator = Stark252PrimeField::get_primitive_root_of_unity(10).unwrap();

    let (_, composition_poly_evals) = (0..1024).fold(
        (offset, Vec::<FieldElement<Stark252PrimeField>>::new()),
        |(eval_point, mut evals), _| {
            evals.push(eval_composition_polynomial(
                &trace_poly,
                &eval_point,
                &trace_poly_generator,
            ));
            (eval_point * lde_poly_generator, evals)
        },
    );

    let interpolated_poly =
        Polynomial::interpolate_fft::<Stark252PrimeField>(&composition_poly_evals).unwrap();
    println!("{}", interpolated_poly.degree())
}
