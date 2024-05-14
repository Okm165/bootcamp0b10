use lambdaworks_math::field::element::FieldElement;
use lambdaworks_math::field::fields::mersenne31::extension::Mersenne31Complex;
use lambdaworks_math::field::fields::mersenne31::field::Mersenne31Field;
use lambdaworks_math::field::traits::IsField;

fn main() {
    let a: <Mersenne31Complex as IsField>::BaseType = [
        FieldElement::<Mersenne31Field>::from(&10_u32),
        FieldElement::<Mersenne31Field>::from(&10_u32),
    ];

    let _b: <Mersenne31Complex as IsField>::BaseType = [
        FieldElement::<Mersenne31Field>::from(&70_u32),
        FieldElement::<Mersenne31Field>::from(&20_u32),
    ];

    // let c = Mersenne31Complex::add(&a, &b);
    let c = Mersenne31Complex::inv(&a).unwrap();

    let z = Mersenne31Complex::mul(&a, &c);

    println!("{}:{}", z[0].representative(), z[1].representative())
}
