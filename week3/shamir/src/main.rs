use lambdaworks_math::{
    field::{
        element::FieldElement,
        fields::montgomery_backed_prime_fields::{IsModulus, MontgomeryBackendPrimeField},
    },
    polynomial::Polynomial,
    traits::ByteConversion,
    unsigned_integer::element::UnsignedInteger,
};

const LIBMS: usize = 4;

#[derive(Debug, Clone)]
struct Modulus {}

impl IsModulus<UnsignedInteger<LIBMS>> for Modulus {
    const MODULUS: UnsignedInteger<LIBMS> =
        UnsignedInteger::from_hex_unchecked("3154cffe955cf6ca26055ac843cbf4f2c2476b07d3");
}

type FE = FieldElement<MontgomeryBackendPrimeField<Modulus, LIBMS>>;

fn main() {
    let shares: usize = 12;
    let poly_degree: usize = 10;

    let secret = FE::from_bytes_be(&rand::random::<[u8; 32]>()).unwrap();
    println!("hidden secret: {}", secret.representative().to_hex());

    // define poly secret + a1 * x + a2 * x^2 ....
    let poly: Polynomial<FE> = Polynomial::new(
        &[secret.clone()]
            .into_iter()
            .chain(
                (0..poly_degree).map(|_| FE::from_bytes_be(&rand::random::<[u8; 32]>()).unwrap()),
            )
            .collect::<Vec<FE>>(),
    );
    assert_eq!(poly_degree, poly.degree());

    let shares_xs: Vec<FE> = (1..=shares).map(|f| FE::from(f as u64)).collect();
    let shares_ys: Vec<FE> = shares_xs.iter().map(|f| poly.evaluate(f)).collect();

    // interpolate based on given shares
    let interpolated_poly = Polynomial::interpolate(
        &shares_xs.as_slice()[..=poly_degree],
        &shares_ys.as_slice()[..=poly_degree],
    )
    .unwrap();

    // evaluate poly at p(0)
    let derived_secret = interpolated_poly.evaluate(&FE::zero());
    println!(
        "derived_secret: {}",
        derived_secret.representative().to_hex()
    );
    assert_eq!(secret, derived_secret)
}
