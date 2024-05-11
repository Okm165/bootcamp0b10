use crate::U2048;

pub fn square_and_multiply(mut x: U2048, mut n: U2048, modulus: &U2048) -> U2048 {
    let zero = U2048::from_u64(0);
    let one = U2048::from_u64(1);
    let two = U2048::from_u64(2);

    if n == zero {
        return one;
    }

    let mut y = one;

    while n > one {
        let (_, rem) = n.div_rem(&two);

        if rem == one {
            let (_, y_rem) = (x * y).div_rem(modulus);
            y = y_rem;
            n = n - one;
        }
        let (_, x_rem) = (x * x).div_rem(modulus);
        x = x_rem;

        let (quo, _) = n.div_rem(&two);
        n = quo;
    }
    x * y
}

fn update_step(a: &mut i128, old_a: &mut i128, quotient: i128) {
    let temp = *a;
    *a = *old_a - quotient * temp;
    *old_a = temp;
}

pub fn extended_euclidean_algorithm(a: i128, b: i128) -> (i128, i128, i128) {
    let (mut old_r, mut rem) = (a, b);
    let (mut old_s, mut coeff_s) = (1, 0);
    let (mut old_t, mut coeff_t) = (0, 1);

    while rem != 0 {
        let quotient = old_r / rem;

        update_step(&mut rem, &mut old_r, quotient);
        update_step(&mut coeff_s, &mut old_s, quotient);
        update_step(&mut coeff_t, &mut old_t, quotient);
    }

    (old_r, old_s, old_t)
}
