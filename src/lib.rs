#![no_std]

const MAX_PLACES: usize = 815;
const MAX_DEC_PLACES: usize = 5;

// xₙ₊₁ = ½(xₙ+S÷xₙ)
fn herons_sqrt(num: u16) -> u16 {
    if num == 1 || num == 0 {
        return num;
    }

    let mut cur = num / 2;

    loop {
        let nex = (cur + num / cur) / 2;

        if nex >= cur {
            break;
        }

        cur = nex;
    }

    cur
}

// 1 < a ≤ b < num, num = a×b = √num×√num
//  ⇒ a=b=√num ∨ a < b ⇒ a < √num ∧ b > √num
fn prime_ck(num: u16) -> bool {
    if num < 2 {
        return false;
    }

    let sqrt = herons_sqrt(num);

    for i in 2u16..=sqrt {
        if num % i == 0 {
            return false;
        }
    }

    true
}

/// converts number to decimal places
fn to_decimals(mut num: u16) -> ([u8; MAX_DEC_PLACES], usize) {
    let mut decimals = [0; MAX_DEC_PLACES];
    let mut ix = 0;
    loop {
        let d = num % 10;
        decimals[ix] = d as u8;
        num = num / 10;

        ix += 1;
        if num == 0 {
            break;
        }
    }

    (decimals, ix)
}

/// converts decimal places to number
fn from_decimals(decimals: &[u8]) -> u16 {
    #[cfg(test)]
    assert!(decimals.len() > 0);

    let mut num = 0;
    let len = decimals.len();

    for ix in 0..len {
        let place = decimals[ix];
        if place == 0 {
            continue;
        }

        num += place as u16 * 10u16.pow(ix as u32);
    }

    num
}

fn rem(dividend: &[u8], divisor: &[u8]) -> u16 {
    #[cfg(test)]
    assert!(dividend.len() > 0);
    #[cfg(test)]
    assert!(divisor.len() > 0);

    #[cfg(test)]
    assert!(from_decimals(divisor) != 0);

    // mayhap population beforehand is more effecient
    let mut rem_populated = false;

    let dividend_len = dividend.len();
    let divisor_len = divisor.len();

    let mut rem = [0; MAX_PLACES];
    let rem_ptr = rem.as_ptr();
    let mut dividend_ptr = dividend.as_ptr();

    let mut takeover;
    let mut ix;

    loop {
        takeover = 0;
        ix = 0;

        while ix < dividend_len {
            let sor_num = if ix < divisor_len {
                divisor[ix]
            } else if ix >= divisor_len && takeover == 0 && rem_populated {
                break;
            } else {
                0
            };

            let mut end_num = unsafe { dividend_ptr.offset(ix as isize).read() };

            let total = sor_num + takeover;
            takeover = if end_num < total {
                end_num += 10;
                1
            } else {
                0
            };

            rem[ix] = end_num - total;
            ix += 1;
        }

        if takeover == 1 {
            ix = 0;
            takeover = 0;
            while ix < divisor_len {
                let correction = rem[ix] + divisor[ix];
                rem[ix] = ones(correction, &mut takeover);
                ix += 1;
            }

            while ix < MAX_DEC_PLACES {
                rem[ix] = 0;
                ix += 1;
            }
            break;
        }

        if !rem_populated {
            dividend_ptr = rem_ptr;
            rem_populated = true;
        }
    }

    from_decimals(&rem[0..MAX_DEC_PLACES])
}

fn pow(base: &[u8], pow: u8) -> ([u8; MAX_PLACES], usize) {
    let mut mcand = [0; MAX_PLACES];
    if pow == 0 {
        mcand[0] = 1;
        return (mcand, 1);
    }

    let base_len = base.len();
    for ix in 0..base_len {
        mcand[ix] = base[ix]
    }

    if pow == 1 {
        return (mcand, base_len);
    }

    let mut sum = [0; MAX_PLACES];

    let mut limit = (pow - 1) as usize;
    let mut mcand_len = base_len;

    loop {
        let mut sum_len = 0;

        for base_off in 0..base_len {
            sum_len = muladd(&mcand[0..mcand_len], base[base_off], &mut sum, base_off);
        }

        mcand_len = sum_len;
        limit -= 1;
        if limit == 0 {
            mcand = sum;
            break;
        }

        mcand.clone_from(&sum);
        for ix in 0..sum_len {
            sum[ix] = 0;
        }
    }

    (mcand, mcand_len)
}

fn muladd(mcand: &[u8], mpler: u8, sum: &mut [u8], base_off: usize) -> usize {
    let mut sum_max_ix = 0;

    let mut ix = 0;
    let mcand_len = mcand.len();

    loop {
        let prod = mpler * mcand[ix];

        let max_wr_ix = sumadd(prod, sum, base_off + ix);

        if max_wr_ix > sum_max_ix {
            sum_max_ix = max_wr_ix
        };

        ix += 1;

        if ix == mcand_len {
            break;
        }
    }

    sum_max_ix + 1
}

fn sumadd(mut addend: u8, sum: &mut [u8], mut off: usize) -> usize {
    let mut takeover = 0;

    loop {
        let augend = sum[off];

        sum[off] = ones(augend + addend, &mut takeover);

        if takeover == 0 {
            break;
        } else {
            addend = 0;
            off += 1;
        }
    }

    off
}

fn ones(num: u8, takeover_ref: &mut u8) -> u8 {
    let mut takeover_val = *takeover_ref;
    let total = num + takeover_val;

    takeover_val = total / 10;
    *takeover_ref = takeover_val;

    total - takeover_val * 10
}

#[cfg(test)]
mod tests_of_units {

    mod herons_sqrt {
        use crate::herons_sqrt;

        #[test]
        fn basic_test() {
            assert_eq!(4, herons_sqrt(16));
        }

        #[test]
        fn test_17() {
            assert_eq!(4, herons_sqrt(17));
        }

        #[test]
        fn test_24() {
            assert_eq!(4, herons_sqrt(24));
        }

        #[test]
        fn test_25() {
            assert_eq!(5, herons_sqrt(25));
        }

        #[test]
        fn load_test() {
            assert_eq!(255, herons_sqrt(65025));
        }

        #[test]
        fn one_test() {
            assert_eq!(1, herons_sqrt(1));
        }

        #[test]
        fn zero_test() {
            assert_eq!(0, herons_sqrt(0));
        }
    }

    mod prime_ck {
        use crate::prime_ck;

        #[test]
        fn basic_test() {
            assert!(prime_ck(2));
        }

        #[test]
        fn even_test() {
            assert_eq!(false, prime_ck(256));
        }

        #[test]
        fn test_65521() {
            assert!(prime_ck(65521));
        }

        #[test]
        fn test_49() {
            assert_eq!(false, prime_ck(49));
        }

        #[test]
        fn one_test() {
            assert_eq!(false, prime_ck(1));
        }

        #[test]
        fn zero_test() {
            assert_eq!(false, prime_ck(0));
        }
    }

    mod to_decimals {
        use crate::to_decimals;

        #[test]
        fn basic_test() {
            let decimals = to_decimals(1);
            assert_eq!([1, 0, 0, 0, 0], decimals.0);
            assert_eq!(1, decimals.1);
        }

        #[test]
        fn zero_test() {
            let decimals = to_decimals(0);
            assert_eq!([0, 0, 0, 0, 0], decimals.0);
            assert_eq!(1, decimals.1);
        }

        #[test]
        fn test_65535() {
            let decimals = to_decimals(65535);
            assert_eq!([5, 3, 5, 5, 6], decimals.0);
            assert_eq!(5, decimals.1);
        }
    }

    mod from_decimals {
        use crate::from_decimals;

        #[test]
        fn basic_test() {
            assert_eq!(1, from_decimals(&[1]));
        }

        #[test]
        fn zero_test() {
            assert_eq!(0, from_decimals(&[0]));
        }

        #[test]
        fn zero_place_test() {
            assert_eq!(101, from_decimals(&[1, 0, 1]));
        }

        #[test]
        fn test_65535() {
            assert_eq!(u16::MAX, from_decimals(&[5, 3, 5, 5, 6]));
        }
    }

    mod rem {
        use crate::{rem, to_decimals};

        #[test]
        fn basic_test() {
            let dividend = to_decimals(4);
            let divisor = to_decimals(4);

            let rem = rem(&dividend.0[..dividend.1], &divisor.0[..dividend.1]);
            assert_eq!(0, rem);
        }

        #[test]
        fn advanced_test() {
            let dividend = to_decimals(171);
            let divisor = to_decimals(22);

            let rem = rem(&dividend.0[..dividend.1], &divisor.0[..dividend.1]);
            assert_eq!(17, rem);
        }

        #[test]
        fn minuend_copy_test() {
            let dividend = to_decimals(15);
            let divisor = to_decimals(4);

            let rem = rem(&dividend.0[..dividend.1], &divisor.0[..dividend.1]);
            // if rem was no populated in first run it would be `1`
            assert_eq!(3, rem);
        }

        #[test]
        fn takeover_test() {
            let dividend = to_decimals(909);
            let divisor = to_decimals(9);

            let rem = rem(&dividend.0[..dividend.1], &divisor.0[..dividend.1]);
            assert_eq!(0, rem);
        }

        #[test]
        fn overrun_clearing_test() {
            let dividend = to_decimals(65002);
            let divisor = to_decimals(65);

            let rem = rem(&dividend.0[..dividend.1], &divisor.0[..dividend.1]);
            assert_eq!(2, rem);
        }
    }

    mod pow {
        use crate::{pow, to_decimals};

        #[test]
        fn basic_test() {
            let pow = pow(&[2], 3);

            assert_eq!(1, pow.1);
            assert_eq!(&[8], &pow.0[..pow.1]);
        }

        #[test]
        fn advanced_test() {
            let decimals = to_decimals(u16::MAX);
            let proof = [5, 2, 2, 6, 3, 8, 4, 9, 2, 4];
            let proof_len = proof.len();

            let pow = pow(&decimals.0, 2);

            assert_eq!(proof_len, pow.1);
            assert_eq!(proof, &pow.0[0..pow.1]);
        }

        #[test]
        fn advanced_test2() {
            let decimals = to_decimals(90);
            let proof = [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 6, 5, 6, 6, 6, 9, 9, 6, 1, 8,
                1, 7, 7, 6, 6, 1,
            ];
            let proof_len = proof.len();

            let pow = pow(&decimals.0[0..decimals.1], 17);

            assert_eq!(proof_len, pow.1);
            assert_eq!(proof, &pow.0[0..pow.1]);
        }

        #[test]
        fn zero_power_test() {
            let pow = pow(&[0], 0);
            assert_eq!(1, pow.1);
            assert_eq!(&[1], &pow.0[..pow.1]);
        }

        #[test]
        fn one_power_test() {
            let decimals = to_decimals(3398);
            let decimals = &decimals.0[..decimals.1];
            let pow = pow(&decimals, 1);

            assert_eq!(4, pow.1);
            assert_eq!(decimals, &pow.0[..pow.1]);
        }

        #[test]
        fn power_of_zero_test() {
            let pow = pow(&[0], 255);

            assert_eq!(1, pow.1);
            assert_eq!(&[0], &pow.0[..pow.1]);
        }

        #[test]
        fn power_of_one_test() {
            let pow = pow(&[1], 255);

            assert_eq!(1, pow.1);
            assert_eq!(&[1], &pow.0[..pow.1]);
        }
    }

    mod muladd {
        use crate::muladd;

        #[test]
        fn basic_test() {
            let mcand = [3, 2, 1, 0, 0];
            let mut sum = [0; 5];

            let len = muladd(&mcand[0..3], 3, &mut sum, 0);

            assert_eq!([9, 6, 3, 0, 0], sum);
            assert_eq!(3, len);
        }

        #[test]
        fn offset_test() {
            let mcand = [1, 1, 1, 0, 0];
            let mut sum = [1, 1, 1, 0, 0];

            let len = muladd(&mcand[0..3], 3, &mut sum, 1);
            assert_eq!([1, 4, 4, 3, 0], sum);
            assert_eq!(4, len);
        }

        #[test]
        fn len_test() {
            let mcand = [1, 1, 1, 0, 0, 0];
            let mut sum = [9, 9, 9, 0, 0, 0];

            let len = muladd(&mcand[0..3], 9, &mut sum, 1);

            assert_eq!([9, 8, 9, 0, 1, 0], sum);
            assert_eq!(5, len);
        }
    }

    mod sumadd {
        use crate::sumadd;

        #[test]
        fn basic_test() {
            const OFF: usize = 0;
            let mut sum = [5; 1];
            let max_wr_ix = sumadd(4, &mut sum, OFF);

            assert_eq!(OFF, max_wr_ix);
            assert_eq!([9], sum);
        }

        #[test]
        fn offset_test() {
            const OFF: usize = 1;
            let mut sum = [0, 5];

            let max_wr_ix = sumadd(4, &mut sum, OFF);

            assert_eq!(OFF, max_wr_ix);
            assert_eq!([0, 9], sum);
        }

        #[test]
        fn takeover_test() {
            let mut sum = [9, 4, 1];
            _ = sumadd(1, &mut sum, 0);

            assert_eq!([0, 5, 1], sum);
        }

        #[test]
        fn wite_index_test() {
            let mut sum = [9, 9, 9, 9, 0, 0];
            let off = sumadd(1, &mut sum, 1);

            assert_eq!(4, off);
            assert_eq!([9, 0, 0, 0, 1, 0], sum);
        }
    }

    mod ones {
        use crate::ones;

        #[test]
        fn basic_test() {
            let num = 9;
            let mut takeover = 0;

            assert_eq!(9, ones(num, &mut takeover));
            assert_eq!(0, takeover);
        }

        #[test]
        fn split_test() {
            let num = 9;
            let mut takeover = 3;

            assert_eq!(2, ones(num, &mut takeover));
            assert_eq!(1, takeover);
        }

        #[test]
        fn maximum_test() {
            let num = 246;
            let mut takeover = 9;

            assert_eq!(5, ones(num, &mut takeover));
            assert_eq!(25, takeover);
        }
    }
}
