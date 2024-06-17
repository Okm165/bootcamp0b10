use lambdaworks_crypto::commitments::{
    kzg::{KateZaveruchaGoldberg, StructuredReferenceString},
    traits::IsCommitmentScheme,
};
use lambdaworks_math::{
    cyclic_group::IsGroup,
    elliptic_curve::{
        short_weierstrass::{
            curves::bls12_381::{
                curve::{BLS12381Curve, SUBGROUP_ORDER},
                default_types::{FrConfig, FrElement},
                field_extension::BLS12381PrimeField,
                pairing::BLS12381AtePairing,
                twist::BLS12381TwistCurve,
            },
            point::ShortWeierstrassProjectivePoint,
        },
        traits::IsEllipticCurve,
    },
    field::{
        element::FieldElement, fields::montgomery_backed_prime_fields::MontgomeryBackendPrimeField,
    },
    polynomial::Polynomial,
};

type G1Point = ShortWeierstrassProjectivePoint<BLS12381Curve>;
type G2Point = ShortWeierstrassProjectivePoint<BLS12381TwistCurve>;

type Kzg = KateZaveruchaGoldberg<MontgomeryBackendPrimeField<FrConfig, 4>, BLS12381AtePairing>;
pub type Fq = FieldElement<BLS12381PrimeField>;

fn challenge_polynomial() -> Polynomial<FrElement> {
    Polynomial::<FrElement>::new(&[
        FieldElement::from(69),
        FieldElement::from(78),
        FieldElement::from(32),
        FieldElement::from(65),
        FieldElement::from(82),
        FieldElement::from(71),
        FieldElement::from(69),
        FieldElement::from(78),
        FieldElement::from(84),
        FieldElement::from(73),
        FieldElement::from(78),
        FieldElement::from(65),
        FieldElement::from(32),
        FieldElement::from(78),
        FieldElement::from(65),
        FieldElement::from(67),
        FieldElement::from(73),
        FieldElement::from(32),
        FieldElement::from(84),
        FieldElement::from(73),
        FieldElement::from(69),
        FieldElement::from(82),
        FieldElement::from(65),
    ])
}

fn main() {
    let base_dir = env!("CARGO_MANIFEST_DIR");
    let srs_path = base_dir.to_owned() + "/srs.bin";
    let srs = StructuredReferenceString::<G1Point, G2Point>::from_file(&srs_path).unwrap();

    let g = BLS12381Curve::generator();
    assert!(g.operate_with_self(SUBGROUP_ORDER).is_neutral_element());

    // found s from periodicity of srs
    let s = find_s(&g, &srs).unwrap();

    // asserting correct s by s*p0 = p1
    assert_eq!(
        g.operate_with_self(s.representative()),
        srs.powers_main_group[1]
    );

    // asserting correct s by s^2*p0 = p2
    assert_eq!(
        g.operate_with_self(s.square().representative()),
        srs.powers_main_group[2]
    );

    // asserting correct s by s*o0 = o1
    assert_eq!(
        srs.powers_secondary_group[0].operate_with_self(s.representative()),
        srs.powers_secondary_group[1]
    );

    let kzg = Kzg::new(srs.clone());

    let p = challenge_polynomial();

    let p_commitment: G1Point = kzg.commit(&p);

    // q(s) = (p(s) - 3) * (s - 1)^-1
    let q_s =
        (p.evaluate(&s) - FrElement::from(3)) * (s.to_owned() - FrElement::from(1)).inv().unwrap();

    // p(s) - 3 == q(s)(s-1)
    assert_eq!(
        (p.evaluate(&s) - FrElement::from(3)),
        q_s.clone() * (s - FrElement::from(1))
    );

    // q(s) * g1
    let fake_proof = g.operate_with_self(q_s.representative());

    println!("Fake proof for submission:");
    println!("{:?}", &fake_proof.to_affine().x().to_string());
    println!("{:?}", &fake_proof.to_affine().y().to_string());

    assert!(kzg.verify(
        &FrElement::from(1),
        &FrElement::from(3),
        &p_commitment,
        &fake_proof
    ));
}

fn find_s(g: &G1Point, srs: &StructuredReferenceString<G1Point, G2Point>) -> Option<FrElement> {
    let s_strings = vec![
        "0x45af6345ec055e4d14a1e27164d8fdbd2d967f4be2f951558140d032f0a9ee53",
        "0x50e0903a157988bab4bcd40e22f55448bf6e88fb4c38fb8a360c60997369df4e",
        "0x53c78adc7bff16bae3ee1645113940cf46c3ebf43c92a949a4593e1acca2cb6c",
        "0x20b1ce9140267af9dd1c0af834cec32c17beb312f20b6f7653ea61d87742bcce",
        "0x54fa64fb4536c4fcf6ad66524f0376d9e412abf7f3a89e7acf065a270f3c324f",
        "0x461237e58fcced486fa69d8e4e48506e3317ae6451bb89de69679532ae1234c",
        "0x36b79931cfdd8947f799cf20f675fde6a7493f160ce4cb729b4cb21179cfb0e",
        "0x345766f603fa66e78c0625cd70d77ce2b38b21c28713b7007228fd3397743f7a",
        "0x5391ad6a79b61c1a71d544f7800a7e4ae4dc0ae311f00af48469ef4d246b6883",
        "0x2c7e0457c83a7d9c5aea51f540eb0c04963dc46688b5e11768cc0c58459f155b",
        "0x5a50cc64ae610371dcd9ce528178852eaf9f1f01e2bbf0ac476e05bf67d4973c",
        "0x1edc919ec91f38ac5ccd4631f16edba4967a6b6cfb0faca4807b811a823f728d",
        "0x58c400aba73798bfaf59d0fc7261da72911f590ef73ba2bdc0f1357a508e5e7b",
        "0x56f35bb8ed54ae00468b04010fa5c79f62a6d195014b641082e68bc0bc50a88f",
        "0x28c6d5fd4e2f04c5e7caaba64af676214ee20d3cfc83311c0727b36db1974ef4",
        "0x8d51ccce760304d0ec030002760300000001000000000000",
        "0x28eb300e9079af0b916f129332ba2dfc0bf20a6f5e1709899ddf46bac40ac8e4",
        "0x65f6c5837cb5fca206050b5832d1099726bc7f62d13a6e1c3ec50c9031a36ca3",
        "0x37d3508a14adf95959d7d47f20aa9f0259e74ba2b75ca477f44e14739932aa33",
        "0x4f2c596e753e4fcc6e92a9c460afca4a1ef4e672ebc1e1bb95df4b360411fe73",
        "0xe4840ac57f86f5e293b1d67bc8de5d9a12a70a615d0b8e4d2fc5e69ac5db47f",
        "0x47cb16caf96816fa3a95d2d4016e2bd45593d6ff6dab086ee5bcecc4e7773cb",
        "0xafced2ec80a4115f20c57f6d7dc9533050664a566a603f98c15c05b1901cef2",
        "0x1333b22e5ce11044babc5affca86bf658e74903694b04fd86037fe81ae99502e",
        "0x44ed0520cdfb5d9d6c54cb86cdf73e91232e4312e6011bf5d941e9338fb466f7",
        "0x5303da18a9d30564a8f0cfd2438f018c01e943612401899720d4ed194fccfeb9",
        "0x6e5703824bef73c976407b9926e20836d21ae51df978cc3878f4ee1de45ab2f2",
        "0x38c7f2dd7e0c63fccabf643eda8951f257bc96af334c36bca1abb31fb37786b9",
        "0x6358785206b5761a878d670fcb570ab3b802e4e461e72e18ddc3b03ea91bc267",
        "0x1579b9c6e6797777851425ea12dcacdae7452d43f6d5756f51cb57e0e3035d15",
        "0xfe09ddec7fa8e98e4b5243a8bda7ca37836750f231bcd8672d73ebbe97445d5",
        "0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000000",
        "0x2e3e440d3d981efb1e97f596a4c8da48262724b71d050aa97ebf2fcc0f5611ae",
        "0x230d17191423f48d7e7d03f9e6ac83bc944f1b07b3c56074c9f39f658c9620b3",
        "0x20261c76ad9e668d4f4bc1c2f86897360cf9b80ec36bb2b55ba6c1e4335d3495",
        "0x533bd8c1e977024e561dcd0fd4d314d93bfef0f00df2ec88ac159e2688bd4333",
        "0x1ef34257e466b84b3c8c71b5ba9e612b6faaf80b0c55bd8430f9a5d7f0c3cdb2",
        "0x6f8c83d4d0a0ae73ac3f6e2f24bd52fe708c291cbae2a361196986abd51edcb5",
        "0x70822dc00c9fa4b3b3c03b15fa3a7826e94910119f300f47d64b34dde86304f3",
        "0x3f96405d25a31660a733b23a98ca5b22a032824078eaa4fe8dd702cb688bc087",
        "0x205bf9e8afe7612dc1649310899759ba6ee1991fee0e510a7b9610b1db94977e",
        "0x476fa2fb6162ffabd84f8612c8b6cc00bd7fdf9c77487ae79733f3a6ba60eaa6",
        "0x199cdaee7b3c79d6566009b5882952d6a41e85011d426b52b891fa3f982b68c5",
        "0x551115b4607e449bd66c91d61832fc60bd43389604eeaf5a7f847ee47dc08d74",
        "0x1b29a6a78265e48883e0070b973ffd92c29e4af408c2b9413f0eca84af71a186",
        "0x1cfa4b9a3c48cf47ecaed406f9fc1065f116d26dfeb2f7ee7d19743e43af5772",
        "0x4b26d155db6e78824b6f2c61beab61e404db96c6037b2ae2f8d84c914e68b10d",
        "0x73eda753299d7d47a5e80b39939ed33467baa40089fb5bfefffeffff00000001",
        "0x4b0277449923ce3ca1cac574d6e7aa0947cb9993a1e752756220b9443bf5371d",
        "0xdf6e1cface780a62d34ccafd6d0ce6e2d0124a02ec3ede2c13af36ece5c935e",
        "0x3c1a56c914ef83eed9620388e8f73902f9d6586048a1b7870bb1eb8b66cd55ce",
        "0x24c14de4b45f2d7bc4a72e43a8f20dbb34c8bd90143c7a436a20b4c8fbee018e",
        "0x65a566a6d1a50dea09febaa04d13f22bb293335cea2da31a2d03a19553a24b82",
        "0x6f70f5e67a06fbd88f907adac98af5480e6466930923ab7811a43132b1888c36",
        "0x68f0ba2461933c32412d801131c542d24eb73f5d9958580573ea3fa3e6fe310f",
        "0x60b9f524ccbc6d03787d7d083f1b189fc54913cc6b4e0c269fc8017d5166afd3",
        "0x2f00a2325ba21faac6e50c813baa9974308f60f019fd400926be16cb704b990a",
        "0x20e9cd3a7fca77e38a490835c612d67951d460a1dbfcd267df2b12e5b0330148",
        "0x596a3d0ddae097ebcf95c6ee2bfcfce81a2bee506858fc6870b11e11ba54d0f",
        "0x3b25b475ab91194b687a73c92f188612fc010d53ccb225425e544cdf4c887948",
        "0x10952f0122e8072dabac70f83e4acd519bbabf1e9e172de6223c4fc056e43d9a",
        "0x5e73ed8c432405d0ae25b21df6c52b2a6c7876bf0928e68fae34a81e1cfca2ec",
        "0x640d097461a2eeaf4e84b3cd7dc75b61db872ef3dce28e788d28c143168bba2c",
    ];

    s_strings
        .into_iter()
        .map(|string| FrElement::from_hex(string).unwrap())
        .find(|element| {
            g.operate_with_self(element.representative()) == srs.powers_main_group[1].to_affine()
        })
}
