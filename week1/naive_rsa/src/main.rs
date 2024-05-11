pub mod math;

use lambdaworks_math::unsigned_integer::element::UnsignedInteger;
use math::{extended_euclidean_algorithm, square_and_multiply};

const LIMBS: usize = 32;
pub type U2048 = UnsignedInteger<LIMBS>;

fn main() {
    let p = 1225469249_i128;
    let q = 2494616653_i128;

    let n = UnsignedInteger::from_u128((p * q) as u128);
    let phi_n = (p - 1) * (q - 1);

    let e = 65537_i128;
    let (_, s, _) = extended_euclidean_algorithm(e, phi_n);

    let s = if s < 0 { phi_n + s } else { s };

    assert!((s * e) % phi_n == 1);

    let msg = UnsignedInteger::from_hex("98081038093").unwrap();
    println!("origina;: {msg}");

    let encrypted = square_and_multiply(msg, UnsignedInteger::from_u128(e as u128), &n);
    println!("encrypted: {encrypted}");

    let (_, decrypted) =
        square_and_multiply(encrypted, UnsignedInteger::from_u128(s as u128), &n).div_rem(&n);
    println!("decrypted: {decrypted}");

    assert!(decrypted == msg)
}
