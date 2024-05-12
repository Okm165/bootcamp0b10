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
    let (_, rem) = (x * y).div_rem(modulus);
    rem
}

pub fn extended_euclidean_algorithm(a: U2048, b: U2048) -> (U2048, U2048, U2048) {
    let zero = U2048::from_u64(0);
    let one = U2048::from_u64(1);

    let (mut r0, mut r1) = (a, b);
    let (mut s0, mut s1) = (one, zero);
    let (mut t0, mut t1) = (zero, one);

    let mut n: u64 = 0;

    while r1 != zero {
        let (q, _) = r0.div_rem(&r1);

        r0 = if r0 > q * r1 {
            r0 - q * r1
        } else {
            q * r1 - r0
        };

        (r0, r1) = (r1, r0);

        s0 = s0 + q * s1;
        (s0, s1) = (s1, s0);

        t0 = t0 + q * t1;
        (t0, t1) = (t1, t0);

        n += 1;
    }

    if n % 2 == 1 {
        s0 = b - s0;
    } else {
        t0 = a - t0;
    }

    (r0, s0, t0)
}
