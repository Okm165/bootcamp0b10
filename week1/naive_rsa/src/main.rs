pub mod alg;

use alg::{extended_euclidean_algorithm, square_and_multiply};
use blake2::{Blake2s256, Digest};
use lambdaworks_math::{traits::ByteConversion, unsigned_integer::element::UnsignedInteger};

const LIMBS: usize = 32;
pub type U2048 = UnsignedInteger<LIMBS>;

fn main() {
    let one = U2048::from_u64(1);
    let p = U2048::from_dec_str(
        "66799244443633250852231052109898944437188837306401226063243032174510163111953631",
    )
    .unwrap();
    let q = U2048::from_dec_str(
        "85186604308917486989416657678917289862431384862416026386459825330635389014928221",
    )
    .unwrap();

    let n = p * q;
    let phi_n = (p - one) * (q - one);

    let public_key = U2048::from_u128(65537);
    let (_, private_key, _) = extended_euclidean_algorithm(public_key, phi_n);

    // assert private_key = public_key ^ (-1) mod phi_n
    let (_, rem) = (private_key * public_key).div_rem(&phi_n);
    assert!(rem == one);

    let msg = "Ala ma kota.".to_string();
    println!("Original msg: {msg}");

    let mut hasher = Blake2s256::new();
    hasher.update(msg.as_bytes());
    let mut hash = hasher.finalize().to_vec();
    hash.resize(LIMBS * 8, 0);

    let msg_hash = UnsignedInteger::from_bytes_le(&hash).unwrap();
    println!("Original msg hash: {msg_hash}");

    let encrypted = square_and_multiply(msg_hash, private_key, &n);
    println!("Signature: {encrypted}");

    // ----- SEND via unsafe channel

    let decrypted = square_and_multiply(encrypted, public_key, &n);
    println!("Decrypted msg hash: {decrypted}");

    println!("Signature correct: {}", decrypted == msg_hash)
}
